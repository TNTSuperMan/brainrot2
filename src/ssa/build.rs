use crate::{cfg::cfg::CFG, ssa::defines::program::SSAProgram};

pub fn build_ssa(cfg: &CFG) -> SSAProgram {
    let mut blocks = vec![];
    let mut block_offset = 0;

    let blocks = cfg.0.iter().map(|block| {
        
    })

    

    SSAProgram(blocks)
}
