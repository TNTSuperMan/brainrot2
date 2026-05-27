use crate::{bytecode::bytecode::Bytecode, exec::{bytecode::{program::UnsafeProgram, run_deopt::run_deopt, run_opt::run_opt}, tape::{OutOfRangeError, Tape, UnsafeTape}}};

mod program;
mod run_deopt;
mod run_opt;

enum InterpretResult {
    End,
    ToggleOpt(bool),
}

pub fn run<const FLUSH: bool>(bytecodes: &[Bytecode], pc: usize, tape: &mut Tape, opt_first: bool) -> Result<(), OutOfRangeError> {
    let mut program = UnsafeProgram::new(bytecodes, pc);

    let mut opt = opt_first;

    loop {
        let result = match opt {
            true => {
                let mut unsafe_tape = UnsafeTape::new(tape);
                unsafe { run_opt::<FLUSH>(&mut program, &mut unsafe_tape) }
            },
            false => run_deopt::<FLUSH, true>(&mut program, tape)?,
        };
        match result {
            InterpretResult::End => {
                return Ok(());
            }
            InterpretResult::ToggleOpt(o) => {
                opt = o;
            }
        }
    }
}
