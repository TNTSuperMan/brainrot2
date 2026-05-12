use std::collections::HashMap;

use crate::ssa::defines::{op::SSAOp, value::SSAVersion};

#[derive(Clone, Debug)]
pub struct SSABlock {
    pub alive: bool,
    pub predecessor: Vec<usize>,
    pub phis: HashMap<i16, (usize, Vec<SSAVersion>)>,
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
