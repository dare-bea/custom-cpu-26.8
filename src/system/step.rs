use crate::opcode::{AluBinaryOperation, AluUnaryOperation, ConditionCode, Opcode};
use crate::register::WordRegister;
use crate::register::WordRegister::{FL, PC, SP};
use crate::system::{SP_START, System, SystemError};

impl System {
    /// Checks a condition code against the current state of the system's flags.
    #[inline]
    #[must_use]
    pub fn condition(&self, cc: ConditionCode) -> bool {
        let flags = self.get_regw(WordRegister::FL);
        match cc {
            ConditionCode::Z => flags & (1 << 0) != 0,
            ConditionCode::C => flags & (1 << 1) != 0,
            ConditionCode::S => flags & (1 << 2) != 0,
            ConditionCode::O => flags & (1 << 3) != 0,
            ConditionCode::LE => {
                (flags & (1 << 3) != 0) != (flags & (1 << 2) != 0) || flags & (1 << 0) != 0
            }
            ConditionCode::BE => flags & (1 << 1) != 0 || flags & (1 << 0) != 0,
            ConditionCode::L => (flags & (1 << 3) != 0) != (flags & (1 << 2) != 0),
            ConditionCode::False => false,
            ConditionCode::NZ => flags & (1 << 0) == 0,
            ConditionCode::NC => flags & (1 << 1) == 0,
            ConditionCode::NS => flags & (1 << 2) == 0,
            ConditionCode::NO => flags & (1 << 3) == 0,
            ConditionCode::G => {
                !((flags & (1 << 3) != 0) != (flags & (1 << 2) != 0) || flags & (1 << 0) != 0)
            }
            ConditionCode::A => !(flags & (1 << 1) != 0 || flags & (1 << 0) != 0),
            ConditionCode::GE => (flags & (1 << 3) != 0) == (flags & (1 << 2) != 0),
            ConditionCode::True => true,
        }
    }

    /// Calculates a binary ALU operation on two 8-bit operands, updating the system's flags accordingly.
    pub fn alu_binaryb(&mut self, op: AluBinaryOperation, lhs: u8, rhs: u8) -> u8 {
        use AluBinaryOperation::{Adc, Add, And, Bic, Or, Rol, Ror, Sar, Sbb, Shl, Shr, Sub, Xor};

        let flags = self.get_regw(FL);
        let (value, cf, of) = match op {
            Add => {
                let (value, cf) = lhs.overflowing_add(rhs);
                let of = lhs.cast_signed().checked_add(rhs.cast_signed()).is_none();
                (value, cf, of)
            }
            Sub => {
                let (value, cf) = lhs.overflowing_sub(rhs);
                let of = lhs.cast_signed().checked_sub(rhs.cast_signed()).is_none();
                (value, cf, of)
            }
            Adc => {
                let (t, cf1) = lhs.overflowing_add(rhs);
                let (value, cf2) = t.overflowing_add((flags & 2 != 0).into());
                let (t, of1) = lhs.cast_signed().overflowing_add(rhs.cast_signed());
                let (_, of2) = t.overflowing_add((flags & 2 != 0).into());
                (value, cf1 || cf2, of1 || of2)
            }
            Sbb => {
                let (t, cf1) = lhs.overflowing_sub(rhs);
                let (value, cf2) = t.overflowing_sub((flags & 2 != 0).into());
                let (t, of1) = lhs.cast_signed().overflowing_sub(rhs.cast_signed());
                let (_, of2) = t.overflowing_sub((flags & 2 != 0).into());
                (value, cf1 || cf2, of1 || of2)
            }
            And => (lhs & rhs, false, false),
            Xor => (lhs ^ rhs, false, false),
            Bic => (lhs & !rhs, false, false),
            Or => (lhs | rhs, false, false),
            Shl => {
                let shift = u32::from(rhs & 7);
                let cf = shift != 0 && lhs & (0x80 >> (shift - 1)) != 0;
                (lhs.wrapping_shl(shift), cf, false)
            }
            Shr => {
                let shift = u32::from(rhs & 7);
                let cf = shift != 0 && lhs & (1 << (shift - 1)) != 0;
                (lhs.wrapping_shr(shift), cf, false)
            }
            /*Sal => {
                let shift = u32::from(rhs & 7);
                let cf = shift != 0 && lhs & (0x80 >> (shift - 1)) != 0;
                (lhs.cast_signed().wrapping_shl(shift).cast_unsigned(), cf, false)
            },*/
            Sar => {
                let shift = u32::from(rhs & 7);
                let cf = shift != 0 && lhs & (1 << (shift - 1)) != 0;
                (
                    lhs.cast_signed().wrapping_shr(shift).cast_unsigned(),
                    cf,
                    false,
                )
            }
            Rol => {
                let shift = u32::from(rhs & 7);
                let cf = shift != 0 && lhs & (0x80 >> (shift - 1)) != 0;
                (lhs.rotate_left(shift), cf, false)
            }
            Ror => {
                let shift = u32::from(rhs & 7);
                let cf = shift != 0 && lhs & (1 << (shift - 1)) != 0;
                (lhs.rotate_right(shift), cf, false)
            }
        };
        let sf = value.cast_signed() < 0;
        let zf = value == 0;
        self.set_regw(
            FL,
            (flags & !15)
                | u16::from(zf)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        value
    }

    /// Calculates a binary ALU operation on two 16-bit operands, updating the system's flags accordingly.
    pub fn alu_binaryw(&mut self, op: AluBinaryOperation, lhs: u16, rhs: u16) -> u16 {
        use AluBinaryOperation::{Adc, Add, And, Bic, Or, Rol, Ror, Sar, Sbb, Shl, Shr, Sub, Xor};

        let flags = self.get_regw(FL);
        let (value, cf, of) = match op {
            Add => {
                let (value, cf) = lhs.overflowing_add(rhs);
                let of = lhs.cast_signed().checked_add(rhs.cast_signed()).is_none();
                (value, cf, of)
            }
            Sub => {
                let (value, cf) = lhs.overflowing_sub(rhs);
                let of = lhs.cast_signed().checked_sub(rhs.cast_signed()).is_none();
                (value, cf, of)
            }
            Adc => {
                let (t, cf1) = lhs.overflowing_add(rhs);
                let (value, cf2) = t.overflowing_add((flags & 2 != 0).into());
                let (t, of1) = lhs.cast_signed().overflowing_add(rhs.cast_signed());
                let (_, of2) = t.overflowing_add((flags & 2 != 0).into());
                (value, cf1 || cf2, of1 || of2)
            }
            Sbb => {
                let (t, cf1) = lhs.overflowing_sub(rhs);
                let (value, cf2) = t.overflowing_sub((flags & 2 != 0).into());
                let (t, of1) = lhs.cast_signed().overflowing_sub(rhs.cast_signed());
                let (_, of2) = t.overflowing_sub((flags & 2 != 0).into());
                (value, cf1 || cf2, of1 || of2)
            }
            And => (lhs & rhs, false, false),
            Xor => (lhs ^ rhs, false, false),
            Bic => (lhs & !rhs, false, false),
            Or => (lhs | rhs, false, false),
            Shl => {
                let shift = u32::from(rhs & 15);
                let cf = shift != 0 && lhs & (0x8000 >> (shift - 1)) != 0;
                (lhs.wrapping_shl(shift), cf, false)
            }
            Shr => {
                let shift = u32::from(rhs & 15);
                let cf = shift != 0 && lhs & (1 << (shift - 1)) != 0;
                (lhs.wrapping_shr(shift), cf, false)
            }
            /*Sal => {
                let shift = u32::from(rhs & 15);
                let cf = shift != 0 && lhs & (0x8000 >> (shift - 1)) != 0;
                (lhs.cast_signed().wrapping_shl(shift).cast_unsigned(), cf, false)
            },*/
            Sar => {
                let shift = u32::from(rhs & 15);
                let cf = shift != 0 && lhs & (1 << (shift - 1)) != 0;
                (
                    lhs.cast_signed().wrapping_shr(shift).cast_unsigned(),
                    cf,
                    false,
                )
            }
            Rol => {
                let shift = u32::from(rhs & 15);
                let cf = shift != 0 && lhs & (0x8000 >> (shift - 1)) != 0;
                (lhs.rotate_left(shift), cf, false)
            }
            Ror => {
                let shift = u32::from(rhs & 15);
                let cf = shift != 0 && lhs & (1 << (shift - 1)) != 0;
                (lhs.rotate_right(shift), cf, false)
            }
        };
        let sf = value.cast_signed() < 0;
        let zf = value == 0;
        self.set_regw(
            FL,
            (flags & !15)
                | u16::from(zf)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        value
    }

    /// Calculates a unary ALU operation on an 8-bit operand, updating the system's flags accordingly.
    pub fn alu_unaryb(&mut self, op: AluUnaryOperation, value: u8) -> u8 {
        use AluUnaryOperation::{Abs, Dec, Inc, Neg, Not, Popcnt, Rcl, Rcr, Sgxt, Swap, Zero};

        let flags = self.get_regw(FL);
        let (result, cf, of) = match op {
            Neg => {
                let (result, cf) = value.overflowing_neg();
                let of = value.cast_signed().checked_neg().is_none();
                (result, cf, of)
            }
            Not => (!value, false, false),
            Inc => {
                let result = value.wrapping_add(1);
                let of = value.cast_signed().checked_add(1).is_none();
                (result, flags & 2 != 0, of)
            }
            Dec => {
                let result = value.wrapping_sub(1);
                let of = value.cast_signed().checked_sub(1).is_none();
                (result, flags & 2 != 0, of)
            }
            Abs => {
                let result = value.cast_signed().abs().cast_unsigned();
                let of = value.cast_signed().checked_abs().is_none();
                (result, value.cast_signed() < 0, of)
            }
            Sgxt => (value, false, false),
            Swap => (value.rotate_right(4), false, false),
            Popcnt => (value.count_ones() as u8, false, false),
            Rcl => {
                let cf = value & 0x80 != 0;
                (value.wrapping_shl(1) | u8::from(flags & 2 != 0), cf, false)
            }
            Rcr => {
                let cf = value & 1 != 0;
                (
                    value.wrapping_shr(1) | (u8::from(flags & 2 != 0) << 7),
                    cf,
                    false,
                )
            }
            Zero => (value, value == u8::MAX, value == i8::MAX.cast_unsigned()),
        };
        let sf = result.cast_signed() < 0;
        let zf = result == 0;
        self.set_regw(
            FL,
            (flags & !15)
                | u16::from(zf)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        if op == Zero { 0 } else { result }
    }

    /// Calculates a unary ALU operation on a 16-bit operand, updating the system's flags accordingly.
    pub fn alu_unaryw(&mut self, op: AluUnaryOperation, value: u16) -> u16 {
        use AluUnaryOperation::{Abs, Dec, Inc, Neg, Not, Popcnt, Rcl, Rcr, Sgxt, Swap, Zero};

        let flags = self.get_regw(FL);
        let (result, cf, of) = match op {
            Neg => {
                let (result, cf) = value.overflowing_neg();
                let of = value.cast_signed().checked_neg().is_none();
                (result, cf, of)
            }
            Not => (!value, false, false),
            Inc => {
                let result = value.wrapping_add(1);
                let of = value.cast_signed().checked_add(1).is_none();
                (result, flags & 2 != 0, of)
            }
            Dec => {
                let result = value.wrapping_sub(1);
                let of = value.cast_signed().checked_sub(1).is_none();
                (result, flags & 2 != 0, of)
            }
            Abs => {
                let result = value.cast_signed().abs().cast_unsigned();
                let of = value.cast_signed().checked_abs().is_none();
                (result, value.cast_signed() < 0, of)
            }
            Sgxt => (
                i16::from((value as u8).cast_signed()).cast_unsigned(),
                false,
                false,
            ),
            Swap => (value.rotate_right(8), false, false),
            Popcnt => (value.count_ones() as u16, false, false),
            Rcl => {
                let cf = value & 0x8000 != 0;
                (value.wrapping_shl(1) | u16::from(flags & 2 != 0), cf, false)
            }
            Rcr => {
                let cf = value & 1 != 0;
                (
                    value.wrapping_shr(1) | (u16::from(flags & 2 != 0) << 15),
                    cf,
                    false,
                )
            }
            Zero => (value, value == u16::MAX, value == i16::MAX.cast_unsigned()),
        };
        let sf = result.cast_signed() < 0;
        let zf = result == 0;
        self.set_regw(
            FL,
            (flags & !15)
                | u16::from(zf)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        if op == Zero { 0 } else { result }
    }
}

/// The operation the system performed during a step.
pub enum SystemStep {
    /// The system ran an instruction.
    RanInstruction(Opcode),
    /// The system handled an interrupt.
    Interrupt(u16),
}

impl System {
    /// Performs a step of the CPU.
    /// 
    /// May execute the instruction at the current PC, returning the executed opcode, or execute an interrupt.
    ///
    /// # Errors
    ///
    /// Returns an error if the system is halted or if there is an issue fetching or executing the instruction.
    pub fn step(&mut self) -> Result<SystemStep, SystemError> {
        if self.is_halted() {
            return Err(SystemError::Halted);
        }

        if self.cycles() >= self.next_vblank {
            self.next_vblank += super::gpu::VBLANK_INTERVAL;
            self.interrupt(0x7FF0)?;
            return Ok(SystemStep::Interrupt(0x7FF0));
        }

        let mut bytes = Vec::with_capacity(4);
        for x in 0..4 {
            match self.get_direct_mem(self.get_regw(WordRegister::PC).wrapping_add(x)) {
                Ok(byte) => bytes.push(byte),
                Err(SystemError::ReadOutOfRomBounds) => break,
                Err(e) => return Err(e),
            }
        }
        let opcode = Opcode::from_slice(&bytes)?;

        self.set_regw(PC, self.get_regw(PC) + opcode.to_vec().len() as u16);

        self.run_instruction(opcode)?;

        Ok(SystemStep::RanInstruction(opcode))
    }

    /// Executes a given opcode in the system, updating the system's state accordingly.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue executing the instruction.
    #[allow(clippy::too_many_lines)]
    pub fn run_instruction(&mut self, opcode: Opcode) -> Result<(), SystemError> {
        use Opcode::{
            AlubRIB, AlubRIW, AlubRRB, AlubRRW, AluuRB, AluuRW, CallCcA, ClbRIB, ClbRIW, ClbRRB,
            ClbRRW, CpAlubRIB, CpAlubRIW, CpAlubRRB, CpAlubRRW, CpAluuRB, CpAluuRW, Halt, JmpCcA,
            JrCcX, LeaCcROB, LeaCcROW, MovARB, MovARW, MovCcORB, MovCcORW, MovCcROB, MovCcROW,
            MovCcRRB, MovCcRRW, MovRAB, MovRAW, MovRIB, MovRIW, Nop, PopRB, PopRW, PushRB, PushRW,
            Reti, StbRIB, StbRIW, StbRRB, StbRRW, TbitRIB, TbitRIW, TbitRRB, TbitRRW, TgbRIB,
            TgbRIW, TgbRRB, TgbRRW, XchCcRRB, XchCcRRW,
        };

        match opcode {
            MovRIB(dst, imm) => {
                self.set_regb(dst, imm);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            MovRIW(dst, imm) => {
                self.set_regw(dst, imm);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            MovCcRRB(cc, dst, src) => {
                if self.condition(cc) {
                    self.set_regb(dst, self.get_regb(src));
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            MovCcRRW(cc, dst, src) => {
                if self.condition(cc) {
                    self.set_regw(dst, self.get_regw(src));
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            XchCcRRB(cc, dst, src) => {
                if self.condition(cc) {
                    let a = self.get_regb(dst);
                    let b = self.get_regb(src);
                    self.set_regb(dst, b);
                    self.set_regb(src, a);
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            XchCcRRW(cc, dst, src) => {
                if self.condition(cc) {
                    let a = self.get_regw(dst);
                    let b = self.get_regw(src);
                    self.set_regw(dst, b);
                    self.set_regw(src, a);
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }

            MovRAB(dst, addr) => {
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                self.set_regb(dst, self.get_memb(addr)?);
            }
            MovRAW(dst, addr) => {
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                self.set_regw(dst, self.get_memw(addr)?);
            }

            MovCcROB(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regb(
                        dst,
                        self.get_memb(self.get_regw(base).wrapping_add_signed(offset.into()))?,
                    );
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            MovCcROW(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regw(
                        dst,
                        self.get_memw(self.get_regw(base).wrapping_add_signed(offset.into()))?,
                    );
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            LeaCcROB(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regb(
                        dst,
                        self.get_regw(base).wrapping_add_signed(offset.into()) as u8,
                    );
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            LeaCcROW(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regw(dst, self.get_regw(base).wrapping_add_signed(offset.into()));
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }

            JmpCcA(cc, addr) => {
                if self.condition(cc) {
                    self.set_regw(PC, addr);
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            CallCcA(cc, addr) => {
                if self.condition(cc) {
                    let sp = self.get_regw(SP).wrapping_sub(2);
                    self.set_memw(sp, self.get_regw(PC))?;
                    self.set_regw(SP, sp);
                    self.set_regw(PC, addr);
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }

            MovARB(addr, src) => {
                self.set_memb(addr, self.get_regb(src))?;
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            MovARW(addr, src) => {
                self.set_memw(addr, self.get_regw(src))?;
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            MovCcORB(cc, offset, base, src) => {
                if self.condition(cc) {
                    self.set_memb(
                        self.get_regw(base).wrapping_add_signed(offset.into()),
                        self.get_regb(src),
                    )?;
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }
            MovCcORW(cc, offset, base, src) => {
                if self.condition(cc) {
                    self.set_memw(
                        self.get_regw(base).wrapping_add_signed(offset.into()),
                        self.get_regw(src),
                    )?;
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }

            AlubRRB(op, dst, src) => {
                let value = self.alu_binaryb(op, self.get_regb(dst), self.get_regb(src));
                self.set_regb(dst, value);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            AlubRRW(op, dst, src) => {
                let value = self.alu_binaryw(op, self.get_regw(dst), self.get_regw(src));
                self.set_regw(dst, value);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            AlubRIB(op, dst, imm) => {
                let value = self.alu_binaryb(op, self.get_regb(dst), imm);
                self.set_regb(dst, value);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            AlubRIW(op, dst, imm) => {
                let value = self.alu_binaryw(op, self.get_regw(dst), imm);
                self.set_regw(dst, value);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            AluuRB(op, dst) => {
                let value = self.alu_unaryb(op, self.get_regb(dst));
                self.set_regb(dst, value);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            AluuRW(op, dst) => {
                let value = self.alu_unaryw(op, self.get_regw(dst));
                self.set_regw(dst, value);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            CpAlubRRB(op, dst, src) => {
                let _ = self.alu_binaryb(op, self.get_regb(dst), self.get_regb(src));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            CpAlubRRW(op, dst, src) => {
                let _ = self.alu_binaryw(op, self.get_regw(dst), self.get_regw(src));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            CpAlubRIB(op, dst, imm) => {
                let _ = self.alu_binaryb(op, self.get_regb(dst), imm);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            CpAlubRIW(op, dst, imm) => {
                let _ = self.alu_binaryw(op, self.get_regw(dst), imm);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            CpAluuRB(op, dst) => {
                let _ = self.alu_unaryb(op, self.get_regb(dst));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            CpAluuRW(op, dst) => {
                let _ = self.alu_unaryw(op, self.get_regw(dst));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            JrCcX(cc, offset) => {
                if self.condition(cc) {
                    self.set_regw(PC, self.get_regw(PC).wrapping_add_signed(offset.into()));
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 2);
                } else {
                    self.cycles
                        .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
                }
            }

            PushRB(src) => {
                let sp = self.get_regw(SP).wrapping_sub(1);
                self.set_memb(sp, self.get_regb(src))?;
                self.set_regw(SP, sp);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            PushRW(src) => {
                let sp = self.get_regw(SP).wrapping_sub(2);
                self.set_memw(sp, self.get_regw(src))?;
                self.set_regw(SP, sp);
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            PopRB(dst) => {
                let sp = self.get_regw(SP);
                if sp >= SP_START {
                    self.halt();
                } else {
                    self.set_regb(dst, self.get_memb(sp)?);
                    self.set_regw(SP, sp.wrapping_add(1));
                }
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            PopRW(dst) => {
                let sp = self.get_regw(SP);
                if sp >= SP_START {
                    self.halt();
                } else {
                    self.set_regw(dst, self.get_memw(sp)?);
                    self.set_regw(SP, sp.wrapping_add(2));
                }
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            ClbRIB(dst, bit) => {
                self.set_regb(dst, self.get_regb(dst) & !(1 << (bit & 7)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            ClbRIW(dst, bit) => {
                self.set_regw(dst, self.get_regw(dst) & !(1 << (bit & 15)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            ClbRRB(dst, bitreg) => {
                self.set_regb(
                    dst,
                    self.get_regb(dst) & !(1 << (self.get_regb(bitreg) & 7)),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            ClbRRW(dst, bitreg) => {
                self.set_regw(
                    dst,
                    self.get_regw(dst) & !(1 << (self.get_regb(bitreg) & 15)),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            StbRIB(dst, bit) => {
                self.set_regb(dst, self.get_regb(dst) | (1 << (bit & 7)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            StbRIW(dst, bit) => {
                self.set_regw(dst, self.get_regw(dst) | (1 << (bit & 15)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            StbRRB(dst, bitreg) => {
                self.set_regb(dst, self.get_regb(dst) | (1 << (self.get_regb(bitreg) & 7)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            StbRRW(dst, bitreg) => {
                self.set_regw(
                    dst,
                    self.get_regw(dst) | (1 << (self.get_regb(bitreg) & 15)),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            TgbRIB(dst, bit) => {
                self.set_regb(dst, self.get_regb(dst) ^ (1 << (bit & 7)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            TgbRIW(dst, bit) => {
                self.set_regw(dst, self.get_regw(dst) ^ (1 << (bit & 15)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            TgbRRB(dst, bitreg) => {
                self.set_regb(dst, self.get_regb(dst) ^ (1 << (self.get_regb(bitreg) & 7)));
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            TgbRRW(dst, bitreg) => {
                self.set_regw(
                    dst,
                    self.get_regw(dst) ^ (1 << (self.get_regb(bitreg) & 15)),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            TbitRIB(src, bit) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regb(src) & (1 << (bit & 7)) == 0),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            TbitRIW(src, bit) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regw(src) & (1 << (bit & 15)) == 0),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            TbitRRB(src, bitreg) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regb(src) & (1 << (self.get_regb(bitreg) & 7)) == 0),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            TbitRRW(src, bitreg) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regw(src) & (1 << (self.get_regb(bitreg) & 15)) == 0),
                );
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }

            Nop => {
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64);
            }
            Reti => {
                let sp = self.get_regw(SP);
                if sp >= SP_START {
                    self.halt();
                } else {
                    self.set_regw(PC, self.get_memw(sp)?);
                    self.set_regw(SP, sp.wrapping_add(2));
                }
                self.active_interrupt = None;
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
            Halt => {
                self.halt();
                self.cycles
                    .update(|cycles| cycles + opcode.to_vec().len() as u64 + 1);
            }
        }
        Ok(())
    }
}
