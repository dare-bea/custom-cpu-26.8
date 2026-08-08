use std::error::Error;

use crate::opcode::{AluBinaryOperation, AluUnaryOperation, ConditionCode, Opcode};
use crate::register::WordRegister;
use crate::register::WordRegister::{FL, PC, SP};
use crate::system::{SP_START, System, SystemError};

impl System {
    #[inline]
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

    pub fn alu_binaryb(&mut self, op: AluBinaryOperation, lhs: u8, rhs: u8) -> u8 {
        use AluBinaryOperation::*;
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
                | (u16::from(zf) << 0)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        value
    }

    pub fn alu_binaryw(&mut self, op: AluBinaryOperation, lhs: u16, rhs: u16) -> u16 {
        use AluBinaryOperation::*;
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
                | (u16::from(zf) << 0)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        value
    }

    pub fn alu_unaryb(&mut self, op: AluUnaryOperation, value: u8) -> u8 {
        use AluUnaryOperation::*;
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
                | (u16::from(zf) << 0)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        if op == Zero { 0 } else { result }
    }

    pub fn alu_unaryw(&mut self, op: AluUnaryOperation, value: u16) -> u16 {
        use AluUnaryOperation::*;
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
                | (u16::from(zf) << 0)
                | (u16::from(cf) << 1)
                | (u16::from(sf) << 2)
                | (u16::from(of) << 3),
        );
        if op == Zero { 0 } else { result }
    }
}

impl System {
    pub fn step(&mut self) -> Result<Opcode, Box<dyn Error>> {
        if self.is_halted() {
            return Err(SystemError::Halted.into());
        }

        let mut bytes = Vec::with_capacity(4);
        for x in 0..4 {
            match self.get_memb(self.get_regw(WordRegister::PC).wrapping_add(x)) {
                Ok(byte) => bytes.push(byte),
                Err(e) => {
                    if let Some(SystemError::ReadOutOfRomBounds) = e.downcast_ref() {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        let opcode = Opcode::from_slice(&bytes)?;

        self.set_regw(PC, self.get_regw(PC) + opcode.to_vec().len() as u16);

        self.run_instruction(opcode)?;

        Ok(opcode)
    }

    pub fn run_instruction(&mut self, opcode: Opcode) -> Result<(), Box<dyn Error>> {
        use Opcode::*;

        match opcode {
            MovRIB(dst, imm) => {
                self.set_regb(dst, imm);
            }
            MovRIW(dst, imm) => {
                self.set_regw(dst, imm);
            }

            MovCcRRB(cc, dst, src) => {
                if self.condition(cc) {
                    self.set_regb(dst, self.get_regb(src))
                };
            }
            MovCcRRW(cc, dst, src) => {
                if self.condition(cc) {
                    self.set_regw(dst, self.get_regw(src))
                };
            }
            XchCcRRB(cc, dst, src) => {
                if self.condition(cc) {
                    let a = self.get_regb(dst);
                    let b = self.get_regb(src);
                    self.set_regb(dst, b);
                    self.set_regb(src, a);
                }
            }
            XchCcRRW(cc, dst, src) => {
                if self.condition(cc) {
                    let a = self.get_regw(dst);
                    let b = self.get_regw(src);
                    self.set_regw(dst, b);
                    self.set_regw(src, a);
                }
            }

            MovRAB(dst, addr) => {
                self.set_regb(dst, self.get_memb(addr)?);
            }
            MovRAW(dst, addr) => {
                self.set_regw(dst, self.get_memw(addr)?);
            }

            MovCcROB(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regb(
                        dst,
                        self.get_memb(self.get_regw(base).wrapping_add_signed(offset.into()))?,
                    )
                };
            }
            MovCcROW(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regw(
                        dst,
                        self.get_memw(self.get_regw(base).wrapping_add_signed(offset.into()))?,
                    )
                };
            }
            LeaCcROB(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regb(
                        dst,
                        self.get_regw(base).wrapping_add_signed(offset.into()) as u8,
                    )
                };
            }
            LeaCcROW(cc, dst, offset, base) => {
                if self.condition(cc) {
                    self.set_regw(dst, self.get_regw(base).wrapping_add_signed(offset.into()))
                };
            }

            JmpCcA(cc, addr) => {
                if self.condition(cc) {
                    self.set_regw(PC, addr)
                };
            }
            CallCcA(cc, addr) => {
                if self.condition(cc) {
                    let sp = self.get_regw(SP).wrapping_sub(2);
                    self.set_memw(sp, self.get_regw(PC))?;
                    self.set_regw(SP, sp);
                    self.set_regw(PC, addr);
                };
            }

            MovARB(addr, src) => {
                self.set_memb(addr, self.get_regb(src))?;
            }
            MovARW(addr, src) => {
                self.set_memw(addr, self.get_regw(src))?;
            }

            MovCcORB(cc, offset, base, src) => {
                if self.condition(cc) {
                    self.set_memb(
                        self.get_regw(base).wrapping_add_signed(offset.into()),
                        self.get_regb(src),
                    )?
                };
            }
            MovCcORW(cc, offset, base, src) => {
                if self.condition(cc) {
                    self.set_memw(
                        self.get_regw(base).wrapping_add_signed(offset.into()),
                        self.get_regw(src),
                    )?
                };
            }

            AlubRRB(op, dst, src) => {
                let value = self.alu_binaryb(op, self.get_regb(dst), self.get_regb(src));
                self.set_regb(dst, value);
            }
            AlubRRW(op, dst, src) => {
                let value = self.alu_binaryw(op, self.get_regw(dst), self.get_regw(src));
                self.set_regw(dst, value);
            }
            AlubRIB(op, dst, imm) => {
                let value = self.alu_binaryb(op, self.get_regb(dst), imm);
                self.set_regb(dst, value);
            }
            AlubRIW(op, dst, imm) => {
                let value = self.alu_binaryw(op, self.get_regw(dst), imm);
                self.set_regw(dst, value);
            }
            AluuRB(op, dst) => {
                let value = self.alu_unaryb(op, self.get_regb(dst));
                self.set_regb(dst, value);
            }
            AluuRW(op, dst) => {
                let value = self.alu_unaryw(op, self.get_regw(dst));
                self.set_regw(dst, value);
            }

            CpAlubRRB(op, dst, src) => {
                let _ = self.alu_binaryb(op, self.get_regb(dst), self.get_regb(src));
            }
            CpAlubRRW(op, dst, src) => {
                let _ = self.alu_binaryw(op, self.get_regw(dst), self.get_regw(src));
            }
            CpAlubRIB(op, dst, imm) => {
                let _ = self.alu_binaryb(op, self.get_regb(dst), imm);
            }
            CpAlubRIW(op, dst, imm) => {
                let _ = self.alu_binaryw(op, self.get_regw(dst), imm);
            }
            CpAluuRB(op, dst) => {
                let _ = self.alu_unaryb(op, self.get_regb(dst));
            }
            CpAluuRW(op, dst) => {
                let _ = self.alu_unaryw(op, self.get_regw(dst));
            }

            JrCcX(cc, offset) => {
                if self.condition(cc) {
                    self.set_regw(PC, self.get_regw(PC).wrapping_add_signed(offset.into()));
                }
            }

            PushRB(src) => {
                let sp = self.get_regw(SP).wrapping_sub(1);
                self.set_memb(sp, self.get_regb(src))?;
                self.set_regw(SP, sp);
            }
            PushRW(src) => {
                let sp = self.get_regw(SP).wrapping_sub(2);
                self.set_memw(sp, self.get_regw(src))?;
                self.set_regw(SP, sp);
            }
            PopRB(dst) => {
                let sp = self.get_regw(SP);
                if sp >= SP_START {
                    self.halt();
                } else {
                    self.set_regb(dst, self.get_memb(sp)?);
                    self.set_regw(SP, sp.wrapping_add(1));
                }
            }
            PopRW(dst) => {
                let sp = self.get_regw(SP);
                if sp >= SP_START {
                    self.halt();
                } else {
                    self.set_regw(dst, self.get_memw(sp)?);
                    self.set_regw(SP, sp.wrapping_add(2));
                }
            }

            ClbRIB(dst, bit) => self.set_regb(dst, self.get_regb(dst) & !(1 << (bit & 7))),
            ClbRIW(dst, bit) => self.set_regw(dst, self.get_regw(dst) & !(1 << (bit & 15))),
            ClbRRB(dst, bitreg) => self.set_regb(
                dst,
                self.get_regb(dst) & !(1 << (self.get_regb(bitreg) & 7)),
            ),
            ClbRRW(dst, bitreg) => self.set_regw(
                dst,
                self.get_regw(dst) & !(1 << (self.get_regb(bitreg) & 15)),
            ),

            StbRIB(dst, bit) => self.set_regb(dst, self.get_regb(dst) | (1 << (bit & 7))),
            StbRIW(dst, bit) => self.set_regw(dst, self.get_regw(dst) | (1 << (bit & 15))),
            StbRRB(dst, bitreg) => {
                self.set_regb(dst, self.get_regb(dst) | (1 << (self.get_regb(bitreg) & 7)))
            }
            StbRRW(dst, bitreg) => self.set_regw(
                dst,
                self.get_regw(dst) | (1 << (self.get_regb(bitreg) & 15)),
            ),

            TgbRIB(dst, bit) => self.set_regb(dst, self.get_regb(dst) ^ (1 << (bit & 7))),
            TgbRIW(dst, bit) => self.set_regw(dst, self.get_regw(dst) ^ (1 << (bit & 15))),
            TgbRRB(dst, bitreg) => {
                self.set_regb(dst, self.get_regb(dst) ^ (1 << (self.get_regb(bitreg) & 7)))
            }
            TgbRRW(dst, bitreg) => self.set_regw(
                dst,
                self.get_regw(dst) ^ (1 << (self.get_regb(bitreg) & 15)),
            ),

            TbitRIB(src, bit) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regb(src) & (1 << (bit & 7)) == 0),
                );
            }
            TbitRIW(src, bit) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regw(src) & (1 << (bit & 15)) == 0),
                );
            }
            TbitRRB(src, bitreg) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regb(src) & (1 << (self.get_regb(bitreg) & 7)) == 0),
                );
            }
            TbitRRW(src, bitreg) => {
                self.set_regw(
                    FL,
                    (self.get_regw(FL) & !1)
                        | u16::from(self.get_regw(src) & (1 << (self.get_regb(bitreg) & 15)) == 0),
                );
            }

            Nop => {}
            Halt => {
                self.halt();
            }
        };
        Ok(())
    }
}
