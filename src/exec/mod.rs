use std::{fmt::Debug, num::TryFromIntError, sync::Arc};

use crate::{
    exec::{bytecode::run, ir::exec_ir_with_poll, tape::{OutOfRangeError, Tape}, thread_poll::BytecodeComputePoller}, ir::{error::SyntaxError, ir::IR}, timeline
};

mod tape;
mod bytecode;
mod ir;
mod thread_poll;

pub enum BrainrotError {
    SyntaxError(SyntaxError),
    OutOfRangeError(OutOfRangeError),
    TryFromIntError(TryFromIntError),
}
impl Debug for BrainrotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError(SyntaxError::UnmatchedOpeningBracket) => write!(f, "SyntaxError: Unmatched opening bracket"),
            Self::SyntaxError(SyntaxError::UnmatchedClosingBracket) => write!(f, "SyntaxError: Unmatched closing bracket"),
            Self::OutOfRangeError(err) => write!(f, "{err:?}"),
            Self::TryFromIntError(err) => write!(f, "{err:?}"),
        }
    }
}

pub fn exec<const FLUSH: bool>(code: &str) -> Result<(), BrainrotError> {
    timeline!("parsing ir");
    let (ir, mul_offset) = IR::parse(code)?;
    timeline!("ir parsed");

    let ir_arc = Arc::new(ir);

    let mut poller = BytecodeComputePoller::init(ir_arc.clone());

    let mut tape = Tape::new(mul_offset);

    timeline!("ir executing");

    match exec_ir_with_poll::<FLUSH>(&ir_arc, &mut tape, &mut poller) {
        Ok(Some((bytecodes, pc))) => {
            timeline!("osr bytecode executing: {pc}");
            match run::<FLUSH>(&bytecodes, pc, &mut tape, false) {
                Ok(()) => {
                    timeline!("program ended");
                    Ok(())
                }
                Err(err) => {
                    Err(BrainrotError::OutOfRangeError(err))
                }
            }
        }
        Ok(None) => {
            timeline!("program ended");
            Ok(())
        }
        Err(err) => {
            Err(BrainrotError::OutOfRangeError(err))
        }
    }
}
