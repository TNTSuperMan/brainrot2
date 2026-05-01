use crate::cfg::cfg::{CFGExpr, CFGOp, CFGValue};

impl CFGOp {
    pub fn reads(&self) -> Vec<isize> {
        match self {
            Self::Out(CFGValue::Const(_)) => vec![],
            
            Self::Breakpoint(p) |
            Self::Out(CFGValue::Load(p)) => vec![*p],

            Self::Assign(_, expr) => {
                let values: &[&CFGValue] = match expr {
                    CFGExpr::In => &[],

                    CFGExpr::Value(v) => &[v],

                    CFGExpr::Add(v1, v2) |
                    CFGExpr::Sub(v1, v2) |
                    CFGExpr::Mul(v1, v2) |
                    CFGExpr::MulAdd(v1, v2, _) => &[v1, v2],
                };
                let mut refs = vec![];
                for val in values {
                    if let CFGValue::Load(ptr) = val {
                        refs.push(*ptr);
                    }
                }
                refs
            }
        }
    }
    pub fn writes(&self) -> Option<isize> {
        if let CFGOp::Assign(ptr, _) = self {
            Some(*ptr)
        } else {
            None
        }
    }
}
