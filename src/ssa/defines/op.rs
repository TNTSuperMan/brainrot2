use std::fmt::Display;

use crate::ssa::defines::value::{SSAValue, SSAVersion};

#[derive(Clone, Debug)]
pub enum SSAOp {
    Out(SSAValue),
    In(SSAVersion),
    Assign(SSAVersion, SSAExpr),
    Hint(SSAVersion, SSAValue),
}

impl Display for SSAOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSAOp::Out(val) => write!(f, "stdout < {val}"),
            SSAOp::In(ver) => write!(f, "{ver} < stdin"),
            SSAOp::Assign(ver, expr) => write!(f, "{ver} = {expr}"),
            SSAOp::Hint(ver, val) => write!(f, "{ver} = {val} (hint)"),
        }
    }
}

impl SSAOp {
    pub fn reads(&self) -> Vec<i16> {
        let values = match self {
            SSAOp::In(_) => vec![],
            SSAOp::Out(val) | SSAOp::Hint(_, val) => vec![val],
            SSAOp::Assign(_, expr) => match expr {
                SSAExpr::Phi(vals) => vals.iter().collect(),
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
                SSAExpr::Phi(vals) => vals.iter_mut().collect(),
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
    Phi(Vec<SSAValue>),
    Add(SSAValue, SSAValue),
    Sub(SSAValue, SSAValue),
    Mul(SSAValue, SSAValue),
    MulAdd(SSAValue, SSAValue, u8),
}

impl Display for SSAExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSAExpr::Phi(vals) => {
                write!(
                    f,
                    "φ({})",
                    vals.iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            SSAExpr::Add(v1, v2) => write!(f, "{v1} + {v2}"),
            SSAExpr::Sub(v1, v2) => write!(f, "{v1} - {v2}"),
            SSAExpr::Mul(v1, v2) => write!(f, "{v1} * {v2}"),
            SSAExpr::MulAdd(v1, v2, v3) => write!(f, "{v1} + {v2} * {v3}"),
        }
    }
}
