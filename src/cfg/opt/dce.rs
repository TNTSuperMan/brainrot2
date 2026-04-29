use std::collections::HashSet;

use crate::cfg::cfg::{CFG, CFGEdge};

impl CFG {
    pub fn eliminate_dead_code(&mut self) {
        let mut dead_codes: HashSet<usize> = (0..(self.0.len())).collect();
        let mut stack = vec![0];

        while let Some(b) = stack.pop() {
            if !dead_codes.contains(&b) {
                break;
            }
            dead_codes.remove(&b);
            stack.append(&mut self.0[b].edge.successor());
        }

        for dead_i in dead_codes {
            self.0[dead_i].alive = false;
            self.0[dead_i].predecessor = vec![];
            self.0[dead_i].insts = vec![];
            self.update_edge(dead_i, CFGEdge::End);
        }
    }
}
