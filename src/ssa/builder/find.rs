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
    fn internal_find(&mut self, block: usize, pointer: isize) -> InternalFindResult {
        if self.program.0.len() <= block {
            InternalFindResult::None
        } else {
            let mut i = block;
            loop {
                let block = &self.program.0[i];
                if block.offset.is_some() {
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
                    [pred1, pred2] => {
                        for phi in &block.phis {
                            if phi.pointer == pointer {
                                return InternalFindResult::Version(phi.define_version);
                            }
                        }
                        let ver = self.unique_ver_map.get_unique_version(pointer);
                        let args = [*pred1, *pred2]
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
                                _ => todo!(),
                            })
                            .collect::<Vec<PhiArg>>();
                        let phi = Phi {
                            pointer,
                            define_version: ver,
                            args: match args.try_into() {
                                Ok(args) => args,
                                Err(_) => unreachable!(),
                            },
                        };
                        self.program.0[i].phis.push(phi);
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
