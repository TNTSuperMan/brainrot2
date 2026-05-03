use std::{collections::HashMap, num::TryFromIntError, ops::RangeInclusive};

use crate::{bytecode::{bytecode::Bytecode, order::compute_block_order}, cfg::cfg::{CFG, CFGEdge, CFGExpr, CFGOp, CFGValue}};

fn try_into_bytecode(cfgop: &CFGOp) -> Result<Bytecode, TryFromIntError> {
    Ok(match cfgop {
        CFGOp::Breakpoint(p1) => Bytecode::Breakpoint((*p1).try_into()?),
        CFGOp::Out(CFGValue::Load(p1)) => Bytecode::Out((*p1).try_into()?),
        CFGOp::Out(CFGValue::Const(c1)) => Bytecode::OutConst(*c1),
        CFGOp::Assign(ptr, expr) => {
            let ptr = (*ptr).try_into()?;
            match expr {
                CFGExpr::Value(CFGValue::Const(c1)) => Bytecode::SetC(ptr, *c1),
                CFGExpr::Value(CFGValue::Load(p1)) => Bytecode::SetL(ptr, (*p1).try_into()?),

                CFGExpr::Add(CFGValue::Load(p1), CFGValue::Load(p2)) => Bytecode::AddL(ptr, (*p1).try_into()?, (*p2).try_into()?),
                CFGExpr::Add(CFGValue::Load(p), CFGValue::Const(c)) |
                CFGExpr::Add(CFGValue::Const(c), CFGValue::Load(p)) => Bytecode::AddC(ptr, (*p).try_into()?, *c),
                CFGExpr::Add(CFGValue::Const(c1), CFGValue::Const(c2)) => Bytecode::SetC(ptr, c1.wrapping_add(*c2)),

                CFGExpr::Sub(CFGValue::Load(p1), CFGValue::Load(p2)) => Bytecode::SubLL(ptr, (*p1).try_into()?, (*p2).try_into()?),
                CFGExpr::Sub(CFGValue::Load(p1), CFGValue::Const(c2)) => Bytecode::SubLC(ptr, (*p1).try_into()?, *c2),
                CFGExpr::Sub(CFGValue::Const(c1), CFGValue::Load(p2)) => Bytecode::SubCL(ptr, *c1, (*p2).try_into()?),
                CFGExpr::Sub(CFGValue::Const(c1), CFGValue::Const(c2)) => Bytecode::SetC(ptr, c1.wrapping_sub(*c2)),

                CFGExpr::Mul(CFGValue::Load(p1), CFGValue::Load(p2)) => Bytecode::MulL(ptr, (*p1).try_into()?, (*p2).try_into()?),
                CFGExpr::Mul(CFGValue::Load(p), CFGValue::Const(c)) |
                CFGExpr::Mul(CFGValue::Const(c), CFGValue::Load(p)) => Bytecode::MulC(ptr, (*p).try_into()?, *c),
                CFGExpr::Mul(CFGValue::Const(c1), CFGValue::Const(c2)) => Bytecode::SetC(ptr, c1.wrapping_mul(*c2)),

                CFGExpr::MulAdd(CFGValue::Load(p1), CFGValue::Load(p2), c3) => Bytecode::MulAddL(ptr, (*p1).try_into()?, (*p2).try_into()?, *c3),
                CFGExpr::MulAdd(CFGValue::Load(p1), CFGValue::Const(c2), c3) => Bytecode::AddC(ptr, (*p1).try_into()?, c2.wrapping_mul(*c3)),
                CFGExpr::MulAdd(CFGValue::Const(c1), CFGValue::Load(p2), c3) => Bytecode::MulAddC(ptr, *c1, (*p2).try_into()?, *c3),
                CFGExpr::MulAdd(CFGValue::Const(c1), CFGValue::Const(c2), c3) => Bytecode::SetC(ptr, c1.wrapping_add(c2.wrapping_mul(*c3))),

                CFGExpr::In => Bytecode::In(ptr),
            }
        }
    })
}

pub fn build_bytecode(cfg: &CFG, offset_ranges: &HashMap<usize, RangeInclusive<isize>>) -> Result<Vec<Bytecode>, TryFromIntError> {
    let mut bytecodes = vec![];
    let order = compute_block_order(cfg);

    let mut jumptable = vec![0; cfg.0.len()];

    // この時点のJump系命令はバイトコードアドレスではなくCFGブロックIDを指す
    for (i, b) in order.iter().enumerate() {
        jumptable[*b] = bytecodes.len().try_into()?;
        let block = &cfg.0[*b];
        bytecodes.append(&mut block.insts.iter().map(|e| try_into_bytecode(e)).collect::<Result<Vec<_>, _>>()?);

        if let Some(offset) = block.offset {
            let offset = offset.try_into()?;
            if let Some(range) = offset_ranges.get(&b) {
                let rb = (*range.start()).try_into()?;
                let re = (*range.end()).try_into()?;
                if let CFGEdge::Branch { pointer, zero, nonzero } = &block.edge {
                    if order.get(i + 1).copied() == Some(*nonzero) {

                        bytecodes.push(Bytecode::OffsetRangeJumpZero {
                            offset,
                            rb,
                            re,
                            ptr: (*pointer).try_into()?,
                            jmp: (*zero).try_into()?,
                        });

                        continue;
                    } else if order.get(i + 1).copied() == Some(*zero) {

                        bytecodes.push(Bytecode::OffsetRangeJumpNotZero {
                            offset,
                            rb,
                            re,
                            ptr: (*pointer).try_into()?,
                            jmp: (*nonzero).try_into()?,
                        });
                        
                        continue;
                    }
                }
                bytecodes.push(Bytecode::OffsetWithRangeCheck(offset, rb, re));
            } else {
                bytecodes.push(Bytecode::Offset(offset));
            }
        }

        match &block.edge {
            CFGEdge::Jump(to) => {
                if order.get(i + 1).copied() != Some(*to) {
                    bytecodes.push(Bytecode::Jump((*to).try_into()?));
                }
            }
            CFGEdge::Branch { pointer, zero, nonzero } => {
                if order.get(i + 1).copied() == Some(*nonzero) {
                    bytecodes.push(Bytecode::JumpIfZero((*pointer).try_into()?, (*zero).try_into()?));
                } else if order.get(i + 1).copied() == Some(*zero) {
                    bytecodes.push(Bytecode::JumpIfNotZero((*pointer).try_into()?, (*nonzero).try_into()?));
                } else {
                    bytecodes.push(Bytecode::JumpIfZero((*pointer).try_into()?, (*zero).try_into()?));
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
            Bytecode::JumpIfNotZero(_, addr) => {
                *addr = jumptable[*addr as usize] - (i as i32);
            }
            _ => {}
        }
    }

    Ok(bytecodes)
}
