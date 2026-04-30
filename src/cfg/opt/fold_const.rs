use crate::cfg::{cfg::{CFG, CFGOpKind}, opt::cellstate::CellState};

impl CFG {
    fn internal_get_cellstate_inblock(&self, block_i: usize, inst_i: usize, pointer: isize) -> CellState {
        let last_assign = self.0[block_i].insts[0..inst_i].iter().rev().find(|&inst| 
            !matches!(inst.opcode, CFGOpKind::Breakpoint|CFGOpKind::Out) && inst.pointer == pointer
        );
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

        let mut delete_schedules: Vec<usize> = vec![];

        for i in 0..self.0[block_i].insts.len() {
            let pointer = self.0[block_i].insts[i].pointer;
            match self.0[block_i].insts[i].opcode {
                CFGOpKind::Breakpoint => {},
                CFGOpKind::Add(val) => {
                    if let CellState::Const(v) = self.internal_get_cellstate_inblock(block_i, i, pointer) {
                        self.0[block_i].insts[i].opcode = CFGOpKind::Set(val.wrapping_add(v));
                    }
                }
                CFGOpKind::AddLoad(ptr) => {
                    if let CellState::Const(v) = self.internal_get_cellstate_inblock(block_i, i, ptr) {
                        self.0[block_i].insts[i].opcode = CFGOpKind::Add(v);
                    }
                    match (
                        self.internal_get_cellstate_inblock(block_i, i, pointer),
                        self.internal_get_cellstate_inblock(block_i, i, ptr),
                    ) {
                        (CellState::Const(v1), CellState::Const(v2)) => { // $p1 = v1 + v2
                            self.0[block_i].insts[i].opcode = CFGOpKind::Set(v1.wrapping_add(v2));
                        }
                        (CellState::Const(0), _) => { // $p1 = 0 + $p2
                            self.0[block_i].insts[i].opcode = CFGOpKind::SetLoad(ptr);
                        }
                        (CellState::Const(v1), _) => { // $p1 = v1 + $p2
                            // todo
                            //self.0[block_i].insts[i].opcode = CFGOpKind::MulAddConst(v1, p2, v3);
                        }
                        (_, CellState::Const(0)) => { // $p1 = $p1 + 0
                            delete_schedules.push(i);
                        }
                        (_, CellState::Const(v2)) => { // $p1 = $p1 + v2
                            self.0[block_i].insts[i].opcode = CFGOpKind::Add(v2);
                        }
                        _ => {}
                    }
                }
                CFGOpKind::Set(..) => {},
                CFGOpKind::SetLoad(ptr) => {
                    if let CellState::Const(v) = self.internal_get_cellstate_inblock(block_i, i, ptr) {
                        self.0[block_i].insts[i].opcode = CFGOpKind::Set(v);
                    }
                }
                CFGOpKind::MulAdd(p2, v3) => {
                    if v3 == 1 {
                        self.0[block_i].insts[i].opcode = CFGOpKind::AddLoad(p2);
                        continue;
                    }
                    if v3 == 255 {
                        self.0[block_i].insts[i].opcode = CFGOpKind::SubLoad(p2);
                        continue;
                    }
                    match (
                        self.internal_get_cellstate_inblock(block_i, i, pointer),
                        self.internal_get_cellstate_inblock(block_i, i, p2),
                    ) {
                        (CellState::Const(v1), CellState::Const(v2)) => {
                            self.0[block_i].insts[i].opcode = CFGOpKind::Set(v1.wrapping_add(v2.wrapping_mul(v3)));
                        }
                        (CellState::Const(0), _) => {
                            self.0[block_i].insts[i].opcode = CFGOpKind::Mul(p2, v3);
                        }
                        (CellState::Const(v1), _) => {
                            self.0[block_i].insts[i].opcode = CFGOpKind::MulAddConst(v1, p2, v3);
                        }
                        (_, CellState::Const(0)) => {
                            delete_schedules.push(i);
                        }
                        (_, CellState::Const(v2)) => {
                            self.0[block_i].insts[i].opcode = CFGOpKind::Add(v2.wrapping_mul(v3));
                        }
                        _ => {}
                    }
                }
                CFGOpKind::MulAddConst(v1, p2, v3) => {
                    if v3 == 1 {
                        self.0[block_i].insts[i].opcode = CFGOpKind::Set(v1);
                        continue;
                    }
                    if let CellState::Const(v2) = self.internal_get_cellstate_inblock(block_i, i, p2) {
                        self.0[block_i].insts[i].opcode = CFGOpKind::Set(v1.wrapping_add(v2.wrapping_mul(v3)));
                    }
                }
                CFGOpKind::Mul(p2, v3) => {
                    if v3 == 1 {
                        self.0[block_i].insts[i].opcode = CFGOpKind::SetLoad(p2);
                        continue;
                    }
                    if let CellState::Const(v2) = self.internal_get_cellstate_inblock(block_i, i, p2) {
                        self.0[block_i].insts[i].opcode = CFGOpKind::Set(v2.wrapping_mul(v3));
                    }
                }
                _ => {}
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
