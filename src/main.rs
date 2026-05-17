use std::{collections::HashMap, env::args, fs, io::{Write, stdout}, process::ExitCode};

use crate::{
    bytecode::{build::build_bytecode, bytecode::Bytecode, int::debug_exec_bytecode}, bytecode2::build::build_bytecode2, cfg::{cfg::CFG, dot::cfg_to_dot, range::OffsetRange}, exec::exec, ir::ir::IR, log::start
};

mod bytecode;
mod bytecode2;
mod cfg;
mod exec;
mod ir;
mod log;
// mod int;

pub const TAPE_LENGTH: usize = 65536;

fn gen_bytecode(
    code: &str,
) -> (
    (Vec<Bytecode>, HashMap<usize, usize>),
    HashMap<usize, OffsetRange>,
    u8,
) {
    let (ir, mul_offset) = IR::parse(&code).unwrap();
    let mut cfg = CFG::new(&ir);
    cfg.optimize_heavy();
    let offset_ranges = cfg.compute_offset_ranges();
    (
        build_bytecode(&cfg, &offset_ranges).unwrap(),
        offset_ranges,
        mul_offset,
    )
}

fn main() -> ExitCode {
    start();
    if let [_, kind, file] = args().collect::<Vec<String>>().as_slice() {
        let code = fs::read_to_string(&file).unwrap();
        match kind.as_str() {
            "dump_ir" => {
                let (ir, _) = IR::parse(&code).unwrap();
                println!("{ir:?}");
            }
            "dump_cfg" => {
                let (ir, _) = IR::parse(&code).unwrap();
                let cfg = CFG::new(&ir);
                println!("{cfg:?}");
            }
            "dump_opt_cfg" => {
                let (ir, _) = IR::parse(&code).unwrap();
                let mut cfg = CFG::new(&ir);
                cfg.optimize_heavy();
                println!("{cfg:?}");
            }
            "dot_cfg" => {
                let (ir, _) = IR::parse(&code).unwrap();
                let cfg = CFG::new(&ir);
                println!("{}", cfg_to_dot(&cfg));
            }
            "dot_opt_cfg" => {
                let (ir, _) = IR::parse(&code).unwrap();
                let mut cfg = CFG::new(&ir);
                cfg.optimize_heavy();
                println!("{}", cfg_to_dot(&cfg));
            }
            "dump_bytecode" => {
                for (i, c) in gen_bytecode(&code).0.0.iter().enumerate() {
                    println!("%{i}  \t{c}");
                }
            }
            "dump_classic_bytecode" => {
                for c in gen_bytecode(&code).0.0 {
                    println!("{c:?}");
                }
            }
            "exec_bytecode" => {
                let ((bytecodes, _), _, mul_offset) = gen_bytecode(&code);
                debug_exec_bytecode::<false>(&bytecodes, mul_offset as i16, [0; TAPE_LENGTH], 0);
            }
            "check_exec_counts" => {
                let ((bytecodes, _), _, mul_offset) = gen_bytecode(&code);
                let counts =
                    debug_exec_bytecode::<true>(&bytecodes, mul_offset as i16, [0; TAPE_LENGTH], 0);
                for (i, count) in counts.iter().enumerate() {
                    println!("{} \t{:?}", (count + 1).ilog2(), bytecodes[i]);
                }
            }
            "dump_ir_map" => {
                let ((_, ir_map), ..) = gen_bytecode(&code);
                println!("{:?}", ir_map);
            }
            "dump_offsetrange" => {
                let (_, offset_ranges, ..) = gen_bytecode(&code);
                println!("{:?}", offset_ranges);
            }
            "dump_bytecode2" => {
                let (ir, mul_offset) = IR::parse(&code).unwrap();
                let mut cfg = CFG::new(&ir);
                cfg.optimize_heavy();
                let offset_ranges = cfg.compute_offset_ranges();
                stdout().write(&build_bytecode2(&cfg, &offset_ranges).unwrap()).unwrap();
            }
            "run" => {
                exec(&code).unwrap();
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
