use std::io::{Read, Write, stdin, stdout};

use crate::{TAPE_LENGTH, ir::ir::{IR, IROp}};

pub fn exec_from_ir(ir: &[IR], offset: u8) {
    let mut pc = 0;
    let mut offset = offset as i16;
    let mut memory = [0u8; TAPE_LENGTH];
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    loop {
        let IR {
            pointer,
            opcode,
            loc: _,
        } = match ir.get(pc) {
            Some(ir) => ir,
            None => return,
        };
        let p = (pointer + offset) as usize;
        match opcode {
            IROp::Add(value) => {
                memory[p] = memory[p].wrapping_add(*value);
            }
            IROp::Set(value) => {
                memory[p] = *value;
            }
            IROp::MulAdd(p2, val) => {
                memory[p] =
                    memory[p].wrapping_add(memory[(offset + p2) as usize].wrapping_mul(*val));
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
                if memory[p] == 0 {
                    pc = *addr;
                    continue;
                }
            }
            IROp::JumpNotZero(addr) => {
                if memory[p] != 0 {
                    pc = *addr;
                    continue;
                }
            }
            IROp::JumpNotZeroWithOffset(step, addr) => {
                offset += step;
                if memory[(pointer + offset) as usize] != 0 {
                    pc = *addr;
                    continue;
                }
            }
        }
        pc += 1;
    }
}
