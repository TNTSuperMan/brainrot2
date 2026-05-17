use std::{collections::HashMap, num::TryFromIntError};

use crate::{bytecode::order::compute_block_order, bytecode2::op, cfg::{cfg::{CFG, CFGEdge, CFGExpr, CFGOp, CFGValue}, range::OffsetRange}};

fn push_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn cfgop_to_bytecode(inst: &CFGOp) -> Vec<u8> {
    let mut bytecode = vec![];
    match inst {
        CFGOp::Out(CFGValue::Load(p1)) => {
            bytecode.push(op::OP_OUTLOAD);
            push_i16(&mut bytecode, *p1);
        }
        CFGOp::Out(CFGValue::Const(c1)) => {
            bytecode.push(op::OP_OUTCONST);
            bytecode.push(*c1);
        }
        CFGOp::Assign(ptr, expr) => {
            let ptr = *ptr;
            match expr {
                CFGExpr::Value(CFGValue::Const(c1)) => {
                    bytecode.push(op::OP_SETCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(*c1);
                }
                CFGExpr::Value(CFGValue::Load(p1)) => {
                    bytecode.push(op::OP_SETLOAD);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p1);
                }
                CFGExpr::Add(CFGValue::Load(p1), CFGValue::Load(p2)) => {
                    bytecode.push(op::OP_ADDLOAD);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p1);
                    push_i16(&mut bytecode, *p2);
                }
                CFGExpr::Add(CFGValue::Load(p), CFGValue::Const(c))
                | CFGExpr::Add(CFGValue::Const(c), CFGValue::Load(p)) => {
                    if ptr == *p {
                        bytecode.push(op::OP_ADDASSIGN);
                        push_i16(&mut bytecode, ptr);
                        bytecode.push(*c);
                    } else {
                        bytecode.push(op::OP_ADDCONST);
                        push_i16(&mut bytecode, ptr);
                        push_i16(&mut bytecode, *p);
                        bytecode.push(*c);
                    }
                }
                CFGExpr::Add(CFGValue::Const(c1), CFGValue::Const(c2)) => {
                    bytecode.push(op::OP_SETCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(c1.wrapping_add(*c2));
                }
                CFGExpr::Sub(CFGValue::Load(p1), CFGValue::Load(p2)) => {
                    bytecode.push(op::OP_SUBLL);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p1);
                    push_i16(&mut bytecode, *p2);
                }
                CFGExpr::Sub(CFGValue::Load(p1), CFGValue::Const(c2)) => {
                    bytecode.push(op::OP_SUBLC);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p1);
                    bytecode.push(*c2);
                }
                CFGExpr::Sub(CFGValue::Const(c1), CFGValue::Load(p2)) => {
                    bytecode.push(op::OP_SUBCL);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(*c1);
                    push_i16(&mut bytecode, *p2);
                }
                CFGExpr::Sub(CFGValue::Const(c1), CFGValue::Const(c2)) => {
                    bytecode.push(op::OP_SETCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(c1.wrapping_sub(*c2));
                }
                CFGExpr::Mul(CFGValue::Load(p1), CFGValue::Load(p2)) => {
                    bytecode.push(op::OP_MULLOAD);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p1);
                    push_i16(&mut bytecode, *p2);
                }
                CFGExpr::Mul(CFGValue::Load(p), CFGValue::Const(c))
                | CFGExpr::Mul(CFGValue::Const(c), CFGValue::Load(p)) => {
                    bytecode.push(op::OP_MULCONST);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p);
                    bytecode.push(*c);
                }
                CFGExpr::Mul(CFGValue::Const(c1), CFGValue::Const(c2)) => {
                    bytecode.push(op::OP_SETCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(c1.wrapping_mul(*c2));
                }
                CFGExpr::MulAdd(CFGValue::Load(p1), CFGValue::Load(p2), c3) => {
                    bytecode.push(op::OP_MULADDLOAD);
                    push_i16(&mut bytecode, ptr);
                    push_i16(&mut bytecode, *p1);
                    push_i16(&mut bytecode, *p2);
                    bytecode.push(*c3);
                }
                CFGExpr::MulAdd(CFGValue::Load(p1), CFGValue::Const(c2), c3) => {
                    bytecode.push(op::OP_MULADDCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(c2.wrapping_mul(*c3));
                    push_i16(&mut bytecode, *p1);
                    bytecode.push(*c3);
                }
                CFGExpr::MulAdd(CFGValue::Const(c1), CFGValue::Load(p2), c3) => {
                    bytecode.push(op::OP_MULADDCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(*c1);
                    push_i16(&mut bytecode, *p2);
                    bytecode.push(*c3);
                }
                CFGExpr::MulAdd(CFGValue::Const(c1), CFGValue::Const(c2), c3) => {
                    bytecode.push(op::OP_SETCONST);
                    push_i16(&mut bytecode, ptr);
                    bytecode.push(c1.wrapping_add(c2.wrapping_mul(*c3)));
                }
                CFGExpr::In => {
                    bytecode.push(op::OP_IN);
                    push_i16(&mut bytecode, ptr);
                }
            }
        }
    }

    bytecode
}

fn cfgops_to_bytecode(insts: &[CFGOp]) -> Result<Vec<u8>, TryFromIntError> {
    let mut bytecode = vec![];
    for inst in insts {
        bytecode.extend(cfgop_to_bytecode(inst));
    }
    Ok(bytecode)
}

pub fn build_bytecode2(cfg: &CFG, ranges: &HashMap<usize, OffsetRange>) -> Result<Vec<u8>, TryFromIntError> {
    let mut address = vec![];
    let mut bytecodes = vec![];
    let mut jumpmap: Vec<u32> = vec![0; cfg.0.len()];
    let mut ir_map = HashMap::new();

    let order = compute_block_order(cfg);

    macro_rules! addr {
        ($addr: expr) => {
            address.push(bytecodes.len());
            bytecodes.append(&mut ($addr as u32).to_le_bytes().to_vec())
        };
    }

    for (i, &b) in order.iter().enumerate() {
        jumpmap[b] = bytecodes.len().try_into()?;

        let block = &cfg.0[b];
        bytecodes.append(&mut cfgops_to_bytecode(&block.insts)?);

        if let Some(offset) = block.offset {
            if let Some(range) = ranges.get(&b) {
                if let CFGEdge::Branch { pointer, zero, nonzero } | CFGEdge::BranchWithIRAt { pointer, zero, nonzero, .. } = &block.edge {
                    if order.get(i + 1) == Some(&nonzero) {
                        bytecodes.push(op::OP_OFFSETRANGEJUMPZERO);
                        bytecodes.append(&mut offset.to_le_bytes().to_vec());
                        bytecodes.append(&mut range.start.to_le_bytes().to_vec());
                        bytecodes.append(&mut range.end.to_le_bytes().to_vec());
                        bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                        addr!(*zero);
                        continue;
                    } else if order.get(i + 1) == Some(&zero) {
                        bytecodes.push(op::OP_OFFSETRANGEJUMPNOTZERO);
                        bytecodes.append(&mut offset.to_le_bytes().to_vec());
                        bytecodes.append(&mut range.start.to_le_bytes().to_vec());
                        bytecodes.append(&mut range.end.to_le_bytes().to_vec());
                        bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                        addr!(*nonzero);
                        continue;
                    }
                }
                bytecodes.push(op::OP_OFFSETWITHRANGECHECK);
                bytecodes.append(&mut offset.to_le_bytes().to_vec());
                bytecodes.append(&mut range.start.to_le_bytes().to_vec());
                bytecodes.append(&mut range.end.to_le_bytes().to_vec());
            } else {
                bytecodes.push(op::OP_OFFSET);
                bytecodes.append(&mut offset.to_le_bytes().to_vec());
            }
        }

        if let CFGEdge::BranchWithIRAt { ir_at, .. } = &block.edge {
            ir_map.insert(*ir_at, bytecodes.len());
        }

        match &block.edge {
            CFGEdge::Jump(addr) => {
                if order.get(i + 1) != Some(addr) {
                    bytecodes.push(op::OP_JUMP);
                    addr!(*addr);
                }
            }
            CFGEdge::Branch { pointer, zero, nonzero } | CFGEdge::BranchWithIRAt { pointer, zero, nonzero, .. } => {
                if order.get(i + 1) == Some(nonzero) {
                    bytecodes.push(op::OP_JUMPIFZERO);
                    bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                    addr!(*zero);
                } else if order.get(i + 1) == Some(zero) {
                    bytecodes.push(op::OP_JUMPIFNOTZERO);
                    bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                    addr!(*nonzero);
                } else {
                    bytecodes.push(op::OP_JUMPIFZERO);
                    bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                    addr!(*zero);
                    bytecodes.push(op::OP_JUMP);
                    addr!(*nonzero);
                }
            }
            CFGEdge::FindZeroAndJump { pointer, delta, jumpto } => {
                if let Some(range) = ranges.get(&b) {
                    bytecodes.push(op::OP_FINDZEROWITHRANCECHECK);
                    bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                    bytecodes.append(&mut delta.to_le_bytes().to_vec());
                    bytecodes.append(&mut range.start.to_le_bytes().to_vec());
                    bytecodes.append(&mut range.end.to_le_bytes().to_vec());
                } else {
                    bytecodes.push(op::OP_FINDZERO);
                    bytecodes.append(&mut pointer.to_le_bytes().to_vec());
                    bytecodes.append(&mut delta.to_le_bytes().to_vec());
                }

                if order.get(i + 1) != Some(jumpto) {
                    bytecodes.push(op::OP_JUMP);
                    addr!(*jumpto);
                }
            }
            CFGEdge::End => {
                bytecodes.push(op::OP_END);
            }
        }
    }

    for addr in address {
        let addr_b = bytecodes[addr..(addr + 4)].as_mut_array().unwrap();
        
        let block_id = u32::from_le_bytes(addr_b.clone());
        *addr_b = jumpmap[block_id as usize].to_le_bytes();
    }

    Ok(bytecodes)
}
