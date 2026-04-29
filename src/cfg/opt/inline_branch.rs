use crate::cfg::cfg::{CFG, CFGEdge, CFGOpKind};

impl CFG {
    fn internal_inline_branch(&mut self, block_i: usize) {
        if !self.0[block_i].alive { return }

        let (pointer, zero, nonzero) = match self.0[block_i].edge {
            CFGEdge::Jump(..) => return,
            CFGEdge::Branch { pointer, zero, nonzero } => (pointer + self.0[block_i].offset.unwrap_or(0), zero, nonzero),
            CFGEdge::End => return,
        };

        let last_assign = self.0[block_i].insts.iter().rev().find(|&inst| inst.pointer == pointer);
        if let Some(last_assign) = last_assign {
            if let CFGOpKind::Set(val) = last_assign.opcode {
                self.0[block_i].edge = CFGEdge::Jump(if val == 0 {
                    zero
                } else {
                    nonzero
                });
            }
            return;
        }
        if self.is_zero_cell(block_i, pointer) {
            self.0[block_i].edge = CFGEdge::Jump(zero);
        }
    }
    pub fn inline_branch(&mut self) {
        for i in 0..self.0.len() {
            self.internal_inline_branch(i);
        }
    }
}
