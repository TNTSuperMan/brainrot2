use crate::ssa::defines::value::{SSAValue, SSAVersion};

#[derive(Clone, Debug)]
pub enum SSAOp {
    Out(SSAValue),
    In(SSAVersion),
    Assign(SSAVersion, SSAExpr),
    Hint(SSAVersion, SSAValue),
}

impl SSAOp {
    pub fn reads(&self) -> Vec<i16> {
        let values = match self {
            SSAOp::In(_) => vec![],
            SSAOp::Out(val) | SSAOp::Hint(_, val) => vec![val],
            SSAOp::Assign(_, expr) => match expr {
                SSAExpr::Add(v1, v2)
                | SSAExpr::Sub(v1, v2)
                | SSAExpr::Mul(v1, v2)
                | SSAExpr::MulAdd(v1, v2, _) => vec![v1, v2],
            },
        };
        let mut reads = vec![];
        for value in values {
            if let SSAValue::Version(SSAVersion {
                pointer,
                version: _,
            }) = value
            {
                reads.push(*pointer);
            }
        }
        reads
    }
    pub fn get_values_mut(&mut self) -> Vec<&mut SSAValue> {
        match self {
            SSAOp::In(_) => vec![],
            SSAOp::Out(val) | SSAOp::Hint(_, val) => vec![val],
            SSAOp::Assign(_, expr) => match expr {
                SSAExpr::Add(v1, v2)
                | SSAExpr::Sub(v1, v2)
                | SSAExpr::Mul(v1, v2)
                | SSAExpr::MulAdd(v1, v2, _) => vec![v1, v2],
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum SSAExpr {
    Add(SSAValue, SSAValue),
    Sub(SSAValue, SSAValue),
    Mul(SSAValue, SSAValue),
    MulAdd(SSAValue, SSAValue, u8),
}
