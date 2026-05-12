use std::collections::HashMap;

use crate::ssa::defines::{op::SSAOp, value::SSAVersion};

#[derive(Clone, Debug)]
pub struct SSABlock {
    pub alive: bool,
    pub predecessor: Vec<usize>,
    pub phis: HashMap<i16, (u32, Vec<SSAVersion>)>,
    pub insts: Vec<SSAOp>,
    pub offset: Option<i16>,
    pub edge: SSAEdge,
}

#[derive(Clone, Debug)]
pub enum SSAEdge {
    Jump(usize),
    Branch {
        version: SSAVersion,
        zero: usize,
        nonzero: usize,
        ir_at: Option<usize>,
    },
    End,
}

impl SSABlock {
    pub fn find_def(&self, pointer: i16) -> Option<SSAVersion> {
        for inst in self.insts.iter().rev() {
            match inst {
                SSAOp::Out(_) => {}
                SSAOp::In(ver) | SSAOp::Hint(ver, _) | SSAOp::Assign(ver, _) => {
                    if ver.pointer == pointer {
                        return Some(*ver);
                    }
                }
            }
        }
        if let Some((version, _)) = self.phis.get(&pointer) {
            return Some(SSAVersion {
                pointer,
                version: *version,
            });
        }
        None
    }
}
