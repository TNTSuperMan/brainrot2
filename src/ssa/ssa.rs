use std::fmt::Debug;

#[derive(Clone)]
pub struct SSAProgram(pub Vec<SSABlock>);

#[derive(Clone)]
pub struct SSABlock {
    pub predecessor: Vec<usize>,
    pub phis: Vec<Phi>,
    pub edge: SSAEdge,
    pub insts: Vec<SSAOp>,
    pub offset: Option<isize>,
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

#[derive(Clone, Copy)]
pub struct SSAVersion {
    pointer: isize,
    version: usize,
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

#[derive(Clone)]
pub enum PhiArg {
    Version(usize),
    Load,
}

#[derive(Clone)]
pub struct Phi {
    pointer: isize,
    define_version: usize,
    args: [PhiArg; 2],
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
    AddVV(SSAVersion, SSAVersion),
    AddVC(SSAVersion, u8),
    MulAdd(SSAVersion, SSAVersion, u8), // 0 + 1 * 2
    In,
}
