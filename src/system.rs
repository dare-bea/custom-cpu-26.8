//! The core logic for the system.
//!
//! This module contains the implementation of the `System` struct, which represents the state of the `CPU3v2` emulator. It includes methods for managing registers, memory, and system operations.

use crate::register::{ByteRegister, WordRegister};
use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

mod step;

/// An error that occurs during system operations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::RomTooLarge => "rom too large",
            Self::WriteToRom => "attempt to write to rom",
            Self::ReadOutOfRomBounds => "read out of rom bounds",
            Self::Halted => "system was halted",
        })
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
    cycles: u32,
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
            rom: Box::<[u8]>::default(),
            cycles: 0,
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

impl System {
    /// Returns the value of the memory at the specified address.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::ReadOutOfRomBounds`] if the address is out of bounds for the ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error reading from stdin when accessing `0x7F80`.
    #[inline]
    pub fn get_memb(&self, addr: u16) -> Result<u8, Box<dyn Error>> {
        match usize::from(addr) {
            0x7F80 => {
                let mut value = 0xFFu8;
                let _ = std::io::stdin().read(std::array::from_mut(&mut value))?;
                Ok(value)
            }
            index @ ..RAM_SIZE => Ok(self.ram[index]),
            index @ RAM_SIZE.. => self
                .rom
                .get(index - RAM_SIZE + usize::from(self.get_memb(0x7FFE)?) * ROM_PAGE_SIZE)
                .copied()
                .ok_or(SystemError::ReadOutOfRomBounds.into()),
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
    pub fn set_memb(&mut self, addr: u16, value: u8) -> Result<(), Box<dyn Error>> {
        match usize::from(addr) {
            0x7F80 => {
                std::io::stdout().write_all(&[value])?;
            }
            index @ ..RAM_SIZE => self.ram[index] = value,
            RAM_SIZE.. => return Err(SystemError::WriteToRom.into()),
        }
        Ok(())
    }
    /// Returns the value of the memory at the specified address as a 16-bit word.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::ReadOutOfRomBounds`] if the address is out of bounds for the ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error reading from stdin when accessing `0x7F80`.
    #[inline]
    pub fn get_memw(&self, addr: u16) -> Result<u16, Box<dyn Error>> {
        Ok(u16::from_be_bytes([
            self.get_memb(addr)?,
            self.get_memb(addr.wrapping_add(1))?,
        ]))
    }
    /// Sets the value of the memory at the specified address as a 16-bit word.
    ///
    /// # Errors
    ///
    /// Returns [`SystemError::WriteToRom`] if the address is in ROM.
    ///
    /// Returns [`std::io::Error`] if there is an error writing to stdout when accessing `0x7F80`.
    #[inline]
    pub fn set_memw(&mut self, addr: u16, value: u16) -> Result<(), Box<dyn Error>> {
        let [hi, lo] = value.to_be_bytes();
        self.set_memb(addr, hi)?;
        self.set_memb(addr.wrapping_add(1), lo)?;
        Ok(())
    }
}
