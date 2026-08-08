use std::error::Error;
use std::fmt::Display;

use crate::register::{ByteRegister, WordRegister};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InvalidOpcodeError {
    UndefinedByteRegister,
    UndefinedWordRegister,
    UndefinedConditionCode,
    UndefinedOpcode,
}

impl Display for InvalidOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UndefinedByteRegister => "undefined byte register",
            Self::UndefinedWordRegister => "undefined word register",
            Self::UndefinedConditionCode => "undefined condition code",
            Self::UndefinedOpcode => "undefined opcode",
        })
    }
}

impl Error for InvalidOpcodeError {}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ConditionCode {
    Z,
    C,
    S,
    O,
    LE,
    BE,
    L,
    False,
    NZ,
    NC,
    NS,
    NO,
    G,
    A,
    GE,
    #[default]
    True,
}

impl std::ops::Not for ConditionCode {
    type Output = ConditionCode;
    fn not(self) -> Self::Output {
        use ConditionCode::*;
        match self {
            Z => NZ,
            C => NC,
            S => NS,
            O => NO,
            LE => G,
            BE => A,
            L => GE,
            False => True,
            NZ => Z,
            NC => C,
            NS => S,
            NO => O,
            G => LE,
            A => BE,
            GE => L,
            True => False,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AluBinaryOperation {
    Add,
    Sub,
    Adc,
    Sbb,
    And,
    Xor,
    Bic,
    Or,
    Shl,
    Shr,
    Sar = 11,
    Rol,
    Ror,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AluUnaryOperation {
    Neg,
    Not,
    Inc,
    Dec,
    Abs,
    Sgxt,
    Swap,
    Popcnt,
    Rcl,
    Rcr,
    Zero = 15,
}

impl TryFrom<u8> for ByteRegister {
    type Error = InvalidOpcodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ByteRegister::H,
            1 => ByteRegister::A,
            2 => ByteRegister::B,
            3 => ByteRegister::C,
            4 => ByteRegister::X,
            5 => ByteRegister::L,
            6 => ByteRegister::M,
            7 => ByteRegister::N,
            _ => return Err(InvalidOpcodeError::UndefinedByteRegister),
        })
    }
}

impl TryFrom<u8> for WordRegister {
    type Error = InvalidOpcodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => WordRegister::HA,
            1 => WordRegister::BC,
            2 => WordRegister::XL,
            3 => WordRegister::MN,
            4 => WordRegister::R4,
            5 => WordRegister::SP,
            6 => WordRegister::FL,
            7 => WordRegister::PC,
            _ => return Err(InvalidOpcodeError::UndefinedWordRegister),
        })
    }
}

impl TryFrom<u8> for ConditionCode {
    type Error = InvalidOpcodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ConditionCode::Z,
            1 => ConditionCode::C,
            2 => ConditionCode::S,
            3 => ConditionCode::O,
            4 => ConditionCode::LE,
            5 => ConditionCode::BE,
            6 => ConditionCode::L,
            7 => ConditionCode::False,
            8 => ConditionCode::NZ,
            9 => ConditionCode::NC,
            10 => ConditionCode::NS,
            11 => ConditionCode::NO,
            12 => ConditionCode::G,
            13 => ConditionCode::A,
            14 => ConditionCode::GE,
            15 => ConditionCode::True,
            _ => return Err(InvalidOpcodeError::UndefinedConditionCode),
        })
    }
}

impl TryFrom<u8> for AluBinaryOperation {
    type Error = InvalidOpcodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => AluBinaryOperation::Add,
            1 => AluBinaryOperation::Sub,
            2 => AluBinaryOperation::Adc,
            3 => AluBinaryOperation::Sbb,
            4 => AluBinaryOperation::And,
            5 => AluBinaryOperation::Xor,
            6 => AluBinaryOperation::Bic,
            7 => AluBinaryOperation::Or,
            8 => AluBinaryOperation::Shl,
            9 => AluBinaryOperation::Shr,
            10 => AluBinaryOperation::Sar,
            11 => AluBinaryOperation::Rol,
            12 => AluBinaryOperation::Ror,
            _ => return Err(InvalidOpcodeError::UndefinedOpcode),
        })
    }
}

impl TryFrom<u8> for AluUnaryOperation {
    type Error = InvalidOpcodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => AluUnaryOperation::Neg,
            1 => AluUnaryOperation::Not,
            2 => AluUnaryOperation::Inc,
            3 => AluUnaryOperation::Dec,
            4 => AluUnaryOperation::Abs,
            5 => AluUnaryOperation::Sgxt,
            6 => AluUnaryOperation::Swap,
            7 => AluUnaryOperation::Popcnt,
            8 => AluUnaryOperation::Rcl,
            9 => AluUnaryOperation::Rcr,
            15 => AluUnaryOperation::Zero,
            _ => return Err(InvalidOpcodeError::UndefinedOpcode),
        })
    }
}

impl Display for ConditionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ConditionCode::*;
        f.write_str(match self {
            Z => "Z",
            C => "C",
            S => "S",
            O => "O",
            LE => "LE",
            BE => "BE",
            L => "L",
            False => "[FALSE]",
            NZ => "NZ",
            NC => "NC",
            NS => "NS",
            NO => "NO",
            G => "G",
            A => "A",
            GE => "GE",
            True => "",
        })
    }
}

impl Display for AluBinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AluBinaryOperation::*;
        f.write_str(match self {
            Add => "ADD",
            Sub => "SUB",
            Adc => "ADC",
            Sbb => "SBB",
            And => "AND",
            Xor => "XOR",
            Bic => "BIC",
            Or => "OR",
            Shl => "SHL",
            Shr => "SHR",
            Sar => "SAR",
            Rol => "ROL",
            Ror => "ROR",
        })
    }
}

impl Display for AluUnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AluUnaryOperation::*;
        f.write_str(match self {
            Neg => "NEG",
            Not => "NOT",
            Inc => "INC",
            Dec => "DEC",
            Abs => "ABS",
            Sgxt => "SGXT",
            Swap => "SWAP",
            Popcnt => "POPCNT",
            Rcl => "RCL",
            Rcr => "RCR",
            Zero => "ZERO",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Opcode {
    MovRIB(ByteRegister, u8),
    MovRIW(WordRegister, u16),
    MovCcRRB(ConditionCode, ByteRegister, ByteRegister),
    MovCcRRW(ConditionCode, WordRegister, WordRegister),
    XchCcRRB(ConditionCode, ByteRegister, ByteRegister),
    XchCcRRW(ConditionCode, WordRegister, WordRegister),
    MovRAB(ByteRegister, u16),
    MovRAW(WordRegister, u16),
    MovCcROB(ConditionCode, ByteRegister, i8, WordRegister),
    MovCcROW(ConditionCode, WordRegister, i8, WordRegister),
    LeaCcROB(ConditionCode, ByteRegister, i8, WordRegister),
    LeaCcROW(ConditionCode, WordRegister, i8, WordRegister),
    JmpCcA(ConditionCode, u16),
    CallCcA(ConditionCode, u16),
    MovARB(u16, ByteRegister),
    MovARW(u16, WordRegister),
    MovCcORB(ConditionCode, i8, WordRegister, ByteRegister),
    MovCcORW(ConditionCode, i8, WordRegister, WordRegister),
    AlubRRB(AluBinaryOperation, ByteRegister, ByteRegister),
    AlubRRW(AluBinaryOperation, WordRegister, WordRegister),
    AlubRIB(AluBinaryOperation, ByteRegister, u8),
    AlubRIW(AluBinaryOperation, WordRegister, u16),
    AluuRB(AluUnaryOperation, ByteRegister),
    AluuRW(AluUnaryOperation, WordRegister),
    CpAlubRRB(AluBinaryOperation, ByteRegister, ByteRegister),
    CpAlubRRW(AluBinaryOperation, WordRegister, WordRegister),
    CpAlubRIB(AluBinaryOperation, ByteRegister, u8),
    CpAlubRIW(AluBinaryOperation, WordRegister, u16),
    CpAluuRB(AluUnaryOperation, ByteRegister),
    CpAluuRW(AluUnaryOperation, WordRegister),
    JrCcX(ConditionCode, i8),
    PushRB(ByteRegister),
    PushRW(WordRegister),
    PopRB(ByteRegister),
    PopRW(WordRegister),
    ClbRIB(ByteRegister, u8),
    ClbRIW(WordRegister, u8),
    ClbRRB(ByteRegister, ByteRegister),
    ClbRRW(WordRegister, ByteRegister),
    StbRIB(ByteRegister, u8),
    StbRIW(WordRegister, u8),
    StbRRB(ByteRegister, ByteRegister),
    StbRRW(WordRegister, ByteRegister),
    TgbRIB(ByteRegister, u8),
    TgbRIW(WordRegister, u8),
    TgbRRB(ByteRegister, ByteRegister),
    TgbRRW(WordRegister, ByteRegister),
    TbitRIB(ByteRegister, u8),
    TbitRIW(WordRegister, u8),
    TbitRRB(ByteRegister, ByteRegister),
    TbitRRW(WordRegister, ByteRegister),
    Nop,
    Halt,
}

impl Opcode {
    pub fn to_vec(self) -> Vec<u8> {
        use Opcode::*;

        match self {
            MovRIB(dst, imm) => vec![dst as u8, imm],
            MovRIW(dst, imm) => {
                let [hi, lo] = imm.to_be_bytes();
                vec![0x08 | dst as u8, hi, lo]
            }

            MovCcRRB(cc, dst, src) => {
                vec![0x10 | dst as u8, ((src as u8) << 5) | cc as u8]
            }
            MovCcRRW(cc, dst, src) => {
                vec![0x18 | dst as u8, ((src as u8) << 5) | cc as u8]
            }
            XchCcRRB(cc, dst, src) => {
                vec![0x10 | dst as u8, ((src as u8) << 5) | 0x10 | cc as u8]
            }
            XchCcRRW(cc, dst, src) => {
                vec![0x18 | dst as u8, ((src as u8) << 5) | 0x10 | cc as u8]
            }

            MovRAB(dst, addr) => {
                let [hi, lo] = addr.to_be_bytes();
                vec![0x20 | dst as u8, hi, lo]
            }
            MovRAW(dst, addr) => {
                let [hi, lo] = addr.to_be_bytes();
                vec![0x28 | dst as u8, hi, lo]
            }

            MovCcROB(cc, dst, offset, base) => {
                vec![
                    0x30 | dst as u8,
                    ((base as u8) << 5) | cc as u8,
                    offset as u8,
                ]
            }
            MovCcROW(cc, dst, offset, base) => {
                vec![
                    0x38 | dst as u8,
                    ((base as u8) << 5) | cc as u8,
                    offset as u8,
                ]
            }
            LeaCcROB(cc, dst, offset, base) => {
                vec![
                    0x30 | dst as u8,
                    ((base as u8) << 5) | 0x10 | cc as u8,
                    offset as u8,
                ]
            }
            LeaCcROW(cc, dst, offset, base) => {
                vec![
                    0x38 | dst as u8,
                    ((base as u8) << 5) | 0x10 | cc as u8,
                    offset as u8,
                ]
            }

            JmpCcA(cc, addr) => {
                let [hi, lo] = addr.to_be_bytes();
                vec![0x40 | cc as u8, hi, lo]
            }
            CallCcA(cc, addr) => {
                let [hi, lo] = addr.to_be_bytes();
                vec![0x50 | cc as u8, hi, lo]
            }

            MovARB(addr, src) => {
                let [hi, lo] = addr.to_be_bytes();
                vec![0x60 | src as u8, hi, lo]
            }
            MovARW(addr, src) => {
                let [hi, lo] = addr.to_be_bytes();
                vec![0x68 | src as u8, hi, lo]
            }

            MovCcORB(cc, offset, base, src) => {
                vec![
                    0x70 | src as u8,
                    ((base as u8) << 5) | cc as u8,
                    offset as u8,
                ]
            }
            MovCcORW(cc, offset, base, src) => {
                vec![
                    0x78 | src as u8,
                    ((base as u8) << 5) | cc as u8,
                    offset as u8,
                ]
            }

            AlubRRB(op, dst, src) => {
                vec![0x80 | dst as u8, ((src as u8) << 5) | 0x10 | op as u8]
            }
            AlubRRW(op, dst, src) => {
                vec![0x88 | dst as u8, ((src as u8) << 5) | 0x10 | op as u8]
            }
            AlubRIB(op, dst, imm) => vec![0x80 | dst as u8, op as u8, imm],
            AlubRIW(op, dst, imm) => {
                let [hi, lo] = imm.to_be_bytes();
                vec![0x88 | dst as u8, op as u8, hi, lo]
            }
            AluuRB(op, dst) => vec![0x80 | dst as u8, 0x80 | op as u8],
            AluuRW(op, dst) => vec![0x88 | dst as u8, 0x80 | op as u8],

            CpAlubRRB(op, dst, src) => {
                vec![0x90 | dst as u8, ((src as u8) << 5) | 0x10 | op as u8]
            }
            CpAlubRRW(op, dst, src) => {
                vec![0x98 | dst as u8, ((src as u8) << 5) | 0x10 | op as u8]
            }
            CpAlubRIB(op, dst, imm) => vec![0x90 | dst as u8, op as u8, imm],
            CpAlubRIW(op, dst, imm) => {
                let [hi, lo] = imm.to_be_bytes();
                vec![0x98 | dst as u8, op as u8, hi, lo]
            }
            CpAluuRB(op, dst) => vec![0x90 | dst as u8, 0x80 | op as u8],
            CpAluuRW(op, dst) => vec![0x98 | dst as u8, 0x80 | op as u8],

            JrCcX(cc, offset) => vec![0xA0 | cc as u8, offset as u8],

            PushRB(src) => vec![0xC0 | src as u8],
            PushRW(src) => vec![0xC8 | src as u8],
            PopRB(dst) => vec![0xD0 | dst as u8],
            PopRW(dst) => vec![0xD8 | dst as u8],

            ClbRIB(dst, bit) => vec![0xE0 | dst as u8, (bit & 0xF) << 4],
            ClbRIW(dst, bit) => vec![0xE8 | dst as u8, (bit & 0xF) << 4],
            ClbRRB(dst, bitreg) => vec![0xE0 | dst as u8, ((bitreg as u8) << 5) | 0x08],
            ClbRRW(dst, bitreg) => vec![0xE8 | dst as u8, ((bitreg as u8) << 5) | 0x08],

            StbRIB(dst, bit) => vec![0xE0 | dst as u8, ((bit & 0xF) << 4) | 0x01],
            StbRIW(dst, bit) => vec![0xE8 | dst as u8, ((bit & 0xF) << 4) | 0x01],
            StbRRB(dst, bitreg) => vec![0xE0 | dst as u8, ((bitreg as u8) << 5) | 0x09],
            StbRRW(dst, bitreg) => vec![0xE8 | dst as u8, ((bitreg as u8) << 5) | 0x09],

            TgbRIB(dst, bit) => vec![0xE0 | dst as u8, ((bit & 0xF) << 4) | 0x02],
            TgbRIW(dst, bit) => vec![0xE8 | dst as u8, ((bit & 0xF) << 4) | 0x02],
            TgbRRB(dst, bitreg) => vec![0xE0 | dst as u8, ((bitreg as u8) << 5) | 0x0A],
            TgbRRW(dst, bitreg) => vec![0xE8 | dst as u8, ((bitreg as u8) << 5) | 0x0A],

            TbitRIB(src, bit) => vec![0xE0 | src as u8, ((bit & 0xF) << 4) | 0x03],
            TbitRIW(src, bit) => vec![0xE8 | src as u8, ((bit & 0xF) << 4) | 0x03],
            TbitRRB(src, bitreg) => vec![0xE0 | src as u8, ((bitreg as u8) << 5) | 0x0B],
            TbitRRW(src, bitreg) => vec![0xE8 | src as u8, ((bitreg as u8) << 5) | 0x0B],

            Nop => vec![0xF0],
            Halt => vec![0xFF],
        }
    }
}

impl Opcode {
    /// Decodes an `Opcode` from the front of `bytes`. Extra trailing bytes are
    /// permitted and ignored — only as many bytes as the instruction needs are read.
    pub fn from_slice(bytes: &[u8]) -> Result<Opcode, InvalidOpcodeError> {
        let get = |i: usize| {
            bytes
                .get(i)
                .copied()
                .ok_or(InvalidOpcodeError::UndefinedOpcode)
        };
        let read_u16 = |i: usize| -> Result<u16, InvalidOpcodeError> {
            Ok(((get(i)? as u16) << 8) | get(i + 1)? as u16)
        };

        let b0 = get(0)?;

        match b0 >> 4 {
            // MOV %b, $ / MOV %w, $
            0b0000 => {
                let ddd = b0 & 0x7;
                if (b0 >> 3) & 1 == 1 {
                    let dst = WordRegister::try_from(ddd)?;
                    let imm = read_u16(1)?;
                    Ok(Opcode::MovRIW(dst, imm))
                } else {
                    let dst = ByteRegister::try_from(ddd)?;
                    let imm = get(1)?;
                    Ok(Opcode::MovRIB(dst, imm))
                }
            }

            // MOVcc %, % / XCHcc %, %
            0b0001 => {
                let l = (b0 >> 3) & 1;
                let ddd = b0 & 0x7;
                let b2 = get(1)?;
                let sss = (b2 >> 5) & 0x7;
                let is_xch = (b2 >> 4) & 1 == 1;
                let cc = ConditionCode::try_from(b2 & 15)?;
                if l == 0 {
                    let (dst, src) = (ByteRegister::try_from(ddd)?, ByteRegister::try_from(sss)?);
                    Ok(if is_xch {
                        Opcode::XchCcRRB(cc, dst, src)
                    } else {
                        Opcode::MovCcRRB(cc, dst, src)
                    })
                } else {
                    let (dst, src) = (WordRegister::try_from(ddd)?, WordRegister::try_from(sss)?);
                    Ok(if is_xch {
                        Opcode::XchCcRRW(cc, dst, src)
                    } else {
                        Opcode::MovCcRRW(cc, dst, src)
                    })
                }
            }

            // MOV %, a
            0b0010 => {
                let l = (b0 >> 3) & 1;
                let ddd = b0 & 0x7;
                let addr = read_u16(1)?;
                if l == 0 {
                    Ok(Opcode::MovRAB(ByteRegister::try_from(ddd)?, addr))
                } else {
                    Ok(Opcode::MovRAW(WordRegister::try_from(ddd)?, addr))
                }
            }

            // MOVcc %, o / LEAcc %, o  (also covers the "Jcc o" alias, via dst == PC)
            0b0011 => {
                let l = (b0 >> 3) & 1;
                let ddd = b0 & 0x7;
                let b2 = get(1)?;
                let sss = (b2 >> 5) & 0x7;
                let is_lea = (b2 >> 4) & 1 == 1;
                let cc = ConditionCode::try_from(b2 & 15)?;
                let offset = get(2)? as i8;
                let base = WordRegister::try_from(sss)?;
                if l == 0 {
                    let dst = ByteRegister::try_from(ddd)?;
                    Ok(if is_lea {
                        Opcode::LeaCcROB(cc, dst, offset, base)
                    } else {
                        Opcode::MovCcROB(cc, dst, offset, base)
                    })
                } else {
                    let dst = WordRegister::try_from(ddd)?;
                    Ok(if is_lea {
                        Opcode::LeaCcROW(cc, dst, offset, base)
                    } else {
                        Opcode::MovCcROW(cc, dst, offset, base)
                    })
                }
            }

            // Jcc a
            0b0100 => {
                let cc = ConditionCode::try_from(b0 & 15)?;
                let addr = read_u16(1)?;
                Ok(Opcode::JmpCcA(cc, addr))
            }

            // CALLcc a
            0b0101 => {
                let cc = ConditionCode::try_from(b0 & 15)?;
                let addr = read_u16(1)?;
                Ok(Opcode::CallCcA(cc, addr))
            }

            // MOV a, %
            0b0110 => {
                let l = (b0 >> 3) & 1;
                let sss = b0 & 0x7;
                let addr = read_u16(1)?;
                if l == 0 {
                    Ok(Opcode::MovARB(addr, ByteRegister::try_from(sss)?))
                } else {
                    Ok(Opcode::MovARW(addr, WordRegister::try_from(sss)?))
                }
            }

            // MOVcc o, %
            0b0111 => {
                let l = (b0 >> 3) & 1;
                let sss = b0 & 0x7;
                let b2 = get(1)?;
                let base = WordRegister::try_from((b2 >> 5) & 0x7)?;
                let cc = ConditionCode::try_from(b2 & 15)?;
                let offset = get(2)? as i8;
                if l == 0 {
                    Ok(Opcode::MovCcORB(
                        cc,
                        offset,
                        base,
                        ByteRegister::try_from(sss)?,
                    ))
                } else {
                    Ok(Opcode::MovCcORW(
                        cc,
                        offset,
                        base,
                        WordRegister::try_from(sss)?,
                    ))
                }
            }

            // alub/aluu %, % and %, $ ; CPalub/CPaluu variants (incl. CMP/TEST aliases)
            0b1000 | 0b1001 => {
                let is_cp = (b0 >> 4) & 1 == 1;
                let l = (b0 >> 3) & 1;
                let ddd = b0 & 0x7;
                let b2 = get(1)?;
                let reg_form = (b2 >> 4) & 1 == 1;
                let unary_form = (b2 >> 7) & 1 == 1;

                if reg_form {
                    let sss = (b2 >> 5) & 0x7;
                    let op = AluBinaryOperation::try_from(b2 & 15)?;
                    if l == 0 {
                        let (dst, src) =
                            (ByteRegister::try_from(ddd)?, ByteRegister::try_from(sss)?);
                        Ok(if is_cp {
                            Opcode::CpAlubRRB(op, dst, src)
                        } else {
                            Opcode::AlubRRB(op, dst, src)
                        })
                    } else {
                        let (dst, src) =
                            (WordRegister::try_from(ddd)?, WordRegister::try_from(sss)?);
                        Ok(if is_cp {
                            Opcode::CpAlubRRW(op, dst, src)
                        } else {
                            Opcode::AlubRRW(op, dst, src)
                        })
                    }
                } else if unary_form {
                    let op = AluUnaryOperation::try_from(b2 & 15)?;
                    if l == 0 {
                        let dst = ByteRegister::try_from(ddd)?;
                        Ok(if is_cp {
                            Opcode::CpAluuRB(op, dst)
                        } else {
                            Opcode::AluuRB(op, dst)
                        })
                    } else {
                        let dst = WordRegister::try_from(ddd)?;
                        Ok(if is_cp {
                            Opcode::CpAluuRW(op, dst)
                        } else {
                            Opcode::AluuRW(op, dst)
                        })
                    }
                } else {
                    let op = AluBinaryOperation::try_from(b2 & 15)?;
                    if l == 0 {
                        let dst = ByteRegister::try_from(ddd)?;
                        let imm = get(2)?;
                        Ok(if is_cp {
                            Opcode::CpAlubRIB(op, dst, imm)
                        } else {
                            Opcode::AlubRIB(op, dst, imm)
                        })
                    } else {
                        let dst = WordRegister::try_from(ddd)?;
                        let imm = read_u16(2)?;
                        Ok(if is_cp {
                            Opcode::CpAlubRIW(op, dst, imm)
                        } else {
                            Opcode::AlubRIW(op, dst, imm)
                        })
                    }
                }
            }

            // JRcc rel
            0b1010 => {
                let cc = ConditionCode::try_from(b0 & 15)?;
                let offset = get(1)? as i8;
                Ok(Opcode::JrCcX(cc, offset))
            }

            // Reserved
            0b1011 => Err(InvalidOpcodeError::UndefinedOpcode),

            // PUSH %
            0b1100 => {
                let l = (b0 >> 3) & 1;
                let sss = b0 & 0x7;
                if l == 0 {
                    Ok(Opcode::PushRB(ByteRegister::try_from(sss)?))
                } else {
                    Ok(Opcode::PushRW(WordRegister::try_from(sss)?))
                }
            }

            // POP % (RET is the L=1, DDD=PC case, decoded naturally as PopRW(PC))
            0b1101 => {
                let l = (b0 >> 3) & 1;
                let ddd = b0 & 0x7;
                if l == 0 {
                    Ok(Opcode::PopRB(ByteRegister::try_from(ddd)?))
                } else {
                    Ok(Opcode::PopRW(WordRegister::try_from(ddd)?))
                }
            }

            // CLB / STB / TGB / TBIT
            0b1110 => {
                let l = (b0 >> 3) & 1;
                let ddd = b0 & 0x7;
                let b2 = get(1)?;
                let is_reg_bit = (b2 >> 3) & 1 == 1;
                let selector = b2 & 0x3;

                if !is_reg_bit {
                    let bit = (b2 >> 4) & 0xF;
                    if l == 0 {
                        let r = ByteRegister::try_from(ddd)?;
                        match selector {
                            0 => Ok(Opcode::ClbRIB(r, bit)),
                            1 => Ok(Opcode::StbRIB(r, bit)),
                            2 => Ok(Opcode::TgbRIB(r, bit)),
                            3 => Ok(Opcode::TbitRIB(r, bit)),
                            _ => unreachable!(),
                        }
                    } else {
                        let r = WordRegister::try_from(ddd)?;
                        match selector {
                            0 => Ok(Opcode::ClbRIW(r, bit)),
                            1 => Ok(Opcode::StbRIW(r, bit)),
                            2 => Ok(Opcode::TgbRIW(r, bit)),
                            3 => Ok(Opcode::TbitRIW(r, bit)),
                            _ => unreachable!(),
                        }
                    }
                } else {
                    let bitreg = ByteRegister::try_from((b2 >> 5) & 0x7)?;
                    if l == 0 {
                        let r = ByteRegister::try_from(ddd)?;
                        match selector {
                            0 => Ok(Opcode::ClbRRB(r, bitreg)),
                            1 => Ok(Opcode::StbRRB(r, bitreg)),
                            2 => Ok(Opcode::TgbRRB(r, bitreg)),
                            3 => Ok(Opcode::TbitRRB(r, bitreg)),
                            _ => unreachable!(),
                        }
                    } else {
                        let r = WordRegister::try_from(ddd)?;
                        match selector {
                            0 => Ok(Opcode::ClbRRW(r, bitreg)),
                            1 => Ok(Opcode::StbRRW(r, bitreg)),
                            2 => Ok(Opcode::TgbRRW(r, bitreg)),
                            3 => Ok(Opcode::TbitRRW(r, bitreg)),
                            _ => unreachable!(),
                        }
                    }
                }
            }

            // NOP / HALT / reserved
            0b1111 => match b0 {
                0xF0 => Ok(Opcode::Nop),
                0xFF => Ok(Opcode::Halt),
                _ => Err(InvalidOpcodeError::UndefinedOpcode),
            },

            _ => unreachable!(),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Opcode::*;
        match self {
            MovRIB(dst, imm) => {
                write!(f, "MOV %{dst}, ${imm:#x}")
            }
            MovRIW(dst, imm) => {
                write!(f, "MOV %{dst}, ${imm:#x}")
            }

            MovCcRRB(cc, dst, src) => {
                write!(f, "MOV{cc} %{dst}, %{src}")
            }
            MovCcRRW(cc, dst, src) => {
                write!(f, "MOV{cc} %{dst}, %{src}")
            }
            XchCcRRB(cc, dst, src) => {
                write!(f, "XCH{cc} %{dst}, %{src}")
            }
            XchCcRRW(cc, dst, src) => {
                write!(f, "XCH{cc} %{dst}, %{src}")
            }

            MovRAB(dst, addr) => {
                write!(f, "MOV %{dst}, {addr:#x}")
            }
            MovRAW(dst, addr) => {
                write!(f, "MOV %{dst}, {addr:#x}")
            }

            MovCcROB(cc, dst, offset, base) => {
                write!(f, "MOV{cc} %{dst}, {offset}(%{base})")
            }
            MovCcROW(cc, dst, offset, base) => {
                write!(f, "MOV{cc} %{dst}, {offset}(%{base})")
            }
            LeaCcROB(cc, dst, offset, base) => {
                write!(f, "LEA{cc} %{dst}, {offset}(%{base})")
            }
            LeaCcROW(cc, dst, offset, base) => {
                write!(f, "LEA{cc} %{dst}, {offset}(%{base})")
            }

            JmpCcA(cc, addr) => {
                write!(f, "JMP{cc} {addr:#x}")
            }
            CallCcA(cc, addr) => {
                write!(f, "CALL{cc} {addr:#x}")
            }

            MovARB(addr, src) => {
                write!(f, "MOV {addr:#x}, %{src}")
            }
            MovARW(addr, src) => {
                write!(f, "MOV {addr:#x}, %{src}")
            }

            MovCcORB(cc, offset, base, src) => {
                write!(f, "MOV{cc} {offset}(%{base}), %{src}")
            }
            MovCcORW(cc, offset, base, src) => {
                write!(f, "MOV{cc} {offset}(%{base}), %{src}")
            }

            AlubRRB(op, dst, src) => {
                write!(f, "{op} %{dst}, %{src}")
            }
            AlubRRW(op, dst, src) => {
                write!(f, "{op} %{dst}, %{src}")
            }
            AlubRIB(op, dst, imm) => {
                write!(f, "{op} %{dst}, ${imm:#x}")
            }
            AlubRIW(op, dst, imm) => {
                write!(f, "{op} %{dst}, ${imm:#x}")
            }
            AluuRB(op, dst) => {
                write!(f, "{op} %{dst}")
            }
            AluuRW(op, dst) => {
                write!(f, "{op} %{dst}")
            }

            CpAlubRRB(op, dst, src) => {
                write!(f, "CP{op} %{dst}, %{src}")
            }
            CpAlubRRW(op, dst, src) => {
                write!(f, "CP{op} %{dst}, %{src}")
            }
            CpAlubRIB(op, dst, imm) => {
                write!(f, "CP{op} %{dst}, ${imm:#x}")
            }
            CpAlubRIW(op, dst, imm) => {
                write!(f, "CP{op} %{dst}, ${imm:#x}")
            }
            CpAluuRB(op, dst) => {
                write!(f, "CP{op} %{dst}")
            }
            CpAluuRW(op, dst) => {
                write!(f, "CP{op} %{dst}")
            }

            JrCcX(cc, offset) => {
                write!(f, "JR{cc} {offset:+}")
            }

            PushRB(src) => {
                write!(f, "PUSH %{src}")
            }
            PushRW(src) => {
                write!(f, "PUSH %{src}")
            }
            PopRB(dst) => {
                write!(f, "POP %{dst}")
            }
            PopRW(dst) => {
                write!(f, "POP %{dst}")
            }

            ClbRIB(dst, bit) => {
                write!(f, "CLB %{dst}, ${bit}")
            }
            ClbRIW(dst, bit) => {
                write!(f, "CLB %{dst}, ${bit}")
            }
            ClbRRB(dst, bitreg) => {
                write!(f, "CLB %{dst}, %{bitreg}")
            }
            ClbRRW(dst, bitreg) => {
                write!(f, "CLB %{dst}, %{bitreg}")
            }

            StbRIB(dst, bit) => {
                write!(f, "STB %{dst}, ${bit}")
            }
            StbRIW(dst, bit) => {
                write!(f, "STB %{dst}, ${bit}")
            }
            StbRRB(dst, bitreg) => {
                write!(f, "STB %{dst}, %{bitreg}")
            }
            StbRRW(dst, bitreg) => {
                write!(f, "STB %{dst}, %{bitreg}")
            }

            TgbRIB(dst, bit) => {
                write!(f, "TGB %{dst}, ${bit}")
            }
            TgbRIW(dst, bit) => {
                write!(f, "TGB %{dst}, ${bit}")
            }
            TgbRRB(dst, bitreg) => {
                write!(f, "TGB %{dst}, %{bitreg}")
            }
            TgbRRW(dst, bitreg) => {
                write!(f, "TGB %{dst}, %{bitreg}")
            }

            TbitRIB(src, bit) => {
                write!(f, "TBIT %{src}, ${bit}")
            }
            TbitRIW(src, bit) => {
                write!(f, "TBIT %{src}, ${bit}")
            }
            TbitRRB(src, bitreg) => {
                write!(f, "TBIT %{src}, %{bitreg}")
            }
            TbitRRW(src, bitreg) => {
                write!(f, "TBIT %{src}, %{bitreg}")
            }

            Nop => {
                write!(f, "NOP")
            }
            Halt => {
                write!(f, "HALT")
            }
        }
    }
}
