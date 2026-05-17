use std::collections::{HashMap, HashSet};

use crate::{
    cfg::cfg::{CFG, CFGBlock, CFGEdge, CFGExpr, CFGOp, CFGValue},
    ir::ir::{IR, IROp},
};

pub fn ir_to_cfgir(ir: &IR) -> Option<CFGOp> {
    Some(match ir.opcode {
        IROp::Add(val) => CFGOp::Assign(ir.pointer, CFGExpr::Add(CFGValue::Load(ir.pointer), CFGValue::Const(val))),
        IROp::Set(val) => CFGOp::Assign(ir.pointer, CFGExpr::Value(CFGValue::Const(val))),
        IROp::MulAdd(p, v) => CFGOp::Assign(ir.pointer, CFGExpr::MulAdd(CFGValue::Load(ir.pointer), CFGValue::Load(p), v)),
        IROp::In => CFGOp::Assign(ir.pointer, CFGExpr::In),
        IROp::Out => CFGOp::Out(CFGValue::Load(ir.pointer)),
        IROp::JumpZero(..) | IROp::JumpNotZero(..) | IROp::JumpNotZeroWithOffset(..) | IROp::FindZero(..) => {
            return None;
        }
    })
}

fn split_node(nodes: &mut Vec<CFGBlock>, index: usize) {
    let mut node_i = 0usize;
    let mut offset = index;

    while node_i < nodes.len() {
        let node_len = nodes[node_i].insts.len();
        if offset < node_len {
            break;
        }
        offset -= node_len;
        if let CFGEdge::Branch { .. } = nodes[node_i].edge {
            return;
        }
        node_i += 1;
    }

    if node_i >= nodes.len() || offset == 0 {
        return;
    }

    let right = nodes[node_i].insts.split_off(offset);
    let right_edge = nodes[node_i].edge.clone();
    nodes[node_i].edge = CFGEdge::Jump(usize::MAX);
    let right_offset = nodes[node_i].offset;
    nodes[node_i].offset = None;
    nodes.insert(
        node_i + 1,
        CFGBlock {
            insts: right,
            edge: right_edge,
            predecessor: vec![],
            offset: right_offset,
            alive: true,
        },
    );
}

impl CFG {
    pub fn new(insts: &[IR]) -> CFG {
        let mut nodes = vec![];
        let mut node_insts = vec![];
        let mut points: HashSet<usize> = HashSet::new();

        for (i, ir) in insts.iter().enumerate() {
            if points.contains(&i) {
                points.remove(&i);
                if node_insts.len() != 0 {
                    nodes.push(CFGBlock {
                        insts: node_insts,
                        edge: CFGEdge::Jump(usize::MAX),
                        predecessor: vec![],
                        offset: None,
                        alive: true,
                    });
                    node_insts = vec![];
                }
            }
            match ir.opcode {
                IROp::JumpZero(addr) => {
                    nodes.push(CFGBlock {
                        insts: node_insts,
                        edge: CFGEdge::BranchWithIRAt {
                            pointer: ir.pointer,
                            zero: addr,
                            nonzero: i + 1,
                            ir_at: i,
                        },
                        predecessor: vec![],
                        offset: None,
                        alive: true,
                    });
                    if addr < i {
                        split_node(&mut nodes, addr);
                    } else {
                        points.insert(addr);
                    }
                    node_insts = vec![];
                }
                IROp::JumpNotZero(addr) => {
                    nodes.push(CFGBlock {
                        insts: node_insts,
                        edge: CFGEdge::BranchWithIRAt {
                            pointer: ir.pointer,
                            zero: i + 1,
                            nonzero: addr,
                            ir_at: i,
                        },
                        predecessor: vec![],
                        offset: None,
                        alive: true,
                    });
                    if addr < i {
                        split_node(&mut nodes, addr);
                    } else {
                        points.insert(addr);
                    }
                    node_insts = vec![];
                }
                IROp::JumpNotZeroWithOffset(offset, addr) => {
                    nodes.push(CFGBlock {
                        insts: node_insts,
                        edge: CFGEdge::BranchWithIRAt {
                            pointer: ir.pointer,
                            zero: i + 1,
                            nonzero: addr,
                            ir_at: i,
                        },
                        predecessor: vec![],
                        offset: Some(offset),
                        alive: true,
                    });
                    if addr < i {
                        split_node(&mut nodes, addr);
                    } else {
                        points.insert(addr);
                    }
                    node_insts = vec![];
                }
                IROp::FindZero(delta) => {
                    nodes.push(CFGBlock {
                        insts: node_insts,
                        edge: CFGEdge::FindZeroAndJump {
                            pointer: ir.pointer,
                            delta,
                            jumpto: i + 1,
                        },
                        predecessor: vec![],
                        offset: None,
                        alive: true,
                    });
                    node_insts = vec![];
                }
                _ => node_insts.push(ir_to_cfgir(ir).unwrap()),
            }
        }
        let last_i = insts.len();
        if points.contains(&last_i) {
            points.remove(&last_i);
            if node_insts.len() != 0 {
                nodes.push(CFGBlock {
                    insts: node_insts,
                    edge: CFGEdge::Jump(usize::MAX),
                    predecessor: vec![],
                    offset: None,
                    alive: true,
                });
                node_insts = vec![];
            }
        }
        nodes.push(CFGBlock {
            predecessor: vec![],
            insts: node_insts,
            edge: CFGEdge::End,
            offset: None,
            alive: true,
        });

        let mut idx_map: HashMap<usize, usize> = HashMap::new();
        let mut idx_pc = 0usize;
        for (i, node) in nodes.iter().enumerate() {
            idx_map.insert(idx_pc, i);
            idx_pc += node.insts.len();
            if let CFGEdge::Branch { .. } | CFGEdge::BranchWithIRAt { .. } | CFGEdge::FindZeroAndJump { .. } = node.edge {
                idx_pc += 1;
            }
        }
        for i in 0..nodes.len() {
            if let CFGEdge::Jump(addr) = &mut nodes[i].edge {
                *addr = i + 1;
            } else {
                let addrs: Vec<&mut usize> = match &mut nodes[i].edge {
                    CFGEdge::Jump(..) => unreachable!(),
                    CFGEdge::Branch { zero, nonzero, .. } |
                    CFGEdge::BranchWithIRAt { zero, nonzero, .. } => vec![zero, nonzero],
                    CFGEdge::FindZeroAndJump { jumpto, .. } => vec![jumpto],
                    CFGEdge::End => vec![],
                };
                for addr in addrs {
                    *addr = *idx_map.get(addr).unwrap();
                }
            }
            match nodes[i].edge {
                CFGEdge::Jump(addr) => {
                    nodes[addr].predecessor.push(i);
                }
                CFGEdge::Branch {
                    pointer: _,
                    zero,
                    nonzero,
                } | CFGEdge::BranchWithIRAt {
                    pointer: _,
                    zero,
                    nonzero,
                    ir_at: _
                } => {
                    nodes[zero].predecessor.push(i);
                    nodes[nonzero].predecessor.push(i);
                }
                CFGEdge::FindZeroAndJump { jumpto, .. } => {
                    nodes[jumpto].predecessor.push(i);
                }
                CFGEdge::End => {}
            }
        }

        CFG(nodes)
    }
}
