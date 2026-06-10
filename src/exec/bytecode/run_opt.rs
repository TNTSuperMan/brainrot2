use std::io::{Read, Write, stdin, stdout};

use crate::{bytecode::bytecode::Bytecode, exec::{bytecode::{InterpretResult, program::UnsafeProgram}, tape::{OutOfRangeError, UnsafeTape}}, timeline};

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn run_opt<const FLUSH: bool>(program: &mut UnsafeProgram, tape: &mut UnsafeTape) -> Result<InterpretResult, OutOfRangeError> {
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    macro_rules! rangecheck {
        ($offset: expr, $range: expr) => {
            if !$range.contains($offset) {
                timeline!("deopt");
                return Ok(InterpretResult::ToggleOpt(false));
            }
        };
    }

    loop {
        match program.get_op() {
            Bytecode::SetC(p1, value) => {
                *tape.get_mut(*p1) = *value;
            }
            Bytecode::SetL(p1, p2) => {
                let v = tape.get(*p2);
                *tape.get_mut(*p1) = v;
            }
            Bytecode::Add(ptr, val) => {
                let v = tape.get(*ptr).wrapping_add(*val);
                *tape.get_mut(*ptr) = v;
            }
            Bytecode::AddC(p1, p2, value) => {
                let v = tape.get(*p2).wrapping_add(*value);
                *tape.get_mut(*p1) = v;
            }
            Bytecode::AddL(p1, p2, p3) => {
                let v = tape.get(*p2).wrapping_add(tape.get(*p3));
                *tape.get_mut(*p1) = v;
            }
            Bytecode::AddLA(p1, p2) => {
                let v = tape.get(*p1).wrapping_add(tape.get(*p2));
                *tape.get_mut(*p1) = v;
            }
            Bytecode::SubLC(p1, p2, value) => {
                let v = tape.get(*p2).wrapping_sub(*value);
                *tape.get_mut(*p1) = v;
            }
            Bytecode::SubCL(p1, value, p3) => {
                let v = value.wrapping_sub(tape.get(*p3));
                *tape.get_mut(*p1) = v;
            }
            Bytecode::SubLL(p1, p2, p3) => {
                let v = tape.get(*p2).wrapping_sub(tape.get(*p3));
                *tape.get_mut(*p1) = v;
            }
            Bytecode::MulC(p1, p2, value) => {
                let v = tape.get(*p2).wrapping_mul(*value);
                *tape.get_mut(*p1) = v;
            }
            Bytecode::MulL(p1, p2, p3) => {
                let v = tape.get(*p2).wrapping_mul(tape.get(*p3));
                *tape.get_mut(*p1) = v;
            }

            Bytecode::MulAddC(p1, value, p3, factor) => {
                let v = value.wrapping_add(tape.get(*p3).wrapping_mul(*factor));
                *tape.get_mut(*p1) = v;
            }
            Bytecode::MulAddL(p1, p2, p3, factor) => {
                let v = tape.get(*p2).wrapping_add(tape.get(*p3).wrapping_mul(*factor));
                *tape.get_mut(*p1) = v;
            }

            Bytecode::In(p1) => {
                let mut buf = [0u8; 1];
                *tape.get_mut(*p1) = if stdin.read_exact(&mut buf).is_ok() {
                    buf[0]
                } else {
                    0
                };
            }
            
            Bytecode::Out(p1) => {
                let _ = stdout.write(&[tape.get(*p1)]);
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
                if tape.get(*p1) == 0 {
                    program.jump_relative(*a2);
                    continue;
                }
            }
            Bytecode::JumpIfNotZero(p1, a2) => {
                if tape.get(*p1) != 0 {
                    program.jump_relative(*a2);
                    continue;
                }
            }
            Bytecode::Offset(o1) => {
                tape.offset(*o1);
            }
            Bytecode::OffsetWithRangeCheck(o1, range) => {
                rangecheck!(tape.get_offset() + *o1 as i32, range);
                tape.offset(*o1);
            }
            Bytecode::RangeCheck(range) => {
                rangecheck!(tape.get_offset(), range);
            }
            Bytecode::FindZero(ptr, delta) => {
                while tape.get_safe(*ptr)? != 0 {
                    tape.offset(*delta);
                }
            }
            Bytecode::End => {
                return Ok(InterpretResult::End);
            }
            
            Bytecode::SetCSetC(p1, c1, p2, c2) => {
                *tape.get_mut(*p1) = *c1;
                *tape.get_mut(*p2) = *c2;
            }
            Bytecode::AddAdd(p1, c1, p2, c2) => {
                let v = tape.get(*p1).wrapping_add(*c1);
                *tape.get_mut(*p1) = v;

                let v = tape.get(*p2).wrapping_add(*c2);
                *tape.get_mut(*p2) = v;
            }
            Bytecode::AddSetC(p1, c1, p2, c2) => {
                let v = tape.get(*p1).wrapping_add(*c1);
                *tape.get_mut(*p1) = v;

                *tape.get_mut(*p2) = *c2;
            }
            Bytecode::MulAddMulAdd { src, dst1, dst2_rel, val1, val2 } => {
                let v = tape.get(*src);
                let v1 = tape.get(*dst1).wrapping_add(v.wrapping_mul(*val1));
                *tape.get_mut(*dst1) = v1;
                
                let dst2 = src.wrapping_add(*dst2_rel as i16);
                let v2 = tape.get(dst2).wrapping_add(v.wrapping_mul(*val2));
                *tape.get_mut(dst2) = v2;
            }
            Bytecode::OutOut(p1, p2) => {
                let _ = stdout.write(&[tape.get(*p1), tape.get(*p2)]);
                if FLUSH {
                    let _ = stdout.flush();
                }
            }
        }

        program.next();
    }
}
