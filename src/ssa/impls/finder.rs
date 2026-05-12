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
    pub fn find(&mut self, block_i: usize, pointer: i16) -> SSAValue {
        self.find_from(block_i, self.blocks[block_i].insts.len(), pointer)
    }
    pub fn find_from(&mut self, block_i: usize, inst_i: usize, pointer: i16) -> SSAValue {
        if let Some(value) = self.blocks[block_i].find_def_from(pointer, inst_i) {
            return value;
        }

        let preds = self.blocks[block_i].predecessor.clone();
        match preds.as_slice() {
            [] => SSAValue::Const(0),
            [p] => self.find(*p, pointer),
            preds => {
                let version = SSAVersion {
                    pointer,
                    version: *self.last_version,
                };
                *self.last_version += 1;

                self.blocks[block_i]
                    .phis
                    .insert(pointer, (version.version, vec![]));

                let actual_args: Vec<SSAValue> =
                    preds.iter().map(|p| self.find(*p, pointer)).collect();
                self.blocks[block_i]
                    .phis
                    .insert(pointer, (version.version, actual_args));

                SSAValue::Version(version)
            }
        }
    }
}
