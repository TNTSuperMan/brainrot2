use crate::cfg::cfg::{CFGOp, CFGOpKind};

impl CFGOp {
    pub fn reads(&self) -> Vec<isize> {
        match &self.opcode {
            CFGOpKind::Add(_) |
            CFGOpKind::Out => vec![self.pointer],

            CFGOpKind::AddLoad(p) |
            CFGOpKind::SubLoad(p) |
            CFGOpKind::MulAdd(p, _) => vec![self.pointer, *p],

            CFGOpKind::SetLoad(p) |
            CFGOpKind::MulAddConst(_, p, _) |
            CFGOpKind::Mul(p, _) => vec![*p],

            CFGOpKind::Breakpoint |
            CFGOpKind::Set(_) |
            CFGOpKind::In |
            CFGOpKind::OutConst(_) => vec![],
        }
    }
    pub fn writes(&self) -> Option<isize> {
        if matches!(&self.opcode, CFGOpKind::Breakpoint | CFGOpKind::Out | CFGOpKind::OutConst(_)) {
            None
        } else {
            Some(self.pointer)
        }
    }
}
