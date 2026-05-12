use crate::ssa::defines::block::SSABlock;

#[derive(Clone, Debug)]
pub struct SSAProgram(pub Vec<SSABlock>);
