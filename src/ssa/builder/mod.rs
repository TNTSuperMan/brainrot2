mod find;
mod version_map;

use crate::{
    cfg::cfg::{CFG, CFGEdge, CFGOp, CFGOpKind},
    ssa::{
        builder::{find::FindResult, version_map::UniqueVersionMap},
        ssa::{PhiArg, SSABlock, SSAEdge, SSAExpr, SSAOp, SSAProgram, SSAVersion},
    },
};

pub struct SSABuilder<'a> {
    cfg: &'a CFG,
    pub skipped_phis: Vec<(usize, usize, usize)>, // block index, phi index, pred index
    pub program: SSAProgram,
    unique_ver_map: UniqueVersionMap,
}

impl<'a> SSABuilder<'a> {
    pub fn new(cfg: &'a CFG) -> SSABuilder<'a> {
        SSABuilder {
            cfg,
            skipped_phis: vec![],
            program: SSAProgram(vec![]),
            unique_ver_map: UniqueVersionMap::new(),
        }
    }
    pub fn parse_all(&mut self) {
        while self.program.0.len() != self.cfg.0.len() {
            self.step();
        }
        self.resolve_skipped_phi();
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
                CFGOpKind::Add(val) => {
                    def!(match self.find(i, pointer) {
                        FindResult::Version(version) => SSAExpr::AddVC(version, *val),
                        FindResult::Zero => SSAExpr::Const(*val),
                    })
                }
                CFGOpKind::Set(val) => def!(SSAExpr::Const(*val)),
                CFGOpKind::MulAdd(p, val) => {
                    def!(match (self.find(i, pointer), self.find(i, *p)) {
                        (FindResult::Zero, FindResult::Zero) => SSAExpr::Const(0),
                        (FindResult::Version(v1), FindResult::Zero) => SSAExpr::Ref(v1),
                        (FindResult::Zero, FindResult::Version(v2)) => SSAExpr::Mul(v2, *val),
                        (FindResult::Version(v1), FindResult::Version(v2)) =>
                            SSAExpr::MulAdd(v1, v2, *val),
                    })
                }
                CFGOpKind::In => def!(SSAExpr::In),
                CFGOpKind::Out => SSAOp::Out(match self.find(i, pointer) {
                    FindResult::Version(version) => version,
                    FindResult::Zero => {
                        // コード初頭で初期値セルの0を出力するのはレアケースと考えたためパフォーマンスの問題は無視する
                        let zero_p = self.alloc_ver(pointer);
                        self.program.0[i]
                            .insts
                            .push(SSAOp::Define(zero_p, SSAExpr::Const(0)));
                        zero_p
                    }
                }),
            };
            self.program.0[i].insts.push(ssaop);
        }

        self.program.0[i].edge = match cfg_block.edge {
            CFGEdge::Branch {
                pointer,
                zero,
                nonzero,
            } => match self.find(i, pointer) {
                FindResult::Version(version) => SSAEdge::Branch {
                    version,
                    zero,
                    nonzero,
                },
                FindResult::Zero => SSAEdge::Jump(zero),
            },
            CFGEdge::Jump(addr) => SSAEdge::Jump(addr),
            CFGEdge::End => SSAEdge::End,
        }
    }
    fn resolve_skipped_phi(&mut self) {
        for (block_i, phi_i, pred_i) in self.skipped_phis.clone() {
            match self.find(self.program.0[block_i].predecessor[pred_i], self.program.0[block_i].phis[phi_i].pointer) {
                FindResult::Version(ver) => self.program.0[block_i].phis[phi_i].args[pred_i] = PhiArg::Version(ver.version),
                FindResult::Zero => todo!(),
            };
        }
    }
}
