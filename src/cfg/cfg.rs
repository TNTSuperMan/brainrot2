use std::{fmt::Debug, ops::Range};

#[derive(Clone)]
pub struct CFG(pub Vec<CFGBlock>);
impl Debug for CFG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CFG len: {} [", self.0.len())?;
        for (i, node) in self.0.iter().enumerate() {
            write!(f, "n{i}: {node:?}\n")?;
        }
        write!(f, "]")
    }
}
impl CFG {
    pub fn update_edge(&mut self, block_i: usize, edge: CFGEdge) {
        for suc in self.0[block_i].edge.successor() {
            if let Some(idx) = self.0[suc].predecessor.iter().position(|&e| e == block_i) {
                self.0[suc].predecessor.remove(idx);
            }
        }
        for suc in edge.successor() {
            self.0[suc].predecessor.push(block_i);
        }
        self.0[block_i].edge = edge;
    }
}

#[derive(Clone)]
pub struct CFGBlock {
    pub predecessor: Vec<usize>,
    pub edge: CFGEdge,
    pub insts: Vec<CFGOp>,
    pub offset: Option<isize>,
    pub alive: bool,
}
impl Debug for CFGBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CFGBlock pred: {:?} {{\n", self.predecessor)?;
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
            CFGEdge::Branch {
                pointer,
                zero,
                nonzero,
            } => write!(f, "jump ${pointer} ? n{nonzero} : n{zero}"),
        }
    }
}
impl CFGEdge {
    pub fn successor(&self) -> Vec<usize> {
        match self {
            Self::Jump(to) => vec![*to],
            Self::Branch { pointer: _, zero, nonzero } => vec![*zero, *nonzero],
            Self::End => vec![],
        }
    }
}

#[derive(Clone)]
pub struct CFGOp {
    pub pointer: isize,
    pub opcode: CFGOpKind,
    pub loc: Range<usize>,
}
impl Debug for CFGOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode {
            CFGOpKind::Breakpoint => write!(f, "breakpoint"),
            CFGOpKind::Add(val) => write!(f, "${} = ${} + {val}", self.pointer, self.pointer),
            CFGOpKind::AddLoad(ptr) => write!(f, "${} = ${} + ${ptr}", self.pointer, self.pointer),
            CFGOpKind::SubLoad(ptr) => write!(f, "${} = ${} - ${ptr}", self.pointer, self.pointer),
            CFGOpKind::Set(val) => write!(f, "${} = {val}", self.pointer),
            CFGOpKind::SetLoad(ptr) => write!(f, "${} = ${ptr}", self.pointer),
            CFGOpKind::MulAdd(p2, val) => {
                write!(f, "${} = ${} + (${p2} * {val})", self.pointer, self.pointer)
            }
            CFGOpKind::MulAddConst(v1, p2, val) => {
                write!(f, "${} = {v1} + (${p2} * {val})", self.pointer)
            }
            CFGOpKind::Mul(p2, val) => {
                write!(f, "${} = ${p2} * {val}", self.pointer)
            }
            CFGOpKind::In => write!(f, "${} = stdin", self.pointer),
            CFGOpKind::Out => write!(f, "stdout < ${}", self.pointer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CFGOpKind {
    Breakpoint,
    Add(u8),
    AddLoad(isize),
    SubLoad(isize),
    Set(u8),
    SetLoad(isize),
    MulAdd(isize, u8), // [pointer] = [pointer] + [opcode.0] * opcode.1
    MulAddConst(u8, isize, u8), // [pointer] = opcode.0 + [opcode.1] * opcode.2
    Mul(isize, u8), // [pointer] = [opcode.0] * opcode.1
    In,
    Out,
}
