use crate::cfg::cfg::{CFG, CFGEdge};

impl CFG {
    fn internal_try_fold_info(&self, target: usize) -> Option<usize> {
        if self.0[target].insts.is_empty() && self.0[target].offset.is_none() {
            if let CFGEdge::Jump(to) = self.0[target].edge {
                return Some(to);
            }
        }
        None
    }
    fn internal_fold_jump(&mut self, block_i: usize) {
        if !self.0[block_i].alive { return }
        match self.0[block_i].edge {
            CFGEdge::Jump(block_to) => {
                if let Some(to) = self.internal_try_fold_info(block_to) {
                    self.update_edge(block_i, CFGEdge::Jump(to));
                }
            }
            CFGEdge::Branch { pointer, zero, nonzero } => {
                if let Some(zero) = self.internal_try_fold_info(zero) {
                    self.update_edge(block_i, CFGEdge::Branch { pointer, zero, nonzero });
                }
                if let Some(nonzero) = self.internal_try_fold_info(nonzero) {
                    self.update_edge(block_i, CFGEdge::Branch { pointer, zero, nonzero });
                }
            }
            CFGEdge::End => {}
        }
    }
    pub fn fold_jump(&mut self) {
        for i in 0..self.0.len() {
            self.internal_fold_jump(i);
        }
    }
}
