use crate::cfg::cfg::{CFG, CFGOpKind};

pub struct UniqueVersionMap(Vec<usize>);
impl UniqueVersionMap {
    pub fn new() -> Self {
        UniqueVersionMap(vec![])
    }

    fn pointer_to_map_address(&self, pointer: isize) -> usize {
        (if pointer < 0 {
            (-pointer) * 2 + 1
        } else {
            pointer * 2
        }) as usize
    }

    pub fn get_unique_version(&mut self, pointer: isize) -> usize {
        let addr = self.pointer_to_map_address(pointer);
        if self.0.len() <= addr {
            self.0.resize(addr + 1, 0);
        }
        let ver = self.0[addr];
        self.0[addr] = ver + 1;
        ver
    }
}

type VersionMap = Vec<Vec<usize>>; // vec[block_id][inst_index]

fn compute_version_map(cfg: &CFG) -> (VersionMap, UniqueVersionMap) {
    let mut latest_version_map = UniqueVersionMap(vec![]);

    let map: VersionMap = cfg
        .0
        .iter()
        .map(|block| {
            block
                .insts
                .iter()
                .map(|inst| match inst.opcode {
                    CFGOpKind::Breakpoint | CFGOpKind::Out => 0,
                    CFGOpKind::Add(_)
                    | CFGOpKind::Set(_)
                    | CFGOpKind::MulAdd(..)
                    | CFGOpKind::In => latest_version_map.get_unique_version(inst.pointer),
                })
                .collect()
        })
        .collect();
    (map, latest_version_map)
}
