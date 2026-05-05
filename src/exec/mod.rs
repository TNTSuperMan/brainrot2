use std::sync::Arc;

use crate::{TAPE_LENGTH, bytecode::int::debug_exec_bytecode, exec::{ir::exec_ir_with_poll, thread_poll::BytecodeComputePoller}, ir::{error::SyntaxError, ir::IR}};

mod thread_poll;
mod ir;

#[derive(Debug)]
pub enum BrainrotError {
    SyntaxError(SyntaxError),
}

pub fn exec(code: &str) -> Result<(), BrainrotError> {
    let (ir, mul_offset) = match IR::parse(code) {
        Ok(ir) => ir,
        Err(err) => return Err(BrainrotError::SyntaxError(err)),
    };

    let ir_arc = Arc::new(ir);

    let mut poller = BytecodeComputePoller::init(ir_arc.clone());

    let mut memory = [0; TAPE_LENGTH];
    let mut offset = mul_offset.into();

    match exec_ir_with_poll(&ir_arc, &mut memory, &mut offset, &mut poller) {
        Some((bytecodes, pc)) => {
            debug_exec_bytecode::<false>(&bytecodes, offset, memory, pc);
            Ok(())
        }
        None => {
            Ok(())
        }
    }
}
