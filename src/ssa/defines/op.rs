use crate::ssa::defines::value::{SSAValue, SSAVersion};

pub enum SSAOp {
    Out(SSAValue),
    In(SSAVersion),
    Assign(SSAVersion, SSAExpr),
    Hint(SSAVersion, SSAValue),
}

pub enum SSAExpr {
    Phi(Vec<SSAValue>),
    Add(SSAValue, SSAValue),
    Sub(SSAValue, SSAValue),
    Mul(SSAValue, SSAValue),
    MulAdd(SSAValue, SSAValue, u8),
}
