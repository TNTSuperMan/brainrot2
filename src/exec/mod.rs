use std::{fmt::Debug, sync::Arc};

use crate::{
    TAPE_LENGTH,
    bytecode::int::debug_exec_bytecode,
    exec::{ir::exec_ir_with_poll, thread_poll::BytecodeComputePoller},
    ir::{error::SyntaxError, ir::IR},
    timeline,
};

mod ir;
mod thread_poll;

pub enum BrainrotError {
    SyntaxError(SyntaxError),
}
impl Debug for BrainrotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError(SyntaxError::UnmatchedOpeningBracket) => write!(f, "SyntaxError: Unmatched opening bracket"),
            Self::SyntaxError(SyntaxError::UnmatchedClosingBracket) => write!(f, "SyntaxError: Unmatched closing bracket"),
        }
    }
}

pub fn exec(code: &str) -> Result<(), BrainrotError> {
    timeline!("parsing ir");
    let (ir, mul_offset) = match IR::parse(code) {
        Ok(ir) => ir,
        Err(err) => return Err(BrainrotError::SyntaxError(err)),
    };
    timeline!("ir parsed");

    let ir_arc = Arc::new(ir);

    let mut poller = BytecodeComputePoller::init(ir_arc.clone());

    let mut memory = [0; TAPE_LENGTH];
    let mut offset = mul_offset.into();

    timeline!("ir executing");

    match exec_ir_with_poll(&ir_arc, &mut memory, &mut offset, &mut poller) {
        Some((bytecodes, pc)) => {
            timeline!("osr bytecode executing: {pc}");
            debug_exec_bytecode::<false>(&bytecodes, offset, memory, pc);
            timeline!("program ended");
            Ok(())
        }
        None => {
            timeline!("program ended");
            Ok(())
        }
    }
}
