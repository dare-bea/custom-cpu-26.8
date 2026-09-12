//! The core logic for the system.
//!
//! This module contains the implementation of the `System` struct, which represents the state of the `CPU3v2` emulator. It includes methods for managing registers, memory, and system operations.

use crate::{
    opcode::InvalidOpcodeError,
    register::{ByteRegister, WordRegister},
};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

mod step;

pub use step::SystemStep;

/// An error that occurs during system operations.
#[derive(Debug)]
#[non_exhaustive]
pub enum SystemError {
    /// The ROM provided to the system is too large.
    RomTooLarge,
    /// Attempted to write to ROM, which is not allowed.
    WriteToRom,
    /// Attempted to read from ROM out of bounds.
    ReadOutOfRomBounds,
    /// The system is halted and cannot execute further instructions.
    Halted,
    /// I/O error encountered.
    IOError(std::io::Error),
    /// Invalid opcode encountered.
    InvalidOpcodeError(InvalidOpcodeError),
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::RomTooLarge => "rom too large",
            Self::WriteToRom => "attempt to write to rom",
            Self::ReadOutOfRomBounds => "read out of rom bounds",
            Self::IOError(e) => return e.fmt(f),
            Self::InvalidOpcodeError(e) => return e.fmt(f),
            Self::Halted => "system was halted",
        })
    }
}

impl From<std::io::Error> for SystemError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<InvalidOpcodeError> for SystemError {
    fn from(value: InvalidOpcodeError) -> Self {
        Self::InvalidOpcodeError(value)
    }
}

impl Error for SystemError {}

/// The size of the RAM in bytes.
pub const RAM_SIZE: usize = 0x8000;
/// The size of a ROM page in bytes.
pub const ROM_PAGE_SIZE: usize = 0x8000;
/// The starting address of the stack pointer (SP).
pub const SP_START: u16 = 0x7F80;
/// The starting address of the program counter (PC).
pub const PC_START: u16 = 0x8000;

/// The logic for the GPU.
pub mod gpu;

/// The state of the emulator, including registers, memory, and system flags.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct System {
    /// The value of the %H register.
    reg_h: u8,
    /// The value of the %A register.
    reg_a: u8,
    /// The value of the %B register.
    reg_b: u8,
    /// The value of the %C register.
    reg_c: u8,
    /// The value of the %X register.
    reg_x: u8,
    /// The value of the %L register.
    reg_l: u8,
    /// The value of the %M register.
    reg_m: u8,
    /// The value of the %N register.
    reg_n: u8,
    /// The value of the %R4 register.
    reg_r4: u16,
    /// The value of the %SP register.
    reg_sp: u16,
    /// The value of the %FL register.
    reg_fl: u16,
    /// The value of the %PC register.
    reg_pc: u16,
    /// The RAM of the system, represented as a boxed array of bytes.
    ram: Box<[u8; RAM_SIZE]>,
    /// The ROM of the system, represented as a boxed slice of bytes.
    rom: Box<[u8]>,
    /// The number of cycles executed by the system.
    cycles: Cell<u32>,
    /// The number of frames outputted by the system.
    frames: Cell<u32>,
    /// The Video RAM pointer.
    vram_pointer: u16,
    /// The Video RAM, represented as a boxed array of bytes.
    vram: Box<[u8; gpu::VRAM_SIZE]>,
    /// The text input buffer for SDL text input.
    text_input_buffer: RefCell<VecDeque<u8>>,
    /// The active interrupt.
    active_interrupt: Option<u16>,
    /// The next cycle where a vblank interrupt will occur.
    next_vblank: u32,
}

impl Default for System {
    fn default() -> Self {
        Self {
            reg_h: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_x: 0,
            reg_l: 0,
            reg_m: 0,
            reg_n: 0,
            reg_r4: 0,
            reg_sp: SP_START,
            reg_fl: 0,
            reg_pc: PC_START,
            ram: unsafe { Box::<[u8; RAM_SIZE]>::new_zeroed().assume_init() },
            vram_pointer: 0,
            vram: unsafe { Box::<[u8; _]>::new_zeroed().assume_init() },
            rom: Box::<[u8]>::default(),
            cycles: Cell::new(0),
            frames: Cell::new(0),
            text_input_buffer: RefCell::new(VecDeque::new()),
            next_vblank: gpu::VBLANK_INTERVAL,
            active_interrupt: None,
        }
    }
}

impl System {
    /// Creates a new `System` instance with the provided ROM. Returns an error if the ROM is too large.
    ///
    /// # Errors
    ///
    /// Returns `SystemError::RomTooLarge` if the provided ROM exceeds the maximum allowed size of 128 KiB.
    #[inline]
    pub fn new(rom: Box<[u8]>) -> Result<Self, SystemError> {
        if rom.len() > (ROM_PAGE_SIZE << 8) {
            return Err(SystemError::RomTooLarge);
        }
        Ok(Self {
            rom,
            ..Default::default()
        })
    }

    /// Takes ownership of the ROM from the system, replacing it with an empty ROM. Returns the previous ROM.
    #[inline]
    pub fn take_rom(&mut self) -> Box<[u8]> {
        std::mem::take(&mut self.rom)
    }

    /// Replaces the current ROM with a new one, returning the previous ROM.
    ///
    /// # Errors
    ///
    /// Returns `SystemError::RomTooLarge` if the new ROM exceeds the maximum allowed size of 128 KiB.
    /// The ROM will not be replaced if an error occurs.
    #[inline]
    pub fn replace_rom(&mut self, rom: Box<[u8]>) -> Result<Box<[u8]>, SystemError> {
        if rom.len() > (ROM_PAGE_SIZE << 8) {
            return Err(SystemError::RomTooLarge);
        }
        Ok(std::mem::replace(&mut self.rom, rom))
    }

    /// Returns a reference to the RAM of the system.
    #[inline]
    #[must_use]
    pub fn ram(&self) -> &[u8; RAM_SIZE] {
        &self.ram
    }

    /// Returns a mutable reference to the RAM of the system.
    #[inline]
    #[must_use]
    pub fn ram_mut(&mut self) -> &mut [u8; RAM_SIZE] {
        &mut self.ram
    }

    /// Returns a reference to the ROM of the system.
    #[inline]
    #[must_use]
    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    /// Returns a mutable reference to the ROM of the system.
    #[inline]
    #[must_use]
    pub fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    /// Returns whether the system is currently halted.
    #[inline]
    #[must_use]
    pub fn is_halted(&self) -> bool {
        self.reg_fl & (1 << 15) != 0
    }

    /// Halts the system.
    #[inline]
    pub fn halt(&mut self) {
        self.reg_fl |= 1 << 15;
    }

    /// Interrupts the system. Fails if an interrupt is active.
    /// 
    /// # Errors
    /// 
    /// A [`SystemError`] is returned if a memory write fails.
    pub fn interrupt(&mut self, interrupt_vector: u16) -> Result<(), SystemError> {
        let vector = self.get_memw(interrupt_vector)?;
        if self.active_interrupt.is_none() && vector != 0 {
            self.active_interrupt = Some(interrupt_vector);
            self.reg_sp = self.reg_sp.wrapping_sub(2);
            self.set_memw(self.reg_sp, self.reg_pc)?;
            self.reg_pc = vector;
            self.cycles.update(|cycles| cycles + 1);
        }
        Ok(())
    }

    /// Returns the number of cycles the system has performed.
    #[inline]
    pub fn cycles(&self) -> u32 {
        self.cycles.get()
    }

    /// Returns the number of frames the system has outputted.
    #[inline]
    pub fn frames(&self) -> u32 {
        self.frames.get()
    }

    /// Returns a mutable reference to the number of cycles the system has performed.
    #[inline]
    pub fn cycles_mut(&mut self) -> &mut u32 {
        self.cycles.get_mut()
    }

    /// Returns a mutable reference to the number of frames the system has outputted.
    #[inline]
    pub fn frames_mut(&mut self) -> &mut u32 {
        self.frames.get_mut()
    }

    /// Returns the current value of the Video RAM pointer.
    pub fn vram_pointer(&self) -> u16 {
        self.vram_pointer
    }
    /// Returns a reference to the Video RAM.
    pub fn vram(&self) -> &[u8; gpu::VRAM_SIZE] {
        &self.vram
    }

    /// Returns a mutable reference to the Video RAM pointer.
    pub fn vram_pointer_mut(&mut self) -> &mut u16 {
        &mut self.vram_pointer
    }
    /// Returns a mutable reference to the Video RAM.
    pub fn vram_mut(&mut self) -> &mut [u8; gpu::VRAM_SIZE] {
        &mut self.vram
    }

    /// Input text into the system's text input buffer, which can be used for certain MMIO operations.
    pub fn input_text(&mut self, text: &str) {
        for byte in text.bytes() {
            self.text_input_buffer.get_mut().push_back(byte);
        }
    }
}

impl System {
    /// Returns the value of the specified 8-bit register.
    #[inline]
    #[must_use]
    pub fn get_regb(&self, reg: ByteRegister) -> u8 {
        match reg {
            ByteRegister::H => self.reg_h,
            ByteRegister::A => self.reg_a,
            ByteRegister::B => self.reg_b,
            ByteRegister::C => self.reg_c,
            ByteRegister::X => self.reg_x,
            ByteRegister::L => self.reg_l,
            ByteRegister::M => self.reg_m,
            ByteRegister::N => self.reg_n,
        }
    }
    /// Sets the value of the specified 8-bit register.
    #[inline]
    pub fn set_regb(&mut self, reg: ByteRegister, value: u8) {
        match reg {
            ByteRegister::H => self.reg_h = value,
            ByteRegister::A => self.reg_a = value,
            ByteRegister::B => self.reg_b = value,
            ByteRegister::C => self.reg_c = value,
            ByteRegister::X => self.reg_x = value,
            ByteRegister::L => self.reg_l = value,
            ByteRegister::M => self.reg_m = value,
            ByteRegister::N => self.reg_n = value,
        }
    }
    /// Returns the value of the specified 16-bit register.
    #[inline]
    #[must_use]
    pub fn get_regw(&self, reg: WordRegister) -> u16 {
        match reg {
            WordRegister::HA => u16::from_be_bytes([self.reg_h, self.reg_a]),
            WordRegister::BC => u16::from_be_bytes([self.reg_b, self.reg_c]),
            WordRegister::XL => u16::from_be_bytes([self.reg_x, self.reg_l]),
            WordRegister::MN => u16::from_be_bytes([self.reg_m, self.reg_n]),
            WordRegister::R4 => self.reg_r4,
            WordRegister::SP => self.reg_sp,
            WordRegister::FL => self.reg_fl,
            WordRegister::PC => self.reg_pc,
        }
    }
    /// Sets the value of the specified 16-bit register.
    #[inline]
    pub fn set_regw(&mut self, reg: WordRegister, value: u16) {
        match reg {
            WordRegister::HA => [self.reg_h, self.reg_a] = value.to_be_bytes(),
            WordRegister::BC => [self.reg_b, self.reg_c] = value.to_be_bytes(),
            WordRegister::XL => [self.reg_x, self.reg_l] = value.to_be_bytes(),
            WordRegister::MN => [self.reg_m, self.reg_n] = value.to_be_bytes(),
            WordRegister::R4 => self.reg_r4 = value,
            WordRegister::SP => self.reg_sp = value,
            WordRegister::FL => self.reg_fl = value,
            WordRegister::PC => self.reg_pc = value,
        }
    }
}

#[derive(Debug)]
enum MMIOError {
    NotMMIOAddress,
    SystemError(SystemError),
}

impl<T: Into<SystemError>> From<T> for MMIOError {
    fn from(value: T) -> Self {
        Self::SystemError(value.into())
    }
}

impl System {
    #[inline]
    fn get_direct_mem(&self, addr: u16) -> Result<u8, SystemError> {
        match usize::from(addr) {
            index @ ..RAM_SIZE => Ok(self.ram[index]),
            index @ RAM_SIZE.. => self
                .rom
                .get(index - RAM_SIZE + usize::from(self.get_direct_mem(0x7FFE)?) * ROM_PAGE_SIZE)
                .copied()
                .ok_or(SystemError::ReadOutOfRomBounds),
        }
    }
    #[inline]
    fn set_direct_mem(&mut self, addr: u16, value: u8) -> Result<(), SystemError> {
        match usize::from(addr) {
            index @ ..RAM_SIZE => self.ram[index] = value,
            RAM_SIZE.. => return Err(SystemError::WriteToRom),
        }
        Ok(())
    }

    #[inline]
    fn get_mmio(&self, addr: u16) -> Result<u16, MMIOError> {
        match usize::from(addr) {
            0x7F80 => {
                let mut value = 0;
                match std::io::stdin().read(std::array::from_mut(&mut value))? {
                    0 => Ok(0xFFFF),
                    _ => Ok(value.into()),
                }
            }
            0x7F81 => {
                if let Some(byte) = self.text_input_buffer.borrow_mut().pop_front() {
                    Ok(u16::from(byte))
                } else {
                    Ok(0xFFFF)
                }
            }
            0x7F90 => Ok(self.vram_pointer()),
            0x7F91 => Ok(self.vram()[self.vram_pointer() as usize].into()),
            0x7F92 => Ok(u16::from_be_bytes([
                self.vram()[self.vram_pointer() as usize].into(),
                self.vram()[self.vram_pointer().wrapping_add(1) as usize].into(),
            ])),
            _ => Err(MMIOError::NotMMIOAddress),
        }
    }
    #[inline]
    fn set_mmio(&mut self, addr: u16, value: u16) -> Result<(), MMIOError> {
        match usize::from(addr) {
            0x7F80 => std::io::stdout().write_all(&[value as u8])?,
            0x7F90 => *self.vram_pointer_mut() = value,
            0x7F91 => {
                let index = self.vram_pointer() as usize;
                self.vram_mut()[index] = value as u8;
                *self.vram_pointer_mut() = self.vram_pointer().wrapping_add(1);
            }
            0x7F92 => {
                let index = self.vram_pointer() as usize;
                [self.vram_mut()[index], self.vram_mut()[index.wrapping_add(1)]] = value.to_be_bytes();
                *self.vram_pointer_mut() = self.vram_pointer().wrapping_add(2);
            }
            _ => return Err(MMIOError::NotMMIOAddress),
        }
        Ok(())
    }

    /// Returns the value of the memory at the specified address.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::ReadOutOfRomBounds`] if the address is out of bounds for the ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error reading from stdin when accessing `0x7F80`.
    #[inline]
    pub fn get_memb(&self, addr: u16) -> Result<u8, SystemError> {
        match self.get_mmio(addr) {
            Ok(x) => Ok(x as u8),
            Err(MMIOError::SystemError(x)) => Err(x),
            Err(MMIOError::NotMMIOAddress) => self.get_direct_mem(addr),
        }
    }
    /// Sets the value of the memory at the specified address.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::WriteToRom`] if the address is in ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error writing to stdout when accessing `0x7F80`.
    #[inline]
    pub fn set_memb(&mut self, addr: u16, value: u8) -> Result<(), SystemError> {
        match self.set_mmio(addr, u16::from(value)) {
            Ok(()) => Ok(()),
            Err(MMIOError::SystemError(x)) => Err(x),
            Err(MMIOError::NotMMIOAddress) => {
                self.set_direct_mem(addr, value)
            }
        }
    }
    /// Returns the value of the memory at the specified address as a 16-bit word.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::ReadOutOfRomBounds`] if the address is out of bounds for the ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error reading from stdin when accessing `0x7F80`.
    #[inline]
    pub fn get_memw(&self, addr: u16) -> Result<u16, SystemError> {
        match self.get_mmio(addr) {
            Ok(x) => Ok(x),
            Err(MMIOError::SystemError(x)) => Err(x),
            Err(MMIOError::NotMMIOAddress) => Ok(u16::from_be_bytes([
                self.get_direct_mem(addr)?,
                self.get_direct_mem(addr.wrapping_add(1))?,
            ])),
        }
    }
    /// Sets the value of the memory at the specified address as a 16-bit word.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::WriteToRom`] if the address is in ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error writing to stdout when accessing `0x7F80`.
    #[inline]
    pub fn set_memw(&mut self, addr: u16, value: u16) -> Result<(), SystemError> {
        match self.set_mmio(addr, value) {
            Ok(()) => Ok(()),
            Err(MMIOError::SystemError(x)) => Err(x),
            Err(MMIOError::NotMMIOAddress) => {
                let [hi, lo] = value.to_be_bytes();
                self.set_direct_mem(addr, hi)?;
                self.set_direct_mem(addr.wrapping_add(1), lo)?;
                Ok(())
            }
        }
    }
}
