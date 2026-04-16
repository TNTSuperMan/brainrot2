use std::fmt::Debug;

#[derive(Clone)]
pub struct SSAProgram(pub Vec<SSABlock>);
impl Debug for SSAProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSAProgram len: {} [", self.0.len())?;
        for (i, node) in self.0.iter().enumerate() {
            write!(f, "n{i}: {node:?}\n")?;
        }
        write!(f, "]")
    }
}

#[derive(Clone)]
pub struct SSABlock {
    pub predecessor: Vec<usize>,
    pub phis: Vec<Phi>,
    pub edge: SSAEdge,
    pub insts: Vec<SSAOp>,
    pub offset: Option<isize>,
}
impl Debug for SSABlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSABlock pred: {:?} {{\n", self.predecessor)?;
        for phi in &self.phis {
            write!(f, "    {phi:?}\n")?;
        }
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
pub enum SSAEdge {
    Jump(usize),
    Branch {
        version: SSAVersion,
        zero: usize,
        nonzero: usize,
    },
    End,
}
impl Debug for SSAEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jump(addr) => write!(f, "jump n{addr}"),
            Self::Branch {
                version,
                zero,
                nonzero,
            } => write!(f, "jump {version:?} ? n{nonzero} : n{zero}"),
            Self::End => write!(f, "end"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SSAVersion {
    pub pointer: isize,
    pub version: usize,
}
impl Debug for SSAVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}#{}", self.pointer, self.version)
    }
}

#[derive(Clone)]
pub enum SSAOp {
    Breakpoint,
    Define(SSAVersion, SSAExpr),
    AssignToCell(SSAVersion),
    Out(SSAVersion),
}
impl Debug for SSAOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Breakpoint => write!(f, "breakpoint"),
            Self::Define(ver, expr) => write!(f, "{ver:?} = {expr:?}"),
            Self::AssignToCell(ver) => write!(f, "tape[{}] = {ver:?}", ver.pointer),
            Self::Out(ver) => write!(f, "stdout < {ver:?}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum PhiArg {
    Version(usize),
    Load,
}

#[derive(Clone)]
pub struct Phi {
    pub pointer: isize,
    pub define_version: usize,
    pub args: [PhiArg; 2],
}
impl Debug for Phi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}#{} = φ(", self.pointer, self.define_version)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            match arg {
                PhiArg::Version(ver) => write!(f, "${}#{ver}", self.pointer),
                PhiArg::Load => write!(f, "${}", self.pointer),
            }?;
        }
        write!(f, ")")
    }
}

#[derive(Clone)]
pub enum SSAExpr {
    Const(u8),
    Ref(SSAVersion),
    AddVC(SSAVersion, u8),
    MulAdd(SSAVersion, SSAVersion, u8), // 0 + 1 * 2
    Mul(SSAVersion, u8),
    In,
}
impl Debug for SSAExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(val) => write!(f, "{val}"),
            Self::Ref(ver) => write!(f, "{ver:?}"),
            Self::AddVC(v1, v2) => write!(f, "{v1:?} + {v2}"),
            Self::MulAdd(v1, v2, v3) => write!(f, "{v1:?} + ({v2:?} * {v3})"),
            Self::Mul(v1, v2) => write!(f, "{v1:?} * {v2}"),
            Self::In => write!(f, "stdin"),
        }
    }
}
