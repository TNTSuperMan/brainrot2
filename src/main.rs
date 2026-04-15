use std::{env::args, fs, process::ExitCode};

use crate::{
    cfg::{cfg::CFG, dot::cfg_to_dot, int::exec_from_cfg},
    ir::{int::exec_from_ir, ir::IR},
    ssa::{parse::SSAParser, ssa::SSAProgram},
};

mod cfg;
mod error;
mod ir;
mod ssa;

fn main() -> ExitCode {
    if let [_, kind, file] = args().collect::<Vec<String>>().as_slice() {
        let code = fs::read_to_string(&file).unwrap();
        let ir = IR::parse(&code).unwrap();
        let cfg = CFG::new(&ir);
        match kind.as_str() {
            "exec_ir" => {
                exec_from_ir(&ir);
            }
            "exec_cfg" => {
                exec_from_cfg(&cfg);
            }
            "dump_ir" => {
                println!("{ir:?}");
            }
            "dump_cfg" => {
                println!("{cfg:?}");
            }
            "print_cfg_dot" => {
                println!("{}", cfg_to_dot(&cfg));
            }
            "dump_ssa" => {
                let mut ssa_builder = SSAParser::new(&cfg);
                ssa_builder.parse_all();
                println!("{:?}", ssa_builder.program);
            }
            _ => {
                eprintln!("unknown kind");
                return ExitCode::FAILURE;
            }
        }
        ExitCode::SUCCESS
    } else {
        eprintln!("usage: brainrot2 [kind] [file]");
        ExitCode::FAILURE
    }
}
