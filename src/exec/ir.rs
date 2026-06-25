use std::io::{Read, Write, stdin, stdout};

use crate::{
    bytecode::bytecode::Bytecode,
    exec::{
        tape::{OutOfRangeError, Tape},
        thread_poll::BytecodeComputePoller,
    },
    ir::ir::{IR, IROp},
};

pub fn exec_ir_with_poll<const FLUSH: bool>(
    ir: &[IR],
    tape: &mut Tape,
    poller: &mut BytecodeComputePoller,
) -> Result<Option<(Vec<Bytecode>, usize)>, OutOfRangeError> {
    let mut pc = 0;
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    loop {
        let IR {
            pointer,
            opcode,
            loc: _,
        } = match ir.get(pc) {
            Some(ir) => ir,
            None => return Ok(None),
        };

        match opcode {
            IROp::Add(value) => {
                let current = tape.get(*pointer)?;
                *tape.get_mut(*pointer)? = current.wrapping_add(*value);
            }
            IROp::Set(value) => {
                *tape.get_mut(*pointer)? = *value;
            }
            IROp::MulAdd(p2, val) => {
                let current = tape.get(*pointer)?;
                let p2_val = tape.get(*p2)?;
                *tape.get_mut(*pointer)? = current.wrapping_add(p2_val.wrapping_mul(*val));
            }
            IROp::In => {
                let mut buf = [0u8; 1];
                *tape.get_mut(*pointer)? = if stdin.read_exact(&mut buf).is_ok() {
                    buf[0]
                } else {
                    0
                };
            }
            IROp::Out => {
                let _ = stdout.write(&[tape.get(*pointer)?]);
                if FLUSH {
                    let _ = stdout.flush();
                }
            }
            IROp::JumpZero(addr) => {
                if let Some(p_ret) = poller.poll(pc) {
                    return Ok(Some(p_ret));
                }
                if tape.get(*pointer)? == 0 {
                    pc = *addr as usize;
                    continue;
                }
            }
            IROp::JumpNotZero(addr) => {
                if let Some(p_ret) = poller.poll(pc) {
                    return Ok(Some(p_ret));
                }
                if tape.get(*pointer)? != 0 {
                    pc = *addr as usize;
                    continue;
                }
            }
            IROp::JumpNotZeroWithOffset(step, addr) => {
                if let Some(p_ret) = poller.poll(pc) {
                    return Ok(Some(p_ret));
                }
                tape.offset(*step);
                if tape.get(*pointer)? != 0 {
                    pc = *addr as usize;
                    continue;
                }
            }
            IROp::FindZero(delta) => loop {
                if tape.get(*pointer)? == 0 {
                    break;
                }
                tape.offset(*delta);
            },
        }
        pc += 1;
    }
}
