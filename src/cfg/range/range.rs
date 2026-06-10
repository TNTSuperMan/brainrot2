use std::{cmp::{max, min}, fmt::Debug, ops::RangeInclusive};

use crate::TAPE_LENGTH;

pub type AccessRange = RangeInclusive<i16>;

pub fn extend_range(range: &Option<AccessRange>, point: i16) -> Option<AccessRange> {
    Some(if let Some(r) = range {
        (min(*r.start(), point))..=(max(*r.end(), point))
    } else {
        point..=point
    })
}

#[derive(Clone, Copy)]
pub struct OffsetRange {
    start: i16,
    end: u16,
}
impl Debug for OffsetRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..={}", self.start, self.end)
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
    pub fn contains(&self, offset: i32) -> bool {
        (self.start as i32) <= offset && offset <= (self.end as i32)
    }
}
