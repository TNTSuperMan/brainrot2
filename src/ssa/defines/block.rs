use crate::ssa::defines::{op::SSAOp, value::SSAVersion};

#[derive(Clone, Debug)]
pub struct SSABlock {
    pub predecessor: Vec<usize>,
    pub edge: SSAEdge,
    pub insts: Vec<SSAOp>,
    pub offset: Option<i16>,
    pub alive: bool,
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
