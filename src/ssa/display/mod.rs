use std::fmt::Display;

use crate::ssa::defines::{
    block::{SSABlock, SSAEdge},
    op::{SSAExpr, SSAOp},
    program::SSAProgram,
    value::{SSAValue, SSAVersion},
};

impl Display for SSAProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSAProgram len: {}", self.0.len())?;
        for (i, block) in self.0.iter().enumerate() {
            write!(f, "n{i}: {block}\n")?;
        }
        Ok(())
    }
}

impl Display for SSABlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.alive {
            write!(f, "SSABlock <dead>")
        } else {
            write!(f, "SSABlock preds: {:?} {{\n", self.predecessor)?;
            for (ptr, (ver, args)) in &self.phis {
                write!(
                    f,
                    "    ${ptr}#{ver} = φ({})\n",
                    args.iter()
                        .map(|a| format!("{a}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
            }
            for inst in &self.insts {
                write!(f, "    {inst}\n")?;
            }
            if let Some(offset) = self.offset {
                write!(f, "    offset {offset}\n")?;
            }
            write!(f, "    {}\n}}", self.edge)
        }
    }
}
impl Display for SSAEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSAEdge::Jump(to) => write!(f, "jump n{to}"),
            SSAEdge::Branch {
                version,
                zero,
                nonzero,
                ir_at,
            } => {
                write!(f, "branch {version} ? n{nonzero} : n{zero}")?;
                if let Some(ir) = ir_at {
                    write!(f, " (ir at {ir})")
                } else {
                    Ok(())
                }
            }
            SSAEdge::BranchLoad {
                pointer,
                zero,
                nonzero,
                ir_at,
            } => {
                write!(f, "branch load {pointer} ? n{nonzero} : n{zero}")?;
                if let Some(ir) = ir_at {
                    write!(f, " (ir at {ir})")
                } else {
                    Ok(())
                }
            }
            SSAEdge::End => write!(f, "end"),
        }
    }
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
impl Display for SSAExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSAExpr::Add(v1, v2) => write!(f, "{v1} + {v2}"),
            SSAExpr::Sub(v1, v2) => write!(f, "{v1} - {v2}"),
            SSAExpr::Mul(v1, v2) => write!(f, "{v1} * {v2}"),
            SSAExpr::MulAdd(v1, v2, v3) => write!(f, "{v1} + {v2} * {v3}"),
        }
    }
}

impl Display for SSAValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSAValue::Version(ver) => write!(f, "{ver}"),
            SSAValue::Const(val) => write!(f, "{val}"),
            SSAValue::Load(ptr) => write!(f, "load ${ptr}"),
        }
    }
}

impl Display for SSAVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}#{}", self.pointer, self.version)
    }
}
