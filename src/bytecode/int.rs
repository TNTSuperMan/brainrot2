use std::{io::{Read, stdin}, ops::{Index, IndexMut}};

use crate::{TAPE_LENGTH, bytecode::bytecode::Bytecode};

struct Mem {
    offset: isize,
    memory: [u8; TAPE_LENGTH],
}
impl Index<&i16> for Mem {
    type Output = u8;
    fn index(&self, index: &i16) -> &Self::Output {
        &self.memory[(self.offset + *index as isize) as usize]
    }
}
impl IndexMut<&i16> for Mem {
    fn index_mut(&mut self, index: &i16) -> &mut Self::Output {
        &mut self.memory[(self.offset + *index as isize) as usize]
    }
}

pub fn debug_exec_bytecode(bytecodes: &[Bytecode], offset: u8, opt_first: bool) -> (Vec<u8>, Vec<u32>) {
    let mut exec_counts = vec![0; bytecodes.len()];
    let mut stdout = vec![];
    let mut pc: usize = 0;
    let mut mem = Mem {
        offset: offset as isize,
        memory: [0u8; TAPE_LENGTH],
    };
    let mut stdin = stdin().lock();
    let mut opt = opt_first;

    loop {
        exec_counts[pc] += 1;
        match &bytecodes[pc] {
            Bytecode::SetC(p1, value) => {
                mem[p1] = *value;
            }
            Bytecode::SetL(p1, p2) => {
                mem[p1] = mem[p2];
            }
            Bytecode::AddC(p1, p2, value) => {
                mem[p1] = mem[p2].wrapping_add(*value);
            }
            Bytecode::AddL(p1, p2, p3) => {
                mem[p1] = mem[p2].wrapping_add(mem[p3]);
            }
            Bytecode::SubLC(p1, p2, value) => {
                mem[p1] = mem[p2].wrapping_sub(*value);
            }
            Bytecode::SubCL(p1, value, p3) => {
                mem[p1] = value.wrapping_sub(mem[p3]);
            }
            Bytecode::SubLL(p1, p2, p3) => {
                mem[p1] = mem[p2].wrapping_sub(mem[p3]);
            }
            Bytecode::MulC(p1, p2, value) => {
                mem[p1] = mem[p2].wrapping_mul(*value);
            }
            Bytecode::MulL(p1, p2, p3) => {
                mem[p1] = mem[p2].wrapping_mul(mem[p3]);
            }

            Bytecode::MulAddC(p1, value, p3, factor) => {
                mem[p1] = value.wrapping_add(mem[p3].wrapping_mul(*factor));
            }
            Bytecode::MulAddL(p1, p2, p3, factor) => {
                mem[p1] = mem[p2].wrapping_add(mem[p3].wrapping_mul(*factor));
            }

            Bytecode::In(p1) => {
                let mut buf = [0u8; 1];
                mem[p1] = if stdin.read_exact(&mut buf).is_ok() {
                    buf[0]
                } else {
                    0
                };
            }
            
            Bytecode::Breakpoint(p1) => {
                let p = (*p1 as isize + mem.offset) as usize;
                println!("break; {}", p);
            }
            Bytecode::Out(p1) => {
                stdout.push(mem[p1]);
            }
            Bytecode::OutConst(v1) => {
                stdout.push(*v1);
            }
            Bytecode::Jump(a1) => {
                pc = pc.wrapping_add_signed(*a1 as isize);
                continue;
            }
            Bytecode::JumpIfZero(p1, a2) => {
                if mem[p1] == 0 {
                    pc = pc.wrapping_add_signed(*a2 as isize);
                    continue;
                }
            }
            Bytecode::JumpIfNotZero(p1, a2) => {
                if mem[p1] != 0 {
                    pc = pc.wrapping_add_signed(*a2 as isize);
                    continue;
                }
            }
            Bytecode::Offset(o1) => {
                mem.offset += *o1 as isize;
            }
            Bytecode::OffsetWithRangeCheck(o1, rb, re) => {
                mem.offset += *o1 as isize;
                if opt && (mem.offset < (*rb as isize) || (*re as isize) < mem.offset) {
                    eprintln!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    eprintln!("opt {pc}");
                    opt = true;
                }
            }
            Bytecode::End => {
                return (stdout, exec_counts);
            }

            Bytecode::OffsetRangeJumpZero { offset, rb, re, ptr, jmp } => {
                mem.offset += *offset as isize;
                if opt && (mem.offset < (*rb as isize) || (*re as isize) < mem.offset) {
                    eprintln!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    eprintln!("opt {pc}");
                    opt = true;
                }
                if mem[ptr] == 0 {
                    pc = pc.wrapping_add_signed(*jmp as isize);
                }
            }
            Bytecode::OffsetRangeJumpNotZero { offset, rb, re, ptr, jmp } => {
                mem.offset += *offset as isize;
                if opt && (mem.offset < (*rb as isize) || (*re as isize) < mem.offset) {
                    eprintln!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    eprintln!("opt {pc}");
                    opt = true;
                }
                if mem[ptr] != 0 {
                    pc = pc.wrapping_add_signed(*jmp as isize);
                }
            }
        }

        pc += 1;
    }
}
