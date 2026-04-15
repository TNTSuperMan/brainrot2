use std::collections::HashMap;

mod find;
mod version_map;

use crate::{
    cfg::cfg::{CFG, CFGEdge, CFGOp, CFGOpKind},
    ssa::{
        builder::version_map::UniqueVersionMap,
        ssa::{Phi, SSABlock, SSAEdge, SSAExpr, SSAOp, SSAProgram, SSAVersion},
    },
};

struct PhiSchedule {
    phi_block: usize,
    pointer: isize,
}

pub struct SSABuilder<'a> {
    cfg: &'a CFG,
    phi_schedules: HashMap<usize, PhiSchedule>,
    pub program: SSAProgram,
    unique_ver_map: UniqueVersionMap,
}

impl<'a> SSABuilder<'a> {
    pub fn new(cfg: &'a CFG) -> SSABuilder<'a> {
        SSABuilder {
            cfg,
            phi_schedules: HashMap::new(),
            program: SSAProgram(vec![]),
            unique_ver_map: UniqueVersionMap::new(),
        }
    }
    pub fn parse_all(&mut self) {
        while self.program.0.len() != self.cfg.0.len() {
            self.step();
        }
    }
    fn alloc_ver(&mut self, pointer: isize) -> SSAVersion {
        SSAVersion {
            pointer,
            version: self.unique_ver_map.get_unique_version(pointer),
        }
    }
    fn step(&mut self) {
        let i = self.program.0.len();
        let cfg_block = &self.cfg.0[i];
        self.program.0.push(SSABlock {
            predecessor: cfg_block.predecessor.clone(),
            phis: vec![],
            edge: SSAEdge::End,
            insts: vec![],
            offset: cfg_block.offset,
        });

        for CFGOp {
            pointer: _ptr_ref,
            opcode,
            loc: _,
        } in &cfg_block.insts
        {
            let pointer = *_ptr_ref;
            macro_rules! def {
                ($expr: expr) => {
                    SSAOp::Define(self.alloc_ver(pointer), $expr)
                };
            }
            let ssaop = match opcode {
                CFGOpKind::Breakpoint => SSAOp::Breakpoint,
                CFGOpKind::Add(val) => def!(SSAExpr::AddVC(self.find(i, pointer), *val)),
                CFGOpKind::Set(val) => def!(SSAExpr::Const(*val)),
                CFGOpKind::MulAdd(p, val) => def!(SSAExpr::MulAdd(
                    self.find(i, pointer),
                    self.find(i, *p),
                    *val
                )),
                CFGOpKind::In => def!(SSAExpr::In),
                CFGOpKind::Out => SSAOp::Out(self.find(i, pointer)),
            };
            self.program.0[i].insts.push(ssaop);
        }

        self.program.0[i].edge = match cfg_block.edge {
            CFGEdge::Branch {
                pointer,
                zero,
                nonzero,
            } => SSAEdge::Branch {
                version: self.find(i, pointer),
                zero,
                nonzero,
            },
            CFGEdge::Jump(addr) => SSAEdge::Jump(addr),
            CFGEdge::End => SSAEdge::End,
        }
    }
}
