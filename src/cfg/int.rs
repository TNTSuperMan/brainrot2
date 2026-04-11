use std::io::{Read, Write, stdin, stdout};

use crate::cfg::cfg::{CFG, CFGEdge, CFGOp, CFGOpKind};

struct Mem {
    offset: isize,
    memory: [u8; 65536],
}
impl Mem {
    pub fn get(&self, idx: isize) -> u8 {
        self.memory[(self.offset + idx) as usize]
    }
    pub fn set(&mut self, idx: isize, val: u8) {
        self.memory[(self.offset + idx) as usize] = val;
    }
}

fn exec_node_ir(insts: &[CFGOp], mem: &mut Mem) {
    for CFGOp {
        pointer,
        opcode,
        loc: _,
    } in insts
    {
        match opcode {
            CFGOpKind::Breakpoint => {
                println!("break; {}", mem.offset);
            }
            CFGOpKind::Add(val) => {
                mem.set(*pointer, mem.get(*pointer).wrapping_add(*val));
            }
            CFGOpKind::Set(val) => {
                mem.set(*pointer, *val);
            }
            CFGOpKind::MulAdd(ptr2, val) => {
                mem.set(
                    *pointer,
                    mem.get(*pointer)
                        .wrapping_add(mem.get(*ptr2).wrapping_mul(*val)),
                );
            }
            CFGOpKind::In => {
                let mut stdin = stdin().lock();
                let mut buf = [0u8; 1];
                mem.set(
                    *pointer,
                    if stdin.read_exact(&mut buf).is_ok() {
                        buf[0]
                    } else {
                        0
                    },
                );
            }
            CFGOpKind::Out => {
                let mut stdout = stdout().lock();
                let _ = stdout.write(&[mem.get(*pointer)]);
            }
        }
    }
}

pub fn exec_from_cfg(cfg: &CFG) {
    let mut mem = Mem {
        offset: 0,
        memory: [0; 65536],
    };
    let mut node_i = 0;

    loop {
        exec_node_ir(&cfg.0[node_i].insts, &mut mem);
        if let Some(offset) = &cfg.0[node_i].offset {
            mem.offset += offset;
        }
        match &cfg.0[node_i].edge {
            CFGEdge::Jump(addr) => {
                node_i = *addr;
            }
            CFGEdge::Branch {
                pointer,
                zero,
                nonzero,
            } => {
                if mem.get(*pointer) == 0 {
                    node_i = *zero;
                } else {
                    node_i = *nonzero;
                }
            }
            CFGEdge::End => {
                return;
            }
        }
    }
}
