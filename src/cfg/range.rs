use std::{cmp::{max, min}, collections::{HashMap, HashSet}, fmt::Debug, ops::RangeInclusive};

use crate::{TAPE_LENGTH, cfg::cfg::{CFG, CFGEdge}};

#[derive(Clone, Copy)]
pub struct OffsetRange {
    start: i16,
    end: u16,
}
impl Debug for OffsetRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OffsetRange {}..={}", self.start, self.end)
    }
}
impl From<RangeInclusive<i16>> for OffsetRange {
    fn from(value: RangeInclusive<i16>) -> Self {
        OffsetRange {
            start: 0 - *value.start(),
            end: ((TAPE_LENGTH - 1) as u16).wrapping_sub_signed(*value.end()),
        }
    }
}
impl OffsetRange {
    pub fn contains(&self, offset: i16) -> bool {
        self.start <= offset && (offset as i32) <= (self.end as i32)
    }
}

fn extend_range(range: Option<RangeInclusive<i16>>, point: i16) -> Option<RangeInclusive<i16>> {
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
    fn compute_access_range_from_edge(&self, block_i: usize) -> Option<RangeInclusive<i16>> {
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
            } else if block.offset.is_some() {
                if let Some(r) = self.compute_access_range_from_edge(b) {
                    map.insert(b, OffsetRange::from(r));
                }
            }
            dfs_stack.append(&mut block.edge.successor());
        }

        map
    }
}
