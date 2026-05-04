use std::{collections::HashMap, num::TryFromIntError};

use crate::{bytecode::{bytecode::Bytecode, order::compute_block_order}, cfg::{cfg::{CFG, CFGEdge, CFGExpr, CFGOp, CFGValue}, range::OffsetRange}};

fn try_into_bytecode(cfgop: &CFGOp) -> Result<Bytecode, TryFromIntError> {
    Ok(match cfgop {
        CFGOp::Out(CFGValue::Load(p1)) => Bytecode::Out(*p1),
        CFGOp::Out(CFGValue::Const(c1)) => Bytecode::OutConst(*c1),
        CFGOp::Assign(ptr, expr) => {
            let ptr = *ptr;
            match expr {
                CFGExpr::Value(CFGValue::Const(c1)) => Bytecode::SetC(ptr, *c1),
                CFGExpr::Value(CFGValue::Load(p1)) => Bytecode::SetL(ptr, *p1 ),

                CFGExpr::Add(CFGValue::Load(p1), CFGValue::Load(p2)) => Bytecode::AddL(ptr, *p1, *p2),
                CFGExpr::Add(CFGValue::Load(p), CFGValue::Const(c)) |
                CFGExpr::Add(CFGValue::Const(c), CFGValue::Load(p)) => {
                    if ptr == *p {
                        Bytecode::Add(ptr, *c)
                    } else {
                        Bytecode::AddC(ptr, *p, *c)
                    }
                },
                CFGExpr::Add(CFGValue::Const(c1), CFGValue::Const(c2)) => Bytecode::SetC(ptr, c1.wrapping_add(*c2)),

                CFGExpr::Sub(CFGValue::Load(p1), CFGValue::Load(p2)) => Bytecode::SubLL(ptr, *p1, *p2 ),
                CFGExpr::Sub(CFGValue::Load(p1), CFGValue::Const(c2)) => Bytecode::SubLC(ptr, *p1, *c2),
                CFGExpr::Sub(CFGValue::Const(c1), CFGValue::Load(p2)) => Bytecode::SubCL(ptr, *c1, *p2 ),
                CFGExpr::Sub(CFGValue::Const(c1), CFGValue::Const(c2)) => Bytecode::SetC(ptr, c1.wrapping_sub(*c2)),

                CFGExpr::Mul(CFGValue::Load(p1), CFGValue::Load(p2)) => Bytecode::MulL(ptr, *p1 , *p2 ),
                CFGExpr::Mul(CFGValue::Load(p), CFGValue::Const(c)) |
                CFGExpr::Mul(CFGValue::Const(c), CFGValue::Load(p)) => Bytecode::MulC(ptr, *p, *c),
                CFGExpr::Mul(CFGValue::Const(c1), CFGValue::Const(c2)) => Bytecode::SetC(ptr, c1.wrapping_mul(*c2)),

                CFGExpr::MulAdd(CFGValue::Load(p1), CFGValue::Load(p2), c3) => Bytecode::MulAddL(ptr, *p1, *p2, *c3),
                CFGExpr::MulAdd(CFGValue::Load(p1), CFGValue::Const(c2), c3) => Bytecode::AddC(ptr, *p1, c2.wrapping_mul(*c3)),
                CFGExpr::MulAdd(CFGValue::Const(c1), CFGValue::Load(p2), c3) => Bytecode::MulAddC(ptr, *c1, *p2, *c3),
                CFGExpr::MulAdd(CFGValue::Const(c1), CFGValue::Const(c2), c3) => Bytecode::SetC(ptr, c1.wrapping_add(c2.wrapping_mul(*c3))),

                CFGExpr::In => Bytecode::In(ptr),
            }
        }
    })
}

fn cfgops_to_bytecodes(insts: &[CFGOp]) -> Result<Vec<Bytecode>, TryFromIntError> {
    if insts.is_empty() {
        return Ok(vec![]);
    }
    let mut codes = vec![];
    let mut i = 0;
    let len_sub = insts.len() - 1;
    loop {
        if i >= len_sub {
            break;
        }
        let curr_op = try_into_bytecode(&insts[i])?;
        let next_op = try_into_bytecode(&insts[i + 1])?;
        
        match (curr_op, next_op) {
            (Bytecode::SetC(p1, c1), Bytecode::SetC(p2, c2)) => {
                codes.push(Bytecode::SetCSetC(p1, c1, p2, c2));
                i += 2;
                continue;
            }
            (Bytecode::Add(p1, c1), Bytecode::Add(p2, c2)) => {
                codes.push(Bytecode::AddAdd(p1, c1, p2, c2));
                i += 2;
                continue;
            }
            (Bytecode::Add(p1, c1), Bytecode::SetC(p2, c2)) |
            (Bytecode::SetC(p2, c2), Bytecode::Add(p1, c1)) => {
                if p1 != p2 {
                    codes.push(Bytecode::AddSetC(p1, c1, p2, c2));
                    i += 2;
                    continue;
                }
            }
            (Bytecode::AddL(p1, p2, p3), Bytecode::SetC(p4, c5)) |
            (Bytecode::SetC(p4, c5), Bytecode::AddL(p1, p2, p3)) => {
                if p1 != p4 {
                    codes.push(Bytecode::AddLSetC(p1, p2, p3, p4, c5));
                    i += 2;
                    continue;
                }
            }
            _ => {}
        }
        codes.push(try_into_bytecode(&insts[i])?);
        i += 1;
    }
    if i == len_sub {
        codes.push(try_into_bytecode(&insts[i])?);
    }
    Ok(codes)
}

pub fn build_bytecode(cfg: &CFG, offset_ranges: &HashMap<usize, OffsetRange>) -> Result<(Vec<Bytecode>, HashMap<usize, usize>), TryFromIntError> {
    let mut bytecodes = vec![];
    let order = compute_block_order(cfg);

    let mut jumptable = vec![0; cfg.0.len()];

    let mut ir_map = HashMap::new();

    // この時点のJump系命令はバイトコードアドレスではなくCFGブロックIDを指す
    for (i, b) in order.iter().enumerate() {
        jumptable[*b] = bytecodes.len().try_into()?;
        let block = &cfg.0[*b];
        bytecodes.append(&mut cfgops_to_bytecodes(&block.insts)?);

        if let Some(offset) = block.offset {
            let offset = offset;
            if let Some(&range) = offset_ranges.get(&b) {
                if let CFGEdge::Branch { pointer, zero, nonzero } = &block.edge {
                    if order.get(i + 1).copied() == Some(*nonzero) {

                        bytecodes.push(Bytecode::OffsetRangeJumpZero {
                            offset,
                            range,
                            ptr: (*pointer),
                            addr: (*zero).try_into()?,
                        });

                        continue;
                    } else if order.get(i + 1).copied() == Some(*zero) {

                        bytecodes.push(Bytecode::OffsetRangeJumpNotZero {
                            offset,
                            range,
                            ptr: (*pointer),
                            addr: (*nonzero).try_into()?,
                        });
                        
                        continue;
                    }
                }
                bytecodes.push(Bytecode::OffsetWithRangeCheck(offset, range));
            } else {
                bytecodes.push(Bytecode::Offset(offset));
            }
        }
        
        if let CFGEdge::BranchWithIRAt { ir_at, .. } = &block.edge {
            ir_map.insert(*ir_at, bytecodes.len());
        }

        match &block.edge {
            CFGEdge::Jump(to) => {
                if order.get(i + 1).copied() != Some(*to) {
                    bytecodes.push(Bytecode::Jump((*to).try_into()?));
                }
            }
            CFGEdge::Branch { pointer, zero, nonzero } |
            CFGEdge::BranchWithIRAt { pointer, zero, nonzero, ir_at: _ } => {
                if order.get(i + 1).copied() == Some(*nonzero) {
                    bytecodes.push(Bytecode::JumpIfZero(*pointer, (*zero).try_into()?));
                } else if order.get(i + 1).copied() == Some(*zero) {
                    bytecodes.push(Bytecode::JumpIfNotZero(*pointer, (*nonzero).try_into()?));
                } else {
                    bytecodes.push(Bytecode::JumpIfZero(*pointer, (*zero).try_into()?));
                    bytecodes.push(Bytecode::Jump((*nonzero).try_into()?));
                }
            }
            CFGEdge::End => {
                bytecodes.push(Bytecode::End);
            }
        }
    }

    for (i, code) in bytecodes.iter_mut().enumerate() {
        match code {
            Bytecode::Jump(addr) |
            Bytecode::JumpIfZero(_, addr) |
            Bytecode::JumpIfNotZero(_, addr) |
            Bytecode::OffsetRangeJumpZero { addr, .. } |
            Bytecode::OffsetRangeJumpNotZero { addr, .. } => {
                *addr = jumptable[*addr as usize] - (i as i32);
            }
            _ => {}
        }
    }

    Ok((bytecodes, ir_map))
}
