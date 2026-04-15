use crate::ssa::{
    builder::SSABuilder,
    ssa::{SSAOp, SSAVersion},
};

enum InternalFindResult {
    None,
    Version(usize),
    Zero,
    FromCell,
    NeedPhi(usize),
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
                        return InternalFindResult::NeedPhi(i);
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
