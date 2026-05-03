use std::{env::args, fs, process::ExitCode, time::Instant};

use crate::{
    bytecode::{build::build_bytecode, int::exec_bytecode}, cfg::{cfg::CFG, dot::cfg_to_dot, int::exec_from_cfg}, ir::{int::exec_from_ir, ir::IR}
};

mod cfg;
mod ir;
mod bytecode;

pub const TAPE_LENGTH: usize = 30000;

fn main() -> ExitCode {
    if let [_, kind, file] = args().collect::<Vec<String>>().as_slice() {
        let code = fs::read_to_string(&file).unwrap();
        let start = Instant::now();
        let (ir, mul_offset) = IR::parse(&code).unwrap();
        let ir_end = Instant::now();
        let mut cfg = CFG::new(&ir);
        let cfg_end = Instant::now();
        for _ in 0..3 {
            cfg.inline_branch();
            cfg.inline_flow();
            cfg.fold_jump();
            cfg.fold_ref();
            cfg.fold_const();
            cfg.eliminate_dead_code();
            cfg.eliminate_dead_instruction();
        }
        let opt_end = Instant::now();
        let offset_ranges = cfg.compute_offset_ranges();
        let offset_end = Instant::now();
        let bytecodes = build_bytecode(&cfg, &offset_ranges).unwrap();
        let bytecode_end = Instant::now();
        eprintln!("all: {:?}", opt_end - start);
        eprintln!("ir: {:?}", ir_end - start);
        eprintln!("cfg: {:?}", cfg_end - ir_end);
        eprintln!("opt: {:?}", opt_end - cfg_end);
        eprintln!("ofs: {:?}", offset_end - opt_end);
        eprintln!("byt: {:?}", bytecode_end - offset_end);
        match kind.as_str() {
            "exec_ir" => {
                exec_from_ir(&ir, mul_offset);
            }
            "exec_cfg" => {
                exec_from_cfg(&cfg, mul_offset);
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
                for (i, c) in bytecodes.iter().enumerate() {
                    println!("%{i}  \t{c:?}");
                }
            }
            "exec_bytecode" => {
                exec_bytecode::<false>(&bytecodes, mul_offset, match offset_ranges.get(&0) {
                    Some(r) => r.contains(&(mul_offset as isize)),
                    None => true,
                });
            }
            "dump_offsetrange" => {
                println!("{:?}", cfg.compute_offset_ranges());
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
