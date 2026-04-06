use std::{fmt::Debug, ops::Range};

#[derive(Clone)]
pub struct CFG(pub Vec<CFGNode>);
impl Debug for CFG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CFG len: {} [", self.0.len())?;
        for (i, node) in self.0.iter().enumerate() {
            write!(f, "n{i}: {node:?}\n")?;
        }
        write!(f, "]")
    }
}

#[derive(Clone)]
pub struct CFGNode {
    pub predecessor: Vec<usize>,
    pub edge: CFGEdge,
    pub insts: Vec<CFGIR>,
    pub offset: Option<isize>,
}
impl Debug for CFGNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CFGNode pred: {:?} {{\n", self.predecessor)?;
        for inst in &self.insts {
            write!(f, "    {inst:?}\n")?;
        }
        if let Some(offset) = self.offset {
            write!(f, "    offset {offset}\n")?;
        }
        write!(f, "    {:?}\n}}", self.edge)
    }
}

#[derive(Clone)]
pub enum CFGEdge {
    Jump(usize),
    Branch {
        pointer: isize,
        zero: usize,
        nonzero: usize,
    },
    End,
}
impl Debug for CFGEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CFGEdge::Jump(addr) => write!(f, "jump n{addr}"),
            CFGEdge::End => write!(f, "end"),
            CFGEdge::Branch { pointer, zero, nonzero } => write!(f, "branch_zero ${pointer}, n{zero}, n{nonzero}"),
        }
    }
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
