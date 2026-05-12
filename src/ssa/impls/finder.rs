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
        let insts = &self.blocks[block_i].insts[..inst_i];

        for (i, inst) in insts.iter().enumerate().rev() {
            match inst {
                SSAOp::Out(_) => {}
                SSAOp::In(ver) | SSAOp::Assign(ver, _) => {
                    if ver.pointer == pointer {
                        return SSAValue::Version(*ver);
                    }
                }
                SSAOp::Hint(ver, val) => {
                    if ver.pointer == pointer {
                        return if let SSAValue::Version(SSAVersion {
                            pointer,
                            version: u32::MAX,
                        }) = val
                        {
                            self.find(block_i, i, *pointer)
                        } else {
                            *val
                        };
                    }
                }
            }
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
