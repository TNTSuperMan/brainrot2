use crate::cfg::{
    cfg::{CFG, CFGEdge},
    opt::cellstate::CellState,
};

impl CFG {
    fn internal_deep_one(&mut self, block_i: usize, from: usize) -> Option<usize> {
        if !self.0[block_i].insts.is_empty() || self.0[block_i].offset.is_some() {
            return None;
        }

        if let CFGEdge::Branch {
            pointer,
            zero,
            nonzero,
            ..
        }
        | CFGEdge::BranchWithIRAt {
            pointer,
            zero,
            nonzero,
            ..
        } = self.0[block_i].edge
        {
            match self.internal_get_cellstate(block_i, from, pointer, 0) {
                CellState::Const(0) => Some(zero),
                CellState::Const(_) | CellState::NonZero => Some(nonzero),
                CellState::Unknown => None,
            }
        } else {
            None
        }
    }
    fn internal_inline_deepbrach(&mut self, block_i: usize) {
        let mut branch = self.0[block_i].edge.clone();
        let (zero, nonzero) = match &mut branch {
            CFGEdge::Branch { zero, nonzero, .. }
            | CFGEdge::BranchWithIRAt { zero, nonzero, .. } => (zero, nonzero),
            _ => return,
        };

        if let Some(z) = self.internal_deep_one(*zero, block_i) {
            *zero = z;
        }
        if let Some(n) = self.internal_deep_one(*nonzero, block_i) {
            *nonzero = n;
        }

        self.update_edge(block_i, branch);
    }
    pub fn inline_deepbranch(&mut self) {
        for i in 0..self.0.len() {
            self.internal_inline_deepbrach(i);
        }
    }
}
