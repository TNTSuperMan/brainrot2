use crate::cfg::{cfg::{CFG, CFGExpr, CFGOp, CFGValue}, opt::cellstate::CellState};

impl CFG {
    fn internal_get_cellstate_inblock(&self, block_i: usize, inst_i: usize, pointer: i16) -> CellState {
        let last_assign = self.0[block_i].insts[0..inst_i].iter().rev().find(|&inst| inst.writes() == Some(pointer));
        if let Some(last_assign) = last_assign {
            return if let CFGOp::Assign(_, CFGExpr::Value(CFGValue::Const(c))) = last_assign {
                CellState::Const(*c)
            } else {
                CellState::Unknown
            }
        }

        self.get_cellstate(block_i, pointer)
    }
    fn internal_simply_fold_const(&self, block_i: usize, inst_i: usize, val: &mut CFGValue) {
        if let CFGValue::Load(ptr) = *val {
            if let CellState::Const(c) = self.internal_get_cellstate_inblock(block_i, inst_i, ptr) {
                *val = CFGValue::Const(c);
            }
        }
    }
    fn internal_fold_const(&mut self, block_i: usize) {
        if !self.0[block_i].alive { return }

        let mut delete_schedules: Vec<usize> = vec![];

        for i in 0..self.0[block_i].insts.len() {
            
            let mut newop = self.0[block_i].insts[i].clone();
            match &mut newop {
                CFGOp::Out(val) => {
                    self.internal_simply_fold_const(block_i, i, val);
                }
                CFGOp::Assign(_, expr) => match expr {
                    CFGExpr::In => {},

                    CFGExpr::Value(val) => {
                        self.internal_simply_fold_const(block_i, i, val);
                    },
                    CFGExpr::Add(v1, v2) |
                    CFGExpr::Sub(v1, v2) |
                    CFGExpr::Mul(v1, v2) |
                    CFGExpr::MulAdd(v1, v2, _) => {
                        self.internal_simply_fold_const(block_i, i, v1);
                        self.internal_simply_fold_const(block_i, i, v2);
                    }
                }
            }
            self.0[block_i].insts[i] = newop;

            if let CFGOp::Assign(pointer, expr) = &mut self.0[block_i].insts[i] {
                let pointer = *pointer;
                match expr.clone() {
                    CFGExpr::Value(CFGValue::Load(ptr)) => {
                        if pointer == ptr {
                            delete_schedules.push(i);
                        }
                    }
                    CFGExpr::Add(CFGValue::Const(v1), CFGValue::Const(v2)) => {
                        *expr = CFGExpr::Value(CFGValue::Const(v1.wrapping_add(v2)))
                    }
                    CFGExpr::Add(CFGValue::Const(0), CFGValue::Load(p)) |
                    CFGExpr::Add(CFGValue::Load(p), CFGValue::Const(0)) => {
                        if pointer == p {
                            delete_schedules.push(i);
                        } else {
                            *expr = CFGExpr::Value(CFGValue::Load(p));
                        }
                    }
                    CFGExpr::MulAdd(v1, v2, 1) => {
                        *expr = CFGExpr::Add(v1, v2);
                    }
                    CFGExpr::MulAdd(v1, v2, 255) => {
                        *expr = CFGExpr::Sub(v1, v2);
                    }
                    CFGExpr::MulAdd(CFGValue::Const(0), v2, v3) => {
                        *expr = CFGExpr::Mul(v2, CFGValue::Const(v3));
                    }
                    CFGExpr::MulAdd(v1, CFGValue::Const(0), _) |
                    CFGExpr::MulAdd(v1, _, 0) => {
                        *expr = CFGExpr::Value(v1);
                    }
                    CFGExpr::MulAdd(v1, CFGValue::Const(c2), c3) => {
                        *expr = CFGExpr::Add(v1, CFGValue::Const(c2.wrapping_mul(c3)));
                    }

                    CFGExpr::Mul(CFGValue::Const(v1), CFGValue::Const(v2)) => {
                        *expr = CFGExpr::Value(CFGValue::Const(v1.wrapping_mul(v2)));
                    }
                    CFGExpr::Mul(CFGValue::Const(c1), CFGValue::Load(p2)) |
                    CFGExpr::Mul(CFGValue::Load(p2), CFGValue::Const(c1)) => {
                        match c1 {
                            0 => delete_schedules.push(i),
                            1 => *expr = CFGExpr::Value(CFGValue::Load(p2)),
                            255 => *expr = CFGExpr::Sub(CFGValue::Const(0), CFGValue::Load(p2)),
                            _ => {}
                        }
                    }
                    
                    _ => {}
                }
            }
        }

        let mut offset = 0;
        for ci in delete_schedules {
            self.0[block_i].insts.remove(ci - offset);
            offset += 1;
        }
    }
    pub fn fold_const(&mut self) {
        for i in 0..self.0.len() {
            self.internal_fold_const(i);
        }
    }
}
