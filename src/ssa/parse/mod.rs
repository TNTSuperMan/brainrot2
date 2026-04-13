use crate::{
    cfg::cfg::CFG,
    ssa::{parse::version_map::compute_version_map, ssa::SSAProgram},
};

mod version_map;

impl SSAProgram {
    pub fn new(cfg: &CFG) -> SSAProgram {
        let mut blocks = vec![];
        let version_map = compute_version_map(cfg);

        for block in &cfg.0 {
            // todo
        }

        SSAProgram(blocks)
    }
}
