use crate::ssa::{
    builder::SSABuilder,
    ssa::{SSAOp, SSAVersion},
};

enum FindResult {
    None,
    Version(usize),
    Zero,
    FromCell,
    NeedPhi(usize),
}

impl<'a> SSABuilder<'a> {
    // blockはまだ読み込んでないとこも可能
    fn internal_find(&mut self, block: usize, pointer: isize) -> FindResult {
        if self.program.0.len() <= block {
            FindResult::None
        } else {
            let mut i = block;
            loop {
                let block = &self.program.0[i];
                if block.offset.is_some() {
                    return FindResult::FromCell;
                }
                for inst in block.insts.iter().rev() {
                    if let SSAOp::Define(ver, _) = inst {
                        if ver.pointer == pointer {
                            return FindResult::Version(ver.version);
                        }
                    }
                }
                match block.predecessor.as_slice() {
                    [] => return FindResult::Zero,
                    [pred1] => {
                        i = *pred1;
                        continue;
                    }
                    [pred1, pred2] => {
                        for phi in &block.phis {
                            if phi.pointer == pointer {
                                return FindResult::Version(phi.define_version);
                            }
                        }
                        return FindResult::NeedPhi(i);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    pub fn find(&mut self, block: usize, pointer: isize) -> SSAVersion {
        match self.internal_find(block, pointer) {
            FindResult::Version(version) => SSAVersion { pointer, version },
            _ => {
                todo!()
            }
        }
    }
}
