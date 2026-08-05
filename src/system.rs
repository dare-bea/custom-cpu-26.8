use std::{error::Error, fmt::Display, io::{Read, Write}};
use crate::register::{ByteRegister, WordRegister};

mod step;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemError {
    RomTooLarge,
    WriteToRom,
    ReadOutOfRomBounds,
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::RomTooLarge => "rom too large",
            Self::WriteToRom => "attempt to write to rom",
            Self::ReadOutOfRomBounds => "read out of rom bounds"
        })
    }
}

impl Error for SystemError { }

pub const RAM_SIZE: usize = 0x8000;
pub const ROM_PAGE_SIZE: usize = 0x8000;
pub const SP_START: u16 = 0x7F80;
pub const PC_START: u16 = 0x8000;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct System {
    reg_h: u8,
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_x: u8,
    reg_l: u8,
    reg_m: u8,
    reg_n: u8,
    reg_r4: u16,
    reg_sp: u16,
    reg_fl: u16,
    reg_pc: u16,
    ram: [u8; RAM_SIZE],
    rom: Box<[u8]>,
    cycles: u32,
}

impl Default for System {
    fn default() -> Self {
        Self {
            reg_h: 0, reg_a: 0, reg_b: 0, reg_c: 0,
            reg_x: 0, reg_l: 0, reg_m: 0, reg_n: 0,
            reg_r4: 0, reg_sp: SP_START, reg_fl: 0, reg_pc: PC_START,
            ram: [0; _], rom: Box::<[u8]>::default(),
            cycles: 0,
        }
    }
}

impl System {
    #[inline]
    pub fn new(rom: Box<[u8]>) -> Result<Self, SystemError> {
        if rom.len() > (ROM_PAGE_SIZE << 8) {
            return Err(SystemError::RomTooLarge)
        }
        Ok(Self {rom, ..Default::default()})
    }

    #[inline]
    pub fn take_rom(&mut self) -> Box<[u8]> {
        std::mem::take(&mut self.rom)
    }

    #[inline]
    pub fn replace_rom(&mut self, rom: Box<[u8]>) -> Result<Box<[u8]>, SystemError> {
        if rom.len() > (ROM_PAGE_SIZE << 8) {
            return Err(SystemError::RomTooLarge)
        }
        Ok(std::mem::replace(&mut self.rom, rom))
    }

    #[inline]
    pub fn ram(&self) -> &[u8] {
        &self.ram
    }

    #[inline]
    pub fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram
    }

    #[inline]
    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    #[inline]
    pub fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.reg_fl & (1 << 15) == 0
    }
}

impl System {
    #[inline]
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
        };
    }
    #[inline]
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
        };
    }
}

impl System {
    #[inline]
    pub fn get_memb(&self, addr: u16) -> Result<u8, Box<dyn Error>> {
        match usize::from(addr) {
            0x7F80 => {
                let mut value = 0xFFu8;
                let _ = std::io::stdin().read(std::array::from_mut(&mut value))?;
                Ok(value)
            },
            index @ ..RAM_SIZE => Ok(self.ram[index]),
            index @ RAM_SIZE.. => self.rom.get(
                index - RAM_SIZE - usize::from(self.get_memb(0x7FFE)?) * ROM_PAGE_SIZE
            ).copied().ok_or(SystemError::ReadOutOfRomBounds.into())
        }
    }
    #[inline]
    pub fn set_memb(&mut self, addr: u16, value: u8) -> Result<(), Box<dyn Error>> {
        match usize::from(addr) {
            0x7F80 => {std::io::stdout().write_all(&[value])?;},
            index @ ..RAM_SIZE => self.ram[index] = value,
            RAM_SIZE.. => return Err(SystemError::WriteToRom.into()),
        };
        Ok(())
    }
    #[inline]
    pub fn get_memw(&self, addr: u16) -> Result<u16, Box<dyn Error>> {
        Ok(u16::from_be_bytes([self.get_memb(addr)?, self.get_memb(addr.wrapping_add(1))?]))
    }
    #[inline]
    pub fn set_memw(&mut self, addr: u16, value: u16) -> Result<(), Box<dyn Error>> {
        let [hi, lo] = value.to_be_bytes();
        self.set_memb(addr, hi)?;
        self.set_memb(addr.wrapping_add(1), lo)?;
        Ok(())
    }
}