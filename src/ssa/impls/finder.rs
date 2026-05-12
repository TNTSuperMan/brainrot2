use crate::ssa::defines::{
    block::SSABlock,
    op::{SSAExpr, SSAOp},
    value::{SSAValue, SSAVersion},
};

pub struct Finder<'a, 'b> {
    blocks: &'a mut [SSABlock],
    last_version: &'b mut u32,
}

impl<'a, 'b> Finder<'a, 'b> {
    pub fn new(blocks: &'a mut [SSABlock], ver: &'b mut u32) -> Self {
        Self {
            blocks,
            last_version: ver,
        }
    }
    pub fn find(&mut self, block_i: usize, inst_i: usize, pointer: i16) -> SSAValue {
        if let Some(version) = self.blocks[block_i].find_def_from(pointer, inst_i) {
            return SSAValue::Version(version);
        }

        let preds = self.blocks[block_i].predecessor.clone();
        match preds.as_slice() {
            [] => SSAValue::Const(0),
            [p] => self.find(*p, self.blocks[*p].insts.len(), pointer),
            preds => {
                let version = SSAVersion {
                    pointer,
                    version: *self.last_version,
                };
                *self.last_version += 1;

                self.blocks[block_i].insts.insert(
                    0,
                    SSAOp::Assign(
                        version,
                        SSAExpr::Phi(
                            (0..preds.len())
                                .map(|_| {
                                    SSAValue::Version(SSAVersion {
                                        pointer,
                                        version: u32::MAX,
                                    })
                                })
                                .collect(),
                        ),
                    ),
                );

                SSAValue::Version(version)
            }
        }
    }
}
