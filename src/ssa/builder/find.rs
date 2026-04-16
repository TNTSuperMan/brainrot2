use crate::ssa::{
    builder::SSABuilder,
    ssa::{Phi, PhiArg, SSAExpr, SSAOp, SSAVersion},
};

enum InternalFindResult {
    None,
    Version(usize),
    Zero,
    FromCell,
}

pub enum FindResult {
    Version(SSAVersion),
    Zero,
}

impl<'a> SSABuilder<'a> {
    // blockはまだ読み込んでないとこも可能
    fn internal_find(&mut self, block_i: usize, pointer: isize) -> InternalFindResult {
        if self.program.0.len() <= block_i {
            InternalFindResult::None
        } else {
            let mut i = block_i;
            loop {
                let block = &self.program.0[i];
                if i != block_i && block.offset.is_some() {
                    return InternalFindResult::FromCell;
                }
                for inst in block.insts.iter().rev() {
                    if let SSAOp::Define(ver, _) = inst {
                        if ver.pointer == pointer {
                            return InternalFindResult::Version(ver.version);
                        }
                    }
                }
                match block.predecessor.as_slice() {
                    [] => return InternalFindResult::Zero,
                    [pred1] => {
                        i = *pred1;
                        continue;
                    }
                    [pred1r, pred2r] => {
                        let pred1 = *pred1r;
                        let pred2 = *pred2r;
                        for phi in &block.phis {
                            if phi.pointer == pointer {
                                return InternalFindResult::Version(phi.define_version);
                            }
                        }
                        let ver = self.unique_ver_map.get_unique_version(pointer);
                        let phi = Phi {
                            pointer,
                            define_version: ver,
                            args: [PhiArg::Load, PhiArg::Load],
                        };
                        let phi_i = self.program.0[i].phis.len();
                        self.program.0[i].phis.push(phi);

                        let args: [PhiArg; 2] = [pred1, pred2]
                            .iter()
                            .map(|pred| match self.internal_find(*pred, pointer) {
                                InternalFindResult::Version(ver) => PhiArg::Version(ver),
                                InternalFindResult::FromCell => PhiArg::Load,
                                InternalFindResult::Zero => {
                                    let zero_v = self.alloc_ver(pointer);
                                    self.program.0[*pred]
                                        .insts
                                        .push(SSAOp::Define(zero_v, SSAExpr::Const(0)));
                                    PhiArg::Version(zero_v.version)
                                }
                                InternalFindResult::None => PhiArg::Version(usize::MAX), // 後に正しい値にする
                            })
                            .collect::<Vec<PhiArg>>()
                            .try_into()
                            .ok()
                            .unwrap();
                        if args[0] == args[1] {
                            if let PhiArg::Version(v) = args[0] {
                                self.program.0[i].phis.remove(phi_i);
                                self.program.0[i].insts.insert(
                                    0,
                                    SSAOp::Define(
                                        SSAVersion {
                                            pointer,
                                            version: ver,
                                        },
                                        SSAExpr::Ref(SSAVersion {
                                            pointer,
                                            version: v,
                                        }),
                                    ),
                                );
                                return InternalFindResult::Version(ver);
                            }
                        }
                        self.program.0[i].phis[phi_i].args = args;

                        return InternalFindResult::Version(ver);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    pub fn find(&mut self, block: usize, pointer: isize) -> FindResult {
        match self.internal_find(block, pointer) {
            InternalFindResult::Version(version) => {
                FindResult::Version(SSAVersion { pointer, version })
            }
            InternalFindResult::Zero => FindResult::Zero,
            _ => {
                todo!()
            }
        }
    }
}
