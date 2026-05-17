use std::fmt::Debug;

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
    pub offset: Option<i16>,
    pub alive: bool,
}
impl Debug for CFGBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.alive {
            write!(f, "CFGBlock pred: {:?} {{\n", self.predecessor)?;
            for inst in &self.insts {
                write!(f, "    {inst:?}\n")?;
            }
            if let Some(offset) = self.offset {
                write!(f, "    offset {offset}\n")?;
            }
            write!(f, "    {:?}\n}}", self.edge)
        } else {
            write!(f, "CFGBlock [dead]")
        }
    }
}

#[derive(Clone)]
pub enum CFGEdge {
    Jump(usize),
    Branch {
        pointer: i16,
        zero: usize,
        nonzero: usize,
    },
    BranchWithIRAt {
        pointer: i16,
        zero: usize,
        nonzero: usize,
        ir_at: usize,
    },
    FindZeroAndJump {
        pointer: i16,
        delta: i16,
        jumpto: usize,
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
            CFGEdge::BranchWithIRAt {
                pointer,
                zero,
                nonzero,
                ir_at,
            } => write!(f, "jump ${pointer} ? n{nonzero} : n{zero} (ir at {ir_at})"),
            CFGEdge::FindZeroAndJump { pointer, delta, jumpto } => write!(f, "findzero {pointer} {delta}, jump {jumpto}"),
        }
    }
}
impl CFGEdge {
    pub fn successor(&self) -> Vec<usize> {
        match self {
            Self::Jump(to) => vec![*to],
            Self::Branch { pointer: _, zero, nonzero } |
            Self::BranchWithIRAt { pointer: _, zero, nonzero, ir_at: _ } => vec![*zero, *nonzero],
            Self::FindZeroAndJump { jumpto, .. } => vec![*jumpto],
            Self::End => vec![],
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum CFGOp {
    Out(CFGValue),
    Assign(i16, CFGExpr)
}
impl Debug for CFGOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Out(val) => write!(f, "stdout < {val:?}"),
            Self::Assign(ptr, expr) => write!(f, "${ptr} = {expr:?}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum CFGExpr {
    Value(CFGValue),
    Add(CFGValue, CFGValue),
    Sub(CFGValue, CFGValue),
    Mul(CFGValue, CFGValue),
    MulAdd(CFGValue, CFGValue, u8), // [0] + [1] * 2
    In,
}
impl Debug for CFGExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(val) => write!(f, "{val:?}"),
            Self::Add(v1, v2) => write!(f, "{v1:?} + {v2:?}"),
            Self::Sub(v1, v2) => write!(f, "{v1:?} - {v2:?}"),
            Self::Mul(v1, v2) => write!(f, "{v1:?} * {v2:?}"),
            Self::MulAdd(v1, v2, v3) => write!(f, "{v1:?} + {v2:?} * {v3}"),
            Self::In => write!(f, "stdin"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CFGValue {
    Load(i16),
    Const(u8),
}
impl Debug for CFGValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(ptr) => write!(f, "${ptr}"),
            Self::Const(val) => write!(f, "{val}"),
        }
    }
}
