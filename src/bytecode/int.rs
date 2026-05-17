use std::{io::{Read, Write, stdin, stdout}, ops::{Index, IndexMut}};

use crate::{TAPE_LENGTH, bytecode::bytecode::Bytecode, log};

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

pub fn debug_exec_bytecode<const DEBUG: bool>(bytecodes: &[Bytecode], offset: i16, mem: [u8; TAPE_LENGTH], pc: usize) -> Vec<u32> {
    let mut exec_counts = if DEBUG { vec![0; bytecodes.len()] } else { vec![] };
    let mut pc: usize = pc;
    let mut mem = Mem {
        offset: offset as isize,
        memory: mem,
    };
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let mut opt = false;

    loop {
        if DEBUG {
            exec_counts[pc] += 1;
        }
        match &bytecodes[pc] {
            Bytecode::SetC(p1, value) => {
                mem[p1] = *value;
            }
            Bytecode::SetL(p1, p2) => {
                mem[p1] = mem[p2];
            }
            Bytecode::Add(ptr, val) => {
                mem[ptr] = mem[ptr].wrapping_add(*val);
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
            
            Bytecode::Out(p1) => {
                if !DEBUG {
                    let _ = stdout.write(&[mem[p1]]);
                }
            }
            Bytecode::OutConst(v1) => {
                if !DEBUG {
                    let _ = stdout.write(&[*v1]);
                }
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
            Bytecode::OffsetWithRangeCheck(o1, range) => {
                mem.offset += *o1 as isize;
                if opt && !range.contains(mem.offset as i16) {
                    log!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    log!("opt {pc}");
                    opt = true;
                }
            }
            Bytecode::FindZero(ptr, delta) => {
                while mem[ptr] != 0 {
                    mem.offset += *delta as isize;
                }
            }
            Bytecode::FindZeroWithRangeCheck(ptr, delta, range) => {
                while mem[ptr] != 0 {
                    mem.offset += *delta as isize;
                }
                if opt && !range.contains(mem.offset as i16) {
                    log!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    log!("opt {pc}");
                    opt = true;
                }
            }
            Bytecode::End => {
                return exec_counts;
            }

            Bytecode::OffsetRangeJumpZero { offset, range, ptr, addr: jmp } => {
                mem.offset += *offset as isize;
                if opt && !range.contains(mem.offset as i16) {
                    log!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    log!("opt {pc}");
                    opt = true;
                }
                if mem[ptr] == 0 {
                    pc = pc.wrapping_add_signed(*jmp as isize);
                    continue;
                }
            }
            Bytecode::OffsetRangeJumpNotZero { offset, range, ptr, addr: jmp } => {
                mem.offset += *offset as isize;
                if opt && !range.contains(mem.offset as i16) {
                    log!("deopt {pc}");
                    opt = false;
                } else if !opt {
                    log!("opt {pc}");
                    opt = true;
                }
                if mem[ptr] != 0 {
                    pc = pc.wrapping_add_signed(*jmp as isize);
                    continue;
                }
            }
            Bytecode::SetCSetC(p1, c1, p2, c2) => {
                mem[p1] = *c1;
                mem[p2] = *c2;
            }
            Bytecode::AddAdd(p1, c1, p2, c2) => {
                mem[p1] = mem[p1].wrapping_add(*c1);
                mem[p2] = mem[p2].wrapping_add(*c2);
            }
            Bytecode::AddSetC(p1, c1, p2, c2) => {
                mem[p1] = mem[p1].wrapping_add(*c1);
                mem[p2] = *c2;
            }
            Bytecode::AddLSetC(p1, p2, p3, p4, c5) => {
                mem[p1] = mem[p2].wrapping_add(mem[p3]);
                mem[p4] = *c5;
            }
        }

        pc += 1;
    }
}
