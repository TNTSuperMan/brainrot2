use crate::cfg::cfg::CFG;

pub mod cfg;
pub mod dot;
pub mod opt;
pub mod parse;
pub mod range;

impl CFG {
    pub fn optimize_heavy(&mut self) {
        for _ in 0..3 {
            self.inline_branch();
            self.inline_deepbranch();
            self.inline_flow();
            self.fold_jump();
            self.fold_ref();
            self.fold_const();
            self.eliminate_dead_code();
            self.eliminate_dead_instruction();
        }
    }
}
