use std::{fmt::Debug, ops::Range};

#[derive(Clone, Debug)]
pub struct CFG(pub Vec<CFGNode>);

#[derive(Clone, Debug)]
pub struct CFGNode {
    pub predecessor: Vec<usize>,
    pub edge: CFGEdge,
    pub insts: Vec<CFGIR>,
    pub offset: Option<isize>,
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

#[derive(Clone)]
pub struct CFGIR {
    pub pointer: isize,
    pub opcode: CFGOp,
    pub loc: Range<usize>,
}
impl Debug for CFGIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode {
            CFGOp::Breakpoint => write!(f, "breakpoint"),
            CFGOp::Add(val) => write!(f, "add ${}, {val}", self.pointer),
            CFGOp::Set(val) => write!(f, "set ${}, {val}", self.pointer),
            CFGOp::MulAdd(p2, val) => write!(f, "muladd ${}, ${p2}, {val}", self.pointer),
            CFGOp::In => write!(f, "in ${}", self.pointer),
            CFGOp::Out => write!(f, "out ${}", self.pointer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CFGOp {
    Breakpoint,
    Add(u8),
    Set(u8),
    MulAdd(isize, u8), // [pointer] = [pointer] + [opcode.0] * opcode.1
    In,
    Out,
}
