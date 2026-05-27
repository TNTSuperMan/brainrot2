use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use crate::{
    cfg::{cfg::{CFG, CFGEdge}, range::range::extend_range},
};

mod block;
mod range;

pub use range::OffsetRange;

impl CFG {
    fn compute_access_range(&self, block_i: usize) -> Option<RangeInclusive<i16>> {
        let mut dfs_stack = vec![block_i];
        let mut range = None;
        let mut visited = HashSet::new();

        while let Some(b) = dfs_stack.pop() {
            if visited.contains(&b) {
                continue;
            }
            visited.insert(b);
            let block = &self.0[b];
            
            block.extend_access_range(&mut range);

            if !block.has_offset() {
                dfs_stack.append(&mut block.edge.successor());
            }
        }

        range
    }
    fn compute_access_range_from_edge(&self, block_i: usize) -> Option<RangeInclusive<i16>> {
        let mut range = None;
        let block = &self.0[block_i];
        if let CFGEdge::Branch {
            pointer, ..
        } | CFGEdge::BranchWithIRAt {
            pointer, ..
        } = &block.edge
        {
            range = extend_range(&range, *pointer);
        }

        for succ in block.edge.successor() {
            let r = self.compute_access_range(succ);
            if let Some(r) = r {
                if let Some(range) = &mut range {
                    *range = min(*range.start(), *r.start())..=max(*range.end(), *r.end());
                } else {
                    range = Some(r);
                }
            }
        }

        range
    }
    pub fn compute_offset_ranges(&self) -> HashMap<usize, OffsetRange> {
        let mut map = HashMap::new();
        let mut visited = HashSet::new();

        let mut dfs_stack = vec![0];
        while let Some(b) = dfs_stack.pop() {
            if visited.contains(&b) {
                continue;
            }
            visited.insert(b);
            let block = &self.0[b];
            if b == 0 {
                if let Some(r) = self.compute_access_range(0) {
                    map.insert(0, OffsetRange::from(r));
                }
            } else if block.offset.is_some()
                || matches!(block.edge, CFGEdge::FindZeroAndJump { .. })
            {
                if let Some(r) = self.compute_access_range_from_edge(b) {
                    map.insert(b, OffsetRange::from(r));
                }
            }
            dfs_stack.append(&mut block.edge.successor());
        }

        map
    }
}
