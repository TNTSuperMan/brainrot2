use crate::cfg::cfg::{CFG, CFGEdge, CFGOpKind};

impl CFG {
    fn internal_is_zero_cell(&self, from: usize, block_i: usize, pointer: isize, recursive_count: u8) -> bool {
        if recursive_count > 2 {
            return false;
        }

        let block = &self.0[block_i];

        if let CFGEdge::Branch { pointer: b_pointer, zero, nonzero } = &block.edge {
            if *b_pointer == pointer && *zero == from {
                return true;
            }
        }

        let last_assign = block.insts.iter().rev().find(|&inst| inst.pointer == pointer);
        if let Some(last_assign) = last_assign {
            if last_assign.opcode == CFGOpKind::Set(0) {
                return true;
            } else {
                return false;
            }
        }

        block.predecessor.iter().all(|p| {
            self.internal_is_zero_cell(block_i, *p, pointer, recursive_count + 1)
        })
    }

    pub fn is_zero_cell(&self, block_i: usize, pointer: isize) -> bool {
        self.0[block_i].predecessor.iter().all(|p| {
            self.internal_is_zero_cell(block_i, *p, pointer, 0)
        })
    }
}
