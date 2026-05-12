use std::{collections::HashMap, fmt::Display};

use crate::ssa::defines::{op::SSAOp, value::SSAVersion};

#[derive(Clone, Debug)]
pub struct SSABlock {
    pub alive: bool,
    pub predecessor: Vec<usize>,
    pub phis: HashMap<i16, (usize, Vec<SSAVersion>)>,
    pub insts: Vec<SSAOp>,
    pub offset: Option<i16>,
    pub edge: SSAEdge,
}

impl Display for SSABlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.alive {
            write!(f, "SSABlock <dead>")
        } else {
            write!(f, "SSABlock preds: {:?} {{\n", self.predecessor)?;
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

#[derive(Clone, Debug)]
pub enum SSAEdge {
    Jump(usize),
    Branch {
        version: SSAVersion,
        zero: usize,
        nonzero: usize,
        ir_at: Option<usize>,
    },
    End,
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
            SSAEdge::End => write!(f, "end"),
        }
    }
}
