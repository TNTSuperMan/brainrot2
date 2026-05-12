use std::collections::HashMap;

use crate::ssa::defines::{
    op::SSAOp,
    value::{SSAValue, SSAVersion},
};

#[derive(Clone, Debug)]
pub struct SSABlock {
    pub alive: bool,
    pub predecessor: Vec<usize>,
    pub phis: HashMap<i16, (u32, Vec<SSAValue>)>,
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
    pub fn find_def(&self, pointer: i16) -> Option<SSAValue> {
        self.find_def_from(pointer, self.insts.len())
    }
    pub fn find_def_from(&self, pointer: i16, inst_i: usize) -> Option<SSAValue> {
        for inst in self.insts[..inst_i].iter().rev() {
            match inst {
                SSAOp::Out(_) => {}
                SSAOp::In(ver) | SSAOp::Assign(ver, _) => {
                    if ver.pointer == pointer {
                        return Some(SSAValue::Version(*ver));
                    }
                }
                SSAOp::Hint(ver, val) => {
                    if ver.pointer == pointer {
                        return Some(if ver.version == u32::MAX {
                            SSAValue::Version(*ver)
                        } else {
                            *val
                        });
                    }
                }
            }
        }
        if let Some((version, _)) = self.phis.get(&pointer) {
            Some(SSAValue::Version(SSAVersion {
                pointer,
                version: *version,
            }))
        } else {
            None
        }
    }
}
