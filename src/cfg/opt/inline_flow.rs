use crate::cfg::cfg::{CFG, CFGEdge};

impl CFG {
    fn internal_inline(&mut self, block_i: usize) {
        if self.0[block_i].offset.is_some() {
            return;
        }
        if let CFGEdge::Jump(jump_to) = self.0[block_i].edge {
            if self.0[jump_to].offset.is_some() {
                return;
            }
            let mut target_insts = self.0[jump_to].insts.clone();
            self.0[block_i].insts.append(&mut target_insts);
            self.0[block_i].edge = self.0[jump_to].edge.clone();
        }
    }
    pub fn inline_flow(&mut self) {
        for i in 0..(self.0.len()) {
            self.internal_inline(i);
        }
    }
}
