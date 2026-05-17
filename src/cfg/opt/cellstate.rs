use crate::cfg::cfg::{CFG, CFGEdge, CFGExpr, CFGOp, CFGValue};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Const(u8),
    NonZero,
    Unknown,
}

impl CFG {
    fn internal_get_cellstate(&self, from: usize, block_i: usize, pointer: i16, recursive_count: u8) -> CellState {
        if recursive_count > 5 {
            return CellState::Unknown;
        }

        let block = &self.0[block_i];

        let fallback = if let CFGEdge::Branch { pointer: b_pointer, zero, nonzero: _ } = &block.edge {
            if *b_pointer == pointer {
                if *zero == from {
                    return CellState::Const(0);
                }
                CellState::NonZero
            } else {
                CellState::Unknown
            }
        } else {
            CellState::Unknown
        };

        if block.offset.is_some() {
            return fallback;
        }
        if matches!(block.edge, CFGEdge::FindZeroAndJump { .. }) {
            return fallback;
        }

        let last_assign = block.insts.iter().rev().find(|&inst| inst.writes() == Some(pointer));
        if let Some(last_assign) = last_assign {
            return if let CFGOp::Assign(_, CFGExpr::Value(CFGValue::Const(c))) = last_assign {
                CellState::Const(*c)
            } else {
                fallback
            }
        }

        match self.internal_cellstate_recurse(block_i, pointer, recursive_count + 1) {
            CellState::Unknown => fallback,
            s => s,
        }
    }

    fn internal_cellstate_recurse(&self, block_i: usize, pointer: i16, recursive_count: u8) -> CellState {
        match self.0[block_i].predecessor.as_slice() {
            [] => {
                CellState::Const(0)
            }
            [pred] => {
                self.internal_get_cellstate(block_i, *pred, pointer, recursive_count + 1)
            }
            preds => {
                let (first, preds) = preds.split_first().unwrap();
                let state = self.internal_get_cellstate(block_i, *first, pointer, recursive_count + 1);
                if preds.iter().all(|p| state == self.internal_get_cellstate(block_i, *p, pointer, recursive_count + 1)) {
                    state
                } else {
                    CellState::Unknown
                }
            }
        }
    }

    pub fn get_cellstate(&self, block_i: usize, pointer: i16) -> CellState {
        self.internal_cellstate_recurse(block_i, pointer, 0)
    }
}
