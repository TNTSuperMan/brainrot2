use std::fmt::Display;

use crate::ssa::defines::block::SSABlock;

#[derive(Clone, Debug)]
pub struct SSAProgram(pub Vec<SSABlock>);

impl Display for SSAProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSAProgram len: {}", self.0.len())?;
        for (i, block) in self.0.iter().enumerate() {
            write!(f, "n{i}: {block}\n")?;
        }
        Ok(())
    }
}
