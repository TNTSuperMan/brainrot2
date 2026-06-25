use crate::cfg::{
    cfg::{CFGBlock, CFGEdge},
    range::range::{AccessRange, extend_range},
};

impl CFGBlock {
    pub fn extend_access_range(&self, range: &mut Option<AccessRange>) {
        for inst in &self.insts {
            for read in inst.reads() {
                *range = extend_range(range, read);
            }
            if let Some(write) = inst.writes() {
                *range = extend_range(range, write);
            }
        }
        if self.has_offset() {
            return;
        }
        if let CFGEdge::Branch { pointer, .. } | CFGEdge::BranchWithIRAt { pointer, .. } =
            &self.edge
        {
            *range = extend_range(range, *pointer);
        }
    }
    pub fn has_offset(&self) -> bool {
        self.offset.is_some() || matches!(self.edge, CFGEdge::FindZeroAndJump { .. })
    }
}
