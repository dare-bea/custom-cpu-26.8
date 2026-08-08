//! This module defines the opcodes used in the `CPU3v2` emulator, along with parsing and display functionality.

use std::error::Error;
use std::fmt::Display;

use crate::register::{ByteRegister, WordRegister};

/// An error that occurs when decoding an opcode from bytes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InvalidOpcodeError {
    /// An undefined byte register was encountered while decoding an opcode.
    UndefinedByteRegister,
    /// An undefined word register was encountered while decoding an opcode.
    UndefinedWordRegister,
    /// An undefined condition code was encountered while decoding an opcode.
    UndefinedConditionCode,
    /// An undefined opcode was encountered.
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

/// A condition code used in conditional instructions.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ConditionCode {
    /// Zero
    Z,
    /// Unsigned carry (or Below / Borrow) (unsigned)
    C,
    /// Sign
    S,
    /// Signed Overflow
    O,
    /// Less than or equal (signed)
    LE,
    /// Below or equal (unsigned)
    BE,
    /// Less than (signed)
    L,
    /// False
    False,
    /// Not zero
    NZ,
    /// Not unsigned carry (or Not below / Not borrow) (unsigned)
    NC,
    /// Not sign
    NS,
    /// Not signed overflow
    NO,
    /// Greater than (signed)
    G,
    /// Above (unsigned)
    A,
    /// Greater than or equal (signed)
    GE,
    /// True
    #[default]
    True,
}

impl std::ops::Not for ConditionCode {
    type Output = ConditionCode;
    fn not(self) -> Self::Output {
        use ConditionCode::{A, BE, C, False, G, GE, L, LE, NC, NO, NS, NZ, O, S, True, Z};
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

/// An ALU binary operation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AluBinaryOperation {
    /// Add
    Add,
    /// Subtract
    Sub,
    /// Add with carry
    Adc,
    /// Subtract with borrow
    Sbb,
    /// Bitwise AND
    And,
    /// Bitwise XOR
    Xor,
    /// Bitwise AND NOT
    Bic,
    /// Bitwise OR
    Or,
    /// Shift left
    Shl,
    /// Shift right
    Shr,
    /// Arithmetic shift right
    Sar = 11,
    /// Rotate left
    Rol,
    /// Rotate right
    Ror,
}

/// An ALU unary operation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AluUnaryOperation {
    /// Negate
    Neg,
    /// Bitwise NOT
    Not,
    /// Increment
    Inc,
    /// Decrement
    Dec,
    /// Absolute value
    Abs,
    /// Sign-extend
    Sgxt,
    /// Swap nibbles / bytes
    Swap,
    /// Population count
    Popcnt,
    /// Rotate left through carry
    Rcl,
    /// Rotate right through carry
    Rcr,
    /// Zero (flags set to original value)
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
            11 => AluBinaryOperation::Sar,
            12 => AluBinaryOperation::Rol,
            13 => AluBinaryOperation::Ror,
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
        use ConditionCode::{A, BE, C, False, G, GE, L, LE, NC, NO, NS, NZ, O, S, True, Z};
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
        use AluBinaryOperation::{Adc, Add, And, Bic, Or, Rol, Ror, Sar, Sbb, Shl, Shr, Sub, Xor};
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
        use AluUnaryOperation::{Abs, Dec, Inc, Neg, Not, Popcnt, Rcl, Rcr, Sgxt, Swap, Zero};
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

/// An opcode for the system.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Opcode {
    /// Move immediate to byte register
    MovRIB(ByteRegister, u8),
    /// Move immediate to word register
    MovRIW(WordRegister, u16),
    /// Conditional move from byte register to byte register
    MovCcRRB(ConditionCode, ByteRegister, ByteRegister),
    /// Conditional move from word register to word register
    MovCcRRW(ConditionCode, WordRegister, WordRegister),
    /// Conditional exchange between byte register and byte register
    XchCcRRB(ConditionCode, ByteRegister, ByteRegister),
    /// Conditional exchange between word register and word register
    XchCcRRW(ConditionCode, WordRegister, WordRegister),
    /// Move from immediate address to byte register
    MovRAB(ByteRegister, u16),
    /// Move from immediate address to word register
    MovRAW(WordRegister, u16),
    /// Conditional move from offset of base register to byte register
    MovCcROB(ConditionCode, ByteRegister, i8, WordRegister),
    /// Conditional move from offset of base register to word register
    MovCcROW(ConditionCode, WordRegister, i8, WordRegister),
    /// Conditional load effective address from offset of base register to byte register
    LeaCcROB(ConditionCode, ByteRegister, i8, WordRegister),
    /// Conditional load effective address from offset of base register to word register
    LeaCcROW(ConditionCode, WordRegister, i8, WordRegister),
    /// Conditional jump to address
    JmpCcA(ConditionCode, u16),
    /// Conditional call to address
    CallCcA(ConditionCode, u16),
    /// Move from byte register to immediate address
    MovARB(u16, ByteRegister),
    /// Move from word register to immediate address
    MovARW(u16, WordRegister),
    /// Conditional move from byte register to offset of base register
    MovCcORB(ConditionCode, i8, WordRegister, ByteRegister),
    /// Conditional move from word register to offset of base register
    MovCcORW(ConditionCode, i8, WordRegister, WordRegister),
    /// ALU binary operation between byte registers
    AlubRRB(AluBinaryOperation, ByteRegister, ByteRegister),
    /// ALU binary operation between word registers
    AlubRRW(AluBinaryOperation, WordRegister, WordRegister),
    /// ALU binary operation between byte register and immediate
    AlubRIB(AluBinaryOperation, ByteRegister, u8),
    /// ALU binary operation between word register and immediate
    AlubRIW(AluBinaryOperation, WordRegister, u16),
    /// ALU unary operation on byte register
    AluuRB(AluUnaryOperation, ByteRegister),
    /// ALU unary operation on word register
    AluuRW(AluUnaryOperation, WordRegister),
    /// Compare and set flags for ALU binary operation between byte registers
    CpAlubRRB(AluBinaryOperation, ByteRegister, ByteRegister),
    /// Compare and set flags for ALU binary operation between word registers
    CpAlubRRW(AluBinaryOperation, WordRegister, WordRegister),
    /// Compare and set flags for ALU binary operation between byte register and immediate
    CpAlubRIB(AluBinaryOperation, ByteRegister, u8),
    /// Compare and set flags for ALU binary operation between word register and immediate
    CpAlubRIW(AluBinaryOperation, WordRegister, u16),
    /// Compare and set flags for ALU unary operation on byte register
    CpAluuRB(AluUnaryOperation, ByteRegister),
    /// Compare and set flags for ALU unary operation on word register
    CpAluuRW(AluUnaryOperation, WordRegister),
    /// Conditional relative jump by offset
    JrCcX(ConditionCode, i8),
    /// Push byte register onto stack
    PushRB(ByteRegister),
    /// Push word register onto stack
    PushRW(WordRegister),
    /// Pop byte register from stack
    PopRB(ByteRegister),
    /// Pop word register from stack
    PopRW(WordRegister),
    /// Clear immediate bit in byte register
    ClbRIB(ByteRegister, u8),
    /// Clear immediate bit in word register
    ClbRIW(WordRegister, u8),
    /// Clear byte-register-determined bit in word register
    ClbRRB(ByteRegister, ByteRegister),
    /// Clear byte-register-determined bit in word register
    ClbRRW(WordRegister, ByteRegister),
    /// Set immediate bit in byte register
    StbRIB(ByteRegister, u8),
    /// Set immediate bit in word register
    StbRIW(WordRegister, u8),
    /// Set byte-register-determined bit in word register
    StbRRB(ByteRegister, ByteRegister),
    /// Set byte-register-determined bit in word register
    StbRRW(WordRegister, ByteRegister),
    /// Toggle immediate bit in byte register
    TgbRIB(ByteRegister, u8),
    /// Toggle immediate bit in word register
    TgbRIW(WordRegister, u8),
    /// Toggle byte-register-determined bit in word register
    TgbRRB(ByteRegister, ByteRegister),
    /// Toggle byte-register-determined bit in word register
    TgbRRW(WordRegister, ByteRegister),
    /// Test immediate bit in byte register
    TbitRIB(ByteRegister, u8),
    /// Test immediate bit in word register
    TbitRIW(WordRegister, u8),
    /// Test byte-register-determined bit in word register
    TbitRRB(ByteRegister, ByteRegister),
    /// Test byte-register-determined bit in word register
    TbitRRW(WordRegister, ByteRegister),
    /// No operation
    Nop,
    /// Halt the CPU
    Halt,
}

impl Opcode {
    /// Converts the opcode into a vector of bytes representing its binary encoding.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn to_vec(self) -> Vec<u8> {
        use Opcode::{
            AlubRIB, AlubRIW, AlubRRB, AlubRRW, AluuRB, AluuRW, CallCcA, ClbRIB, ClbRIW, ClbRRB,
            ClbRRW, CpAlubRIB, CpAlubRIW, CpAlubRRB, CpAlubRRW, CpAluuRB, CpAluuRW, Halt, JmpCcA,
            JrCcX, LeaCcROB, LeaCcROW, MovARB, MovARW, MovCcORB, MovCcORW, MovCcROB, MovCcROW,
            MovCcRRB, MovCcRRW, MovRAB, MovRAW, MovRIB, MovRIW, Nop, PopRB, PopRW, PushRB, PushRW,
            StbRIB, StbRIW, StbRRB, StbRRW, TbitRIB, TbitRIW, TbitRRB, TbitRRW, TgbRIB, TgbRIW,
            TgbRRB, TgbRRW, XchCcRRB, XchCcRRW,
        };

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
                    offset.cast_unsigned(),
                ]
            }
            MovCcROW(cc, dst, offset, base) => {
                vec![
                    0x38 | dst as u8,
                    ((base as u8) << 5) | cc as u8,
                    offset.cast_unsigned(),
                ]
            }
            LeaCcROB(cc, dst, offset, base) => {
                vec![
                    0x30 | dst as u8,
                    ((base as u8) << 5) | 0x10 | cc as u8,
                    offset.cast_unsigned(),
                ]
            }
            LeaCcROW(cc, dst, offset, base) => {
                vec![
                    0x38 | dst as u8,
                    ((base as u8) << 5) | 0x10 | cc as u8,
                    offset.cast_unsigned(),
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
                    offset.cast_unsigned(),
                ]
            }
            MovCcORW(cc, offset, base, src) => {
                vec![
                    0x78 | src as u8,
                    ((base as u8) << 5) | cc as u8,
                    offset.cast_unsigned(),
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

            JrCcX(cc, offset) => vec![0xA0 | cc as u8, offset.cast_unsigned()],

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
    /// Decodes an [`Opcode`] from `bytes`. Extra trailing bytes are ignored.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidOpcodeError`] if the bytes do not represent a valid opcode.
    #[allow(clippy::too_many_lines)]
    pub fn from_slice(bytes: &[u8]) -> Result<Opcode, InvalidOpcodeError> {
        let get = |i: usize| {
            bytes
                .get(i)
                .copied()
                .ok_or(InvalidOpcodeError::UndefinedOpcode)
        };
        let read_u16 = |i: usize| -> Result<u16, InvalidOpcodeError> {
            Ok((u16::from(get(i)?) << 8) | u16::from(get(i + 1)?))
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
                let offset = get(2)?.cast_signed();
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
                let offset = get(2)?.cast_signed();
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
                let offset = get(1)?.cast_signed();
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

                if is_reg_bit {
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
                } else {
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
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Opcode::{
            AlubRIB, AlubRIW, AlubRRB, AlubRRW, AluuRB, AluuRW, CallCcA, ClbRIB, ClbRIW, ClbRRB,
            ClbRRW, CpAlubRIB, CpAlubRIW, CpAlubRRB, CpAlubRRW, CpAluuRB, CpAluuRW, Halt, JmpCcA,
            JrCcX, LeaCcROB, LeaCcROW, MovARB, MovARW, MovCcORB, MovCcORW, MovCcROB, MovCcROW,
            MovCcRRB, MovCcRRW, MovRAB, MovRAW, MovRIB, MovRIW, Nop, PopRB, PopRW, PushRB, PushRW,
            StbRIB, StbRIW, StbRRB, StbRRW, TbitRIB, TbitRIW, TbitRRB, TbitRRW, TgbRIB, TgbRIW,
            TgbRRB, TgbRRW, XchCcRRB, XchCcRRW,
        };
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
