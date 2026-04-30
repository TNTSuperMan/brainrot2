use std::{env::args, fs, process::ExitCode};

use crate::{
    bytecode::build::build_bytecode, cfg::{cfg::CFG, dot::cfg_to_dot, int::exec_from_cfg}, ir::{int::exec_from_ir, ir::IR}
};

mod cfg;
mod ir;
mod bytecode;

fn main() -> ExitCode {
    if let [_, kind, file] = args().collect::<Vec<String>>().as_slice() {
        let code = fs::read_to_string(&file).unwrap();
        let ir = IR::parse(&code).unwrap();
        let mut cfg = CFG::new(&ir);
        for _ in 0..3 {
            cfg.inline_branch();
            cfg.inline_flow();
            cfg.fold_jump();
            cfg.fold_const();
            cfg.eliminate_dead_code();
        }
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
            "dump_bytecode" => {
                for (i, c) in build_bytecode(&cfg).unwrap().iter().enumerate() {
                    println!("%{i}  \t{c:?}");
                }
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
