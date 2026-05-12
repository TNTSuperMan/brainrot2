use std::collections::HashSet;

use crate::ssa::defines::{
    block::SSABlock,
    op::{SSAExpr, SSAOp},
    value::{SSAValue, SSAVersion},
};

pub struct Finder<'a, 'b> {
    blocks: &'a mut [SSABlock],
    last_version: &'b mut u32,
    visited: HashSet<usize>,
    unresolves: HashSet<(usize, i16)>,
}

impl<'a, 'b> Finder<'a, 'b> {
    pub fn new(blocks: &'a mut [SSABlock], ver: &'b mut u32) -> Self {
        Self {
            blocks,
            last_version: ver,
            visited: HashSet::new(),
            unresolves: HashSet::new(),
        }
    }
    pub fn find(&mut self, block_i: usize, inst_i: usize, pointer: i16) -> SSAValue {
        if self.visited.contains(&block_i) {
            self.unresolves.insert((block_i, pointer));
            return SSAValue::Version(SSAVersion {
                pointer,
                version: u32::MAX,
            });
        }
        self.visited.insert(block_i);

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
                let phi = SSAExpr::Phi(
                    preds
                        .iter()
                        .map(|p| self.find(*p, self.blocks[*p].insts.len(), pointer))
                        .collect(),
                );
                let version = SSAVersion {
                    pointer,
                    version: *self.last_version,
                };

                self.blocks[block_i]
                    .insts
                    .insert(0, SSAOp::Assign(version, phi));

                *self.last_version += 1;

                SSAValue::Version(version)
            }
        }
    }
    pub fn solve(&mut self) {
        let unresolves = self.unresolves.clone();
        self.unresolves = HashSet::new();
        self.visited = HashSet::new();
        for (b, i) in unresolves {
            self.find(b, self.blocks[b].insts.len(), i);
        }
    }
}
