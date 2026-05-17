use std::io::{Read, Write, stdin, stdout};

use crate::{TAPE_LENGTH, cfg::cfg::{CFG, CFGEdge, CFGExpr, CFGOp, CFGValue}};

struct Mem {
    offset: i16,
    memory: [u8; TAPE_LENGTH],
}
impl Mem {
    fn get(&self, index: &CFGValue) -> u8 {
        match index {
            CFGValue::Load(ptr) => self.memory[(*ptr + self.offset) as usize],
            CFGValue::Const(c) => *c,
        }
    }
}

fn exec_node_ir(insts: &[CFGOp], mem: &mut Mem) {
    for opcode in insts
    {
        match opcode {
            CFGOp::Out(val) => {
                let mut stdout = stdout().lock();
                let _ = stdout.write(&[mem.get(val)]);
                let _ = stdout.flush();
            }
            CFGOp::Assign(ptr, expr) => {
                mem.memory[(*ptr + mem.offset) as usize] = match expr {
                    CFGExpr::Value(val) => mem.get(val),
                    CFGExpr::Add(v1, v2) => mem.get(v1).wrapping_add(mem.get(v2)),
                    CFGExpr::Sub(v1, v2) => mem.get(v1).wrapping_sub(mem.get(v2)),
                    CFGExpr::Mul(v1, v2) => mem.get(v1).wrapping_mul(mem.get(v2)),
                    CFGExpr::MulAdd(v1, v2, v3) => mem.get(v1).wrapping_add(mem.get(v2).wrapping_mul(*v3)),
                    CFGExpr::In => {
                        let mut stdin = stdin().lock();
                        let mut buf = [0u8; 1];
                        if stdin.read_exact(&mut buf).is_ok() {
                            buf[0]
                        } else {
                            0
                        }
                    }

                }
            }
        }
    }
}

pub fn exec_from_cfg(cfg: &CFG, offset: u8) {
    let mut mem = Mem {
        offset: offset as i16,
        memory: [0; TAPE_LENGTH],
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
            } | CFGEdge::BranchWithIRAt {
                pointer,
                zero,
                nonzero,
                ir_at: _
            } => {
                if mem.get(&CFGValue::Load(*pointer)) == 0 {
                    node_i = *zero;
                } else {
                    node_i = *nonzero;
                }
            }
            CFGEdge::FindZeroAndJump { pointer, delta, jumpto } => {
                while mem.get(&CFGValue::Load(*pointer)) != 0 {
                    mem.offset += delta;
                }
                node_i = *jumpto;
            }
            CFGEdge::End => {
                return;
            }
        }
    }
}
