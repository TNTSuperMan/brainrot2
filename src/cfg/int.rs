use std::io::{Read, Write, stdin, stdout};

use crate::cfg::cfg::{CFG, CFGEdge, CFGIR, CFGOp};

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

fn exec_node_ir(insts: &[CFGIR], mem: &mut Mem) {
    for CFGIR {
        pointer,
        opcode,
        loc: _,
    } in insts
    {
        match opcode {
            CFGOp::Breakpoint => {
                println!("break; {}", mem.offset);
            }
            CFGOp::Add(val) => {
                mem.set(*pointer, mem.get(*pointer).wrapping_add(*val));
            }
            CFGOp::Set(val) => {
                mem.set(*pointer, *val);
            }
            CFGOp::MulAdd(ptr2, val) => {
                mem.set(
                    *pointer,
                    mem.get(*pointer)
                        .wrapping_add(mem.get(*ptr2).wrapping_mul(*val)),
                );
            }
            CFGOp::In => {
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
            CFGOp::Out => {
                let mut stdout = stdout().lock();
                let _ = stdout.write(&[mem.get(*pointer)]);
            }
            CFGOp::End => {
                return;
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
            CFGEdge::JumpNext => {
                node_i += 1;
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
