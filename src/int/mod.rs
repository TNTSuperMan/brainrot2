use crate::{bytecode::bytecode::Bytecode, int::{program::UnsafeProgram, run_deopt::run_deopt, run_opt::run_opt, tape::{OutOfRangeError, Tape, UnsafeTape}}};

mod tape;
mod program;
mod run_deopt;
mod run_opt;

enum InterpretResult {
    End,
    ToggleOpt(bool),
}

pub fn run(bytecodes: &[Bytecode], mul_offset: u8, opt_first: bool) -> Result<(), OutOfRangeError> {
    let mut program = UnsafeProgram::new(bytecodes);
    let mut tape = Tape::new(mul_offset);

    let mut opt = opt_first;

    loop {
        let result = match opt {
            true => {
                let mut unsafe_tape = UnsafeTape::new(&mut tape);
                unsafe { run_opt::<false>(&mut program, &mut unsafe_tape) }
            },
            false => run_deopt::<false, false>(&mut program, &mut tape)?,
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
