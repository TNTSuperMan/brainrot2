use crate::cfg::cfg::{CFG, CFGExpr, CFGOp, CFGValue};

impl CFG {
    fn internal_fold_ref(&mut self, block_i: usize) {
        let block = &mut self.0[block_i];
        if !block.alive { return }
        if block.insts.is_empty() { return }

        'root_loop: for i in 1..block.insts.len() {
            match block.insts[i] {
                CFGOp::Assign(pointer, CFGExpr::Value(CFGValue::Load(ptr))) => {
                    let assign_i = i - 1 - match block.insts[..i].iter().rev().position(|inst| inst.writes() == Some(ptr)) {
                        Some(i) => i,
                        None => continue,
                    };
                    let assign_reads = block.insts[assign_i].reads();
                    for inst in &block.insts[(assign_i+1)..i] {
                        if let Some(w) = inst.writes() {
                            if assign_reads.contains(&w) {
                                continue 'root_loop;
                            }
                        }
                    }
                    block.insts[i] = CFGOp::Assign(pointer, match block.insts[assign_i] {
                        CFGOp::Assign(_, CFGExpr::Value(CFGValue::Load(ptr))) => CFGExpr::Value(CFGValue::Load(ptr)),
                        _ => continue,
                    });
                }
                _ => {}
            }
        }
    }
    pub fn fold_ref(&mut self) {
        for i in 0..self.0.len() {
            self.internal_fold_ref(i);
        }
    }
}
