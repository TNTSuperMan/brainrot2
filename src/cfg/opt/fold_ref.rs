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
                    for inst in &block.insts[assign_i..i] {
                        if let Some(w) = inst.writes() {
                            if assign_reads.contains(&w) {
                                continue 'root_loop;
                            }
                        }
                    }
                    let expr = if let CFGOp::Assign(_, expr) = &block.insts[assign_i] {
                        expr
                    } else {
                        unreachable!();
                    };
                    if let CFGExpr::Value(CFGValue::Load(ptr)) = expr {
                        block.insts[i] = CFGOp::Assign(pointer, CFGExpr::Value(CFGValue::Load(*ptr)));
                        continue;
                    }
                    if let Some(next) = block.insts[(i+1)..].iter().find(|inst| inst.writes() == Some(ptr)) {
                        if !next.reads().contains(&pointer) && !next.reads().contains(&ptr) {
                            block.insts[i] = CFGOp::Assign(pointer, expr.clone());
                        }
                    }
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
