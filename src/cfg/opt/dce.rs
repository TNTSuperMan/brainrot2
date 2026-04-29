use std::collections::HashSet;

use crate::cfg::cfg::{CFG, CFGEdge};

impl CFG {
    pub fn eliminate_dead_code(&mut self) {
        let mut dead_codes: HashSet<usize> = (1..(self.0.len())).collect();

        for block in &self.0 {
            if block.alive {
                match block.edge {
                    CFGEdge::Jump(to) => {
                        dead_codes.remove(&to);
                    },
                    CFGEdge::Branch { pointer: _, zero, nonzero } => {
                        dead_codes.remove(&zero);
                        dead_codes.remove(&nonzero);
                    }
                    CFGEdge::End => {}
                }
            }
        }

        for dead_i in dead_codes {
            self.0[dead_i].alive = false;
            self.0[dead_i].predecessor = vec![];
            self.0[dead_i].insts = vec![];
            self.update_edge(dead_i, CFGEdge::End);
        }
    }
}
