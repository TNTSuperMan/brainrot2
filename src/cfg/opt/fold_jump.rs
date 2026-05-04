use crate::cfg::cfg::{CFG, CFGEdge};

impl CFG {
    fn internal_try_fold_info(&self, target: usize) -> Option<CFGEdge> {
        if self.0[target].insts.is_empty() && self.0[target].offset.is_none() {
            Some(self.0[target].edge.clone())
        } else {
            None
        }
    }
    fn internal_fold_jump(&mut self, block_i: usize) {
        if !self.0[block_i].alive { return }
        match self.0[block_i].edge {
            CFGEdge::Jump(block_to) => {
                if let Some(CFGEdge::Jump(to)) = self.internal_try_fold_info(block_to) {
                    self.update_edge(block_i, CFGEdge::Jump(to));
                }
            }
            CFGEdge::Branch { pointer, zero, nonzero } |
            CFGEdge::BranchWithIRAt { pointer, zero, nonzero, ir_at: _ } => {
                if let Some(edge) = self.internal_try_fold_info(zero) {
                    match edge {
                        CFGEdge::Jump(zero) => self.update_edge(block_i, CFGEdge::Branch { pointer, zero, nonzero }),
                        CFGEdge::Branch { pointer: edge_p, zero, nonzero: _ } |
                        CFGEdge::BranchWithIRAt { pointer: edge_p, zero, nonzero: _, ir_at: _ } => {
                            if edge_p == pointer {
                                self.update_edge(block_i, CFGEdge::Branch { pointer, zero, nonzero })
                            }
                        }
                        CFGEdge::End => {}
                    }
                }
                if let Some(edge) = self.internal_try_fold_info(nonzero) {
                    match edge {
                        CFGEdge::Jump(nonzero) => self.update_edge(block_i, CFGEdge::Branch { pointer, zero, nonzero }),
                        CFGEdge::Branch { pointer: edge_p, zero: _, nonzero } |
                        CFGEdge::BranchWithIRAt { pointer: edge_p, zero: _, nonzero, ir_at: _ } => {
                            if edge_p == pointer {
                                self.update_edge(block_i, CFGEdge::Branch { pointer, zero, nonzero })
                            }
                    }
                        CFGEdge::End => {}
                    }
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
