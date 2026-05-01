use std::{cmp::{max, min}, collections::{HashMap, HashSet}, ops::RangeInclusive};

use crate::cfg::cfg::{CFG, CFGEdge};

fn extend_range(range: Option<RangeInclusive<isize>>, point: isize) -> Option<RangeInclusive<isize>> {
    Some(if let Some(r) = range {
        (
            min(*r.start(), point)
        )..=(
            max(*r.end(), point)
        )
    } else {
        point..=point
    })
}

fn accessrange_to_offsetrange(range: RangeInclusive<isize>) -> RangeInclusive<isize> {
    (
        0 - range.start()
    )..=(
        65535 - range.end()
    )
}

impl CFG {
    fn compute_access_range(&self, block_i: usize) -> Option<RangeInclusive<isize>> {
        let mut dfs_stack = vec![block_i];
        let mut range = None;
        let mut visited = HashSet::new();

        while let Some(b) = dfs_stack.pop() {
            if visited.contains(&b) {
                continue;
            }
            visited.insert(b);
            let block = &self.0[b];

            for inst in &block.insts {
                for read in inst.reads() {
                    range = extend_range(range, read);
                }
                if let Some(write) = inst.writes() {
                    range = extend_range(range, write);
                }
            }
            if block.offset.is_some() {
                continue;
            }
            if let CFGEdge::Branch { pointer, zero: _, nonzero: _ } = &block.edge {
                range = extend_range(range, *pointer);
            }

            dfs_stack.append(&mut block.edge.successor());
        }

        range
    }
    fn compute_access_range_from_edge(&self, block_i: usize) -> Option<RangeInclusive<isize>> {
        let mut range = None;
        let block = &self.0[block_i];
        if let CFGEdge::Branch { pointer, zero: _, nonzero: _ } = &block.edge {
            range = extend_range(range, *pointer);
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
    pub fn compute_offset_ranges(&self) -> HashMap<usize, RangeInclusive<isize>> {
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
                    map.insert(0, accessrange_to_offsetrange(r));
                }
            } else if block.offset.is_some() {
                if let Some(r) = self.compute_access_range_from_edge(b) {
                    map.insert(b, accessrange_to_offsetrange(r));
                }
            }
            dfs_stack.append(&mut block.edge.successor());
        }

        map
    }
}
