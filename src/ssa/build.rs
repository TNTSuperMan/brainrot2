use crate::{cfg::cfg::{CFG, CFGEdge, CFGExpr, CFGOp, CFGValue}, ssa::defines::{block::{SSABlock, SSAEdge}, op::{SSAExpr, SSAOp}, program::SSAProgram, value::{SSAValue, SSAVersion}}};

fn cv2sv(cv: &CFGValue) -> SSAValue {
    match cv {
        CFGValue::Const(val) => SSAValue::Const(*val),
        CFGValue::Load(ptr) => SSAValue::Version(SSAVersion { pointer: *ptr, version: u32::MAX }),
    }
}

pub fn build_ssa(cfg: &CFG) -> SSAProgram {
    let mut ver = 0;

    let mut blocks = cfg.0.iter().map(|block| {
        SSABlock {
            alive: block.alive,
            predecessor: block.predecessor.clone(),
            edge: match block.edge {
                CFGEdge::Jump(to) => SSAEdge::Jump(to),
                CFGEdge::Branch { pointer, zero, nonzero } => SSAEdge::Branch { version: SSAVersion { pointer, version: u32::MAX }, zero, nonzero, ir_at: None },
                CFGEdge::BranchWithIRAt { pointer, zero, nonzero, ir_at } => SSAEdge::Branch { version: SSAVersion { pointer, version: u32::MAX }, zero, nonzero, ir_at: Some(ir_at) },
                CFGEdge::End => SSAEdge::End,
            },
            insts: block.insts.iter().map(|inst| {
                match inst {
                    CFGOp::Out(val) => SSAOp::Out(cv2sv(val)),
                    CFGOp::Assign(ptr, expr) => {
                        let v = SSAVersion { pointer: *ptr, version: ver };
                        ver += 1;

                        match expr {
                            CFGExpr::Value(val) => SSAOp::Hint(v, cv2sv(val)),
                            CFGExpr::Add(v1, v2) => SSAOp::Assign(v, SSAExpr::Add(cv2sv(v1), cv2sv(v2))),
                            CFGExpr::Sub(v1, v2) => SSAOp::Assign(v, SSAExpr::Sub(cv2sv(v1), cv2sv(v2))),
                            CFGExpr::Mul(v1, v2) => SSAOp::Assign(v, SSAExpr::Mul(cv2sv(v1), cv2sv(v2))),
                            CFGExpr::MulAdd(v1, v2, v3) => SSAOp::Assign(v, SSAExpr::MulAdd(cv2sv(v1), cv2sv(v2), *v3)),
                            CFGExpr::In => SSAOp::In(v),
                        }
                    }
                }
            }).collect(),
            offset: block.offset,
        }
    }).collect();

    

    SSAProgram(blocks)
}
