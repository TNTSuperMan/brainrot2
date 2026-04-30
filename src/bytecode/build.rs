use std::num::TryFromIntError;

use crate::{bytecode::{bytecode::Bytecode, order::compute_block_order}, cfg::cfg::{CFG, CFGEdge, CFGOp, CFGOpKind}};

fn try_into_bytecode(cfgop: &CFGOp) -> Result<Bytecode, TryFromIntError> {
    let CFGOp { pointer, opcode, loc: _ } = cfgop;
    let p1 = (*pointer).try_into()?;
    Ok(match opcode {
        CFGOpKind::Breakpoint => Bytecode::Breakpoint(p1),
        CFGOpKind::Add(v2) => Bytecode::Add(p1, *v2),
        CFGOpKind::AddLoad(p2) => Bytecode::AddLoad(p1, (*p2).try_into()?),
        CFGOpKind::SubLoad(p2) => Bytecode::SubLoad(p1, (*p2).try_into()?),
        CFGOpKind::Set(v2) => Bytecode::Set(p1, *v2),
        CFGOpKind::SetLoad(p2) => Bytecode::SetLoad(p1, (*p2).try_into()?),
        CFGOpKind::MulAdd(p2, v3) => Bytecode::MulAdd(p1, (*p2).try_into()?, *v3),
        CFGOpKind::MulAddConst(v2, p3, v4) => Bytecode::MulAddConst(p1, *v2, (*p3).try_into()?, *v4),
        CFGOpKind::Mul(p2, v3) => Bytecode::Mul(p1, (*p2).try_into()?, *v3),
        CFGOpKind::In => Bytecode::In(p1),
        CFGOpKind::Out => Bytecode::Out(p1),
    })
}

pub fn build_bytecode(cfg: &CFG) -> Result<Vec<Bytecode>, TryFromIntError> {
    let mut bytecodes = vec![];
    let order = compute_block_order(cfg);

    let mut jumptable = vec![0u32; cfg.0.len()];

    // この時点のJump系命令はバイトコードアドレスではなくCFGブロックIDを指す
    for (i, b) in order.iter().enumerate() {
        jumptable[*b] = bytecodes.len().try_into()?;
        let block = &cfg.0[*b];
        bytecodes.append(&mut block.insts.iter().map(|e| try_into_bytecode(e)).collect::<Result<Vec<_>, _>>()?);

        if let Some(offset) = block.offset {
            bytecodes.push(Bytecode::Offset(offset.try_into()?));
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
                    // bytecodes.push(Bytecode::Jump((*nonzero).try_into()?));
                    bytecodes.push(Bytecode::Branch {
                        ptr: (*pointer).try_into()?,
                        zero: (*zero).try_into()?,
                        nonzero: (*nonzero).try_into()?
                    });
                }
            }
            CFGEdge::End => {
                bytecodes.push(Bytecode::End);
            }
        }
    }

    for code in bytecodes.iter_mut() {
        match code {
            Bytecode::Jump(addr) |
            Bytecode::JumpIfZero(_, addr) |
            Bytecode::JumpIfNotZero(_, addr) => {
                *addr = jumptable[*addr as usize];
            }
            Bytecode::Branch { ptr: _, zero, nonzero } => {
                *zero = jumptable[*zero as usize];
                *nonzero = jumptable[*nonzero as usize];
            }
            _ => {}
        }
    }

    Ok(bytecodes)
}
