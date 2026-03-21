use std::{fmt::Debug, ops::Range};

#[derive(Clone, Debug)]
pub struct CFG(pub Vec<CFGNode>);

#[derive(Clone, Debug)]
pub struct CFGNode {
    pub edge: CFGEdge,
    pub insts: Vec<CFGIR>,
}

#[derive(Clone, Debug)]
pub enum CFGEdge {
    JumpNext,
    Branch {
        pointer: isize,
        zero: usize,
        nonzero: usize,
    },
    End,
}

#[derive(Clone, Debug)]
pub struct CFGIR {
    pub pointer: isize,
    pub opcode: CFGOp,
    pub loc: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CFGOp {
    Breakpoint,
    Add(u8),
    Set(u8),
    MulAdd(isize, u8), // [pointer] = [pointer] + [opcode.0] * opcode.1
    In,
    Out,
    Offset(isize),
    End,
}
