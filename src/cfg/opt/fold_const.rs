use crate::cfg::{cfg::{CFG, CFGOpKind}, opt::cellstate::CellState};

impl CFG {
    fn internal_get_cellstate_inblock(&self, block_i: usize, inst_i: usize, pointer: isize) -> CellState {
        let last_assign = self.0[block_i].insts[0..inst_i].iter().rev().find(|&inst| inst.pointer == pointer);
        if let Some(last_assign) = last_assign {
            return if let CFGOpKind::Set(c) = last_assign.opcode {
                CellState::Const(c)
            } else {
                CellState::Unknown
            }
        }

        self.get_cellstate(block_i, pointer)
    }
    fn internal_fold_const(&mut self, block_i: usize) {
        if !self.0[block_i].alive { return }

        let mut change_schedules = vec![];

        for (i, inst) in self.0[block_i].insts.iter().enumerate() {
            match inst.opcode {
                CFGOpKind::Breakpoint => {},
                CFGOpKind::Add(val) => {
                    if let CellState::Const(v) = self.internal_get_cellstate_inblock(block_i, i, inst.pointer) {
                        change_schedules.push((i, Some(CFGOpKind::Set(val.wrapping_add(v)))));
                    }
                }
                CFGOpKind::AddLoad(ptr) => {
                    if let CellState::Const(v) = self.internal_get_cellstate_inblock(block_i, i, ptr) {
                        change_schedules.push((i, Some(CFGOpKind::Add(v))));
                    }
                }
                CFGOpKind::Set(..) => {},
                CFGOpKind::SetLoad(ptr) => {
                    if let CellState::Const(v) = self.internal_get_cellstate_inblock(block_i, i, ptr) {
                        change_schedules.push((i, Some(CFGOpKind::Set(v))));
                    }
                }
                CFGOpKind::MulAdd(p2, v3) => {
                    if v3 == 1 {
                        change_schedules.push((i, Some(CFGOpKind::AddLoad(p2))));
                        continue;
                    }
                    match (
                        self.internal_get_cellstate_inblock(block_i, i, inst.pointer),
                        self.internal_get_cellstate_inblock(block_i, i, p2),
                    ) {
                        (CellState::Const(v1), CellState::Const(v2)) => {
                            change_schedules.push((i, Some(CFGOpKind::Set(v1.wrapping_add(v2.wrapping_mul(v3))))));
                        }
                        (CellState::Const(0), _) => {
                            change_schedules.push((i, Some(CFGOpKind::Mul(p2, v3))));
                        }
                        (CellState::Const(v1), _) => {
                            change_schedules.push((i, Some(CFGOpKind::MulAddConst(v1, p2, v3))));
                        }
                        (_, CellState::Const(0)) => {
                            change_schedules.push((i, None));
                        }
                        (_, CellState::Const(v2)) => {
                            change_schedules.push((i, Some(CFGOpKind::Add(v2.wrapping_mul(v3)))));
                        }
                        _ => {}
                    }
                }
                CFGOpKind::MulAddConst(v1, p2, v3) => {
                    if v3 == 1 {
                        change_schedules.push((i, Some(CFGOpKind::Set(v1))));
                        continue;
                    }
                    if let CellState::Const(v2) = self.internal_get_cellstate_inblock(block_i, i, p2) {
                        change_schedules.push((i, Some(CFGOpKind::Set(v1.wrapping_add(v2.wrapping_mul(v3))))));
                    }
                }
                CFGOpKind::Mul(p2, v3) => {
                    if v3 == 1 {
                        change_schedules.push((i, Some(CFGOpKind::SetLoad(p2))));
                        continue;
                    }
                    if let CellState::Const(v2) = self.internal_get_cellstate_inblock(block_i, i, p2) {
                        change_schedules.push((i, Some(CFGOpKind::Set(v2.wrapping_mul(v3)))));
                    }
                }
                _ => {}
            }
        }

        let mut offset = 0;
        for (ci, change) in change_schedules {
            if let Some(c) = change {
                self.0[block_i].insts[ci - offset].opcode = c;
            } else {
                self.0[block_i].insts.remove(ci - offset);
                offset += 1;
            }
        }
    }
    pub fn fold_const(&mut self) {
        for i in 0..self.0.len() {
            self.internal_fold_const(i);
        }
    }
}
