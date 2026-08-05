use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParseRegisterError { _private: () }

impl Display for ParseRegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("tried to parse invalid register")
    }
}

impl Error for ParseRegisterError { }

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ByteRegister {
    H, A, B, C, X, L, M, N
}

impl Display for ByteRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::H => "H",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::X => "X",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
        })
    }
}

impl FromStr for ByteRegister {
    type Err = ParseRegisterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Self::H),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "X" => Ok(Self::X),
            "L" => Ok(Self::L),
            "M" => Ok(Self::M),
            "N" => Ok(Self::N),
            _ => Err(ParseRegisterError { _private: () })
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum WordRegister {
    HA, BC, XL, MN, R4, SP, FL, PC
}

impl Display for WordRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::HA => "HA",
            Self::BC => "BC",
            Self::XL => "XL",
            Self::MN => "MN",
            Self::R4 => "R4",
            Self::SP => "SP",
            Self::FL => "FL",
            Self::PC => "PC",
        })
    }
}

impl FromStr for WordRegister {
    type Err = ParseRegisterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HA" => Ok(Self::HA),
            "BC" => Ok(Self::BC),
            "XL" => Ok(Self::XL),
            "MN" => Ok(Self::MN),
            "R4" => Ok(Self::R4),
            "SP" => Ok(Self::SP),
            "FL" => Ok(Self::FL),
            "PC" => Ok(Self::PC),
            _ => Err(ParseRegisterError { _private: () })
        }
    }
}