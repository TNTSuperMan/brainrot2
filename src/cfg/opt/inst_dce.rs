use crate::cfg::cfg::{CFG, CFGOp};

impl CFG {
    fn internal_dce_inst(&mut self, block_i: usize) {
        let block = &mut self.0[block_i];
        if !block.alive { return }

        let mut i = 0usize;
        loop {
            if i >= block.insts.len() {
                break;
            }
            let ptr = if let CFGOp::Assign(ptr, _) = block.insts[i] {
                ptr
            } else {
                i += 1;
                continue;
            };
            let next_assign = i + 1 + match block.insts[(i+1)..].iter().position(|inst| inst.writes() == Some(ptr)) {
                Some(n) => n,
                None => {
                    i += 1;
                    continue;
                }
            };

            if block.insts[(i+1)..=next_assign].iter().all(|inst| !inst.reads().contains(&ptr)) {
                block.insts.remove(i);
                continue;
            }

            i += 1;
        }
    }
    pub fn eliminate_dead_instruction(&mut self) {
        for i in 0..(self.0.len()) {
            self.internal_dce_inst(i);
        }
    }
}
