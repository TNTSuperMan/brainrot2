use std::io::{Read, Write, stdin, stdout};

use crate::{TAPE_LENGTH, bytecode::bytecode::Bytecode, exec::thread_poll::BytecodeComputePoller, ir::ir::{IR, IROp}};

pub fn exec_ir_with_poll(ir: &[IR], memory: &mut [u8; TAPE_LENGTH], offset: &mut i16, poller: &mut BytecodeComputePoller) -> Option<(Vec<Bytecode>, usize)> {
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
            None => return None,
        };
        let p = (pointer + *offset) as usize;
        match opcode {
            IROp::Add(value) => {
                memory[p] = memory[p].wrapping_add(*value);
            }
            IROp::Set(value) => {
                memory[p] = *value;
            }
            IROp::MulAdd(p2, val) => {
                memory[p] =
                    memory[p].wrapping_add(memory[(*offset + p2) as usize].wrapping_mul(*val));
            }
            IROp::In => {
                let mut buf = [0u8; 1];
                memory[p] = if stdin.read_exact(&mut buf).is_ok() {
                    buf[0]
                } else {
                    0
                };
            }
            IROp::Out => {
                stdout.write(&[memory[p]]).unwrap();
            }
            IROp::JumpZero(addr) => {
                if let Some(p) = poller.poll(pc) { return Some(p) }
                if memory[p] == 0 {
                    pc = *addr;
                    continue;
                }
            }
            IROp::JumpNotZero(addr) => {
                if let Some(p) = poller.poll(pc) { return Some(p) }
                if memory[p] != 0 {
                    pc = *addr;
                    continue;
                }
            }
            IROp::JumpNotZeroWithOffset(step, addr) => {
                if let Some(p) = poller.poll(pc) { return Some(p) }
                *offset += step;
                if memory[(pointer + *offset) as usize] != 0 {
                    pc = *addr;
                    continue;
                }
            }
        }
        pc += 1;
    }
}
