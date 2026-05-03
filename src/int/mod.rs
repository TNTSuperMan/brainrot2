use crate::{bytecode::bytecode::Bytecode, int::{program::UnsafeProgram, run_deopt::run_deopt, tape::{OutOfRangeError, Tape}}};

mod tape;
mod program;
mod run_deopt;

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
            true => todo!(),
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
