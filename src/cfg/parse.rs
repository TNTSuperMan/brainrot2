use std::collections::{HashMap, HashSet};

use crate::{
    cfg::cfg::{CFG, CFGEdge, CFGIR, CFGNode, CFGOp},
    ir::ir::{IR, IROp},
};

pub fn ir_to_cfgir(ir: &IR) -> Option<CFGIR> {
    Some(CFGIR {
        pointer: ir.pointer,
        loc: ir.loc.clone(),
        opcode: match ir.opcode {
            IROp::Breakpoint => CFGOp::Breakpoint,
            IROp::Add(val) => CFGOp::Add(val),
            IROp::Set(val) => CFGOp::Set(val),
            IROp::MulAdd(p, v) => CFGOp::MulAdd(p, v),
            IROp::In => CFGOp::In,
            IROp::Out => CFGOp::Out,
            IROp::JumpZero(..) | IROp::JumpNotZero(..) | IROp::JumpNotZeroWithOffset(..) => {
                return None;
            }
        },
    })
}

fn split_node(nodes: &mut Vec<CFGNode>, index: usize) {
    let mut node_i = 0usize;
    let mut i = 0usize;
    for _ in 0..index {
        i = i.wrapping_add(1);
        if nodes[node_i].insts.len() <= i {
            i = if let CFGEdge::Branch { .. } = nodes[node_i].edge {
                usize::MAX
            } else {
                0
            };
            node_i += 1;
            continue;
        }
    }
    if i == usize::MAX || i == 0 {
        return;
    }
    let right = nodes[node_i].insts.split_off(i);
    let right_edge = nodes[node_i].edge.clone();
    nodes[node_i].edge = CFGEdge::Jump(usize::MAX);
    let right_offset = nodes[node_i].offset;
    nodes[node_i].offset = None;
    nodes.insert(
        node_i + 1,
        CFGNode {
            insts: right,
            edge: right_edge,
            predecessor: vec![],
            offset: right_offset,
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
                    nodes.push(CFGNode {
                        insts: node_insts,
                        edge: CFGEdge::Jump(usize::MAX),
                        predecessor: vec![],
                        offset: None,
                    });
                    node_insts = vec![];
                }
            }
            match ir.opcode {
                IROp::JumpZero(addr) => {
                    nodes.push(CFGNode {
                        insts: node_insts,
                        edge: CFGEdge::Branch {
                            pointer: ir.pointer,
                            zero: addr,
                            nonzero: i + 1,
                        },
                        predecessor: vec![],
                        offset: None,
                    });
                    if addr < i {
                        split_node(&mut nodes, addr);
                    } else {
                        points.insert(addr);
                    }
                    node_insts = vec![];
                }
                IROp::JumpNotZero(addr) => {
                    nodes.push(CFGNode {
                        insts: node_insts,
                        edge: CFGEdge::Branch {
                            pointer: ir.pointer,
                            zero: i + 1,
                            nonzero: addr,
                        },
                        predecessor: vec![],
                        offset: None,
                    });
                    if addr < i {
                        split_node(&mut nodes, addr);
                    } else {
                        points.insert(addr);
                    }
                    node_insts = vec![];
                }
                IROp::JumpNotZeroWithOffset(offset, addr) => {
                    nodes.push(CFGNode {
                        insts: node_insts,
                        edge: CFGEdge::Branch {
                            pointer: ir.pointer,
                            zero: i + 1,
                            nonzero: addr,
                        },
                        predecessor: vec![],
                        offset: Some(offset),
                    });
                    if addr < i {
                        split_node(&mut nodes, addr);
                    } else {
                        points.insert(addr);
                    }
                    node_insts = vec![];
                }
                _ => node_insts.push(ir_to_cfgir(ir).unwrap()),
            }
        }
        nodes.push(CFGNode {
            predecessor: vec![],
            insts: node_insts,
            edge: CFGEdge::End,
            offset: None,
        });

        let mut idx_map: HashMap<usize, usize> = HashMap::new();
        let mut idx_pc = 0usize;
        for (i, node) in nodes.iter().enumerate() {
            idx_map.insert(idx_pc, i);
            idx_pc += node.insts.len();
            if let CFGEdge::Branch { .. } = node.edge {
                idx_pc += 1;
            }
        }
        for i in 0..nodes.len() {
            if let CFGEdge::Branch {
                pointer: _,
                zero,
                nonzero,
            } = &mut nodes[i].edge
            {
                *zero = *idx_map.get(zero).unwrap();
                *nonzero = *idx_map.get(nonzero).unwrap();
            }
            match nodes[i].edge {
                CFGEdge::Jump(usize::MAX) => {
                    nodes[i].edge = CFGEdge::Jump(i + 1);
                    nodes[i + 1].predecessor.push(i);
                }
                CFGEdge::Jump(addr) => {
                    nodes[addr].predecessor.push(i);
                }
                CFGEdge::Branch { pointer: _, zero, nonzero } => {
                    nodes[zero].predecessor.push(i);
                    nodes[nonzero].predecessor.push(i);
                }
                CFGEdge::End => {}
            }
        }

        CFG(nodes)
    }
}
