use std::io::{Read, Write, stdin, stdout};

use crate::{bytecode::bytecode::Bytecode, int::{InterpretResult, program::UnsafeProgram, tape::{OutOfRangeError, Tape}}};

pub fn run_deopt<const FLUSH: bool, const USE_OPT: bool>(program: &mut UnsafeProgram, tape: &mut Tape) -> Result<InterpretResult, OutOfRangeError> {
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    macro_rules! rangecheck {
        ($range: expr) => {
            if USE_OPT && $range.contains(tape.get_offset()) {
                return Ok(InterpretResult::ToggleOpt(true));
            }
        };
    }

    loop {
        match program.get_op() {
            Bytecode::SetC(p1, value) => {
                *tape.get_mut(*p1)? = *value;
            }
            Bytecode::SetL(p1, p2) => {
                let v = tape.get(*p2)?;
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::Add(ptr, val) => {
                let v = tape.get(*ptr)?.wrapping_add(*val);
                *tape.get_mut(*ptr)? = v;
            }
            Bytecode::AddC(p1, p2, value) => {
                let v = tape.get(*p2)?.wrapping_add(*value);
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::AddL(p1, p2, p3) => {
                let v = tape.get(*p2)?.wrapping_add(tape.get(*p3)?);
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::SubLC(p1, p2, value) => {
                let v = tape.get(*p2)?.wrapping_sub(*value);
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::SubCL(p1, value, p3) => {
                let v = value.wrapping_sub(tape.get(*p3)?);
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::SubLL(p1, p2, p3) => {
                let v = tape.get(*p2)?.wrapping_sub(tape.get(*p3)?);
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::MulC(p1, p2, value) => {
                let v = tape.get(*p2)?.wrapping_mul(*value);
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::MulL(p1, p2, p3) => {
                let v = tape.get(*p2)?.wrapping_mul(tape.get(*p3)?);
                *tape.get_mut(*p1)? = v;
            }

            Bytecode::MulAddC(p1, value, p3, factor) => {
                let v = value.wrapping_add(tape.get(*p3)?.wrapping_mul(*factor));
                *tape.get_mut(*p1)? = v;
            }
            Bytecode::MulAddL(p1, p2, p3, factor) => {
                let v = tape.get(*p2)?.wrapping_add(tape.get(*p3)?.wrapping_mul(*factor));
                *tape.get_mut(*p1)? = v;
            }

            Bytecode::In(p1) => {
                let mut buf = [0u8; 1];
                *tape.get_mut(*p1)? = if stdin.read_exact(&mut buf).is_ok() {
                    buf[0]
                } else {
                    0
                };
            }
            
            Bytecode::Out(p1) => {
                let _ = stdout.write(&[tape.get(*p1)?]);
                if FLUSH {
                    let _ = stdout.flush();
                }
            }
            Bytecode::OutConst(v1) => {
                let _ = stdout.write(&[*v1]);
                if FLUSH {
                    let _ = stdout.flush();
                }
            }
            Bytecode::Jump(a1) => {
                program.jump_relative(*a1);
                continue;
            }
            Bytecode::JumpIfZero(p1, a2) => {
                if tape.get(*p1)? == 0 {
                    program.jump_relative(*a2);
                    continue;
                }
            }
            Bytecode::JumpIfNotZero(p1, a2) => {
                if tape.get(*p1)? != 0 {
                    program.jump_relative(*a2);
                    continue;
                }
            }
            Bytecode::Offset(o1) => {
                tape.offset(*o1);
            }
            Bytecode::OffsetWithRangeCheck(o1, range) => {
                tape.offset(*o1);
                rangecheck!(range);
            }
            Bytecode::RangeCheck(range) => {
                rangecheck!(range);
            }
            Bytecode::FindZero(ptr, delta) => {
                while tape.get(*ptr)? != 0 {
                    tape.offset(*delta);
                }
            }
            Bytecode::End => {
                return Ok(InterpretResult::End);
            }
            
            Bytecode::SetCSetC(p1, c1, p2, c2) => {
                *tape.get_mut(*p1)? = *c1;
                *tape.get_mut(*p2)? = *c2;
            }
            Bytecode::AddAdd(p1, c1, p2, c2) => {
                let v = tape.get(*p1)?.wrapping_add(*c1);
                *tape.get_mut(*p1)? = v;

                let v = tape.get(*p2)?.wrapping_add(*c2);
                *tape.get_mut(*p2)? = v;
            }
            Bytecode::AddSetC(p1, c1, p2, c2) => {
                let v = tape.get(*p1)?.wrapping_add(*c1);
                *tape.get_mut(*p1)? = v;

                *tape.get_mut(*p2)? = *c2;
            }
        }

        program.next();
    }
}
