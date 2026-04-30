use crate::cfg::cfg::{CFG, CFGOp, CFGOpKind};

impl CFGOp {
    fn is_references(&self, ptr: isize) -> bool {
        match &self.opcode {
            CFGOpKind::Add(_) |
            CFGOpKind::Out => self.pointer == ptr,

            CFGOpKind::AddLoad(p) |
            CFGOpKind::SubLoad(p) |
            CFGOpKind::MulAdd(p, _) => self.pointer == ptr || *p == ptr,

            CFGOpKind::SetLoad(p) |
            CFGOpKind::MulAddConst(_, p, _) |
            CFGOpKind::Mul(p, _) => *p == ptr,

            CFGOpKind::Breakpoint |
            CFGOpKind::Set(_) |
            CFGOpKind::In |
            CFGOpKind::OutConst(_) => false,
        }
    }
    fn is_assign_to(&self, ptr: isize) -> bool {
        !matches!(&self.opcode, CFGOpKind::Breakpoint | CFGOpKind::Out | CFGOpKind::OutConst(_)) && self.pointer == ptr
    }
}

impl CFG {
    fn internal_dce_inst(&mut self, block_i: usize) {
        let block = &mut self.0[block_i];
        if !block.alive { return }

        let mut i = 0usize;
        loop {
            if i >= block.insts.len() {
                break;
            }
            if matches!(&block.insts[i].opcode, CFGOpKind::Breakpoint | CFGOpKind::Out | CFGOpKind::OutConst(_)) {
                i += 1;
                continue;
            }
            let ptr = block.insts[i].pointer;
            let next_assign = i + 1 + match block.insts[(i+1)..].iter().position(|inst| inst.is_assign_to(ptr)) {
                Some(n) => n,
                None => {
                    i += 1;
                    continue;
                }
            };

            if block.insts[(i+1)..=next_assign].iter().all(|inst| !inst.is_references(ptr)) {
                block.insts.remove(i);
                continue;
            }

            i += 1;
        }
    }
    pub fn eliminate_dead_instruction(&mut self) {
        for i in 0..(self.0.len()) {
            self.internal_dce_inst(i);
        }
    }
}
