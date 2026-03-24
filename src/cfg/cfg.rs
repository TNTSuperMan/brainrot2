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

#[derive(Clone)]
pub struct CFGIR {
    pub pointer: isize,
    pub opcode: CFGOp,
    pub loc: Range<usize>,
}
impl Debug for CFGIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pointer.fmt(f)?;
        f.write_str(" ")?;
        self.opcode.fmt(f)?;
        Ok(())
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
    Offset(isize),
    End,
}
