use std::{collections::HashMap, num::TryFromIntError};

use crate::{bytecode::order::compute_block_order, bytecode2::op, cfg::{cfg::{CFG, CFGEdge, CFGOp}, range::OffsetRange}};

fn cfgop_to_bytecode(insts: &[CFGOp]) -> Result<Vec<u8>, TryFromIntError> {
    todo!();
}

pub fn build_bytecode2(cfg: &CFG, ranges: &HashMap<usize, OffsetRange>) -> Result<Vec<u8>, TryFromIntError> {
    let mut address = vec![];
    let mut bytecodes = vec![];
    let mut jumpmap: Vec<u32> = vec![];
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
        bytecodes.append(&mut cfgop_to_bytecode(&block.insts)?);

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
