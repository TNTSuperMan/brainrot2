use std::{env::args, fs, process::ExitCode};

use crate::{
    cfg::{cfg::CFG, dot::cfg_to_dot, int::exec_from_cfg},
    ir::{int::exec_from_ir, ir::IR},
};

mod cfg;
mod error;
mod ir;
mod ssa;

fn main() -> ExitCode {
    let args = args();
    if let [_, _, kind, file] = args.collect::<Vec<String>>().as_slice() {
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
            _ => {
                eprintln!("unknown kind");
                return ExitCode::FAILURE;
            }
        }
    }
    return ExitCode::FAILURE;
}
