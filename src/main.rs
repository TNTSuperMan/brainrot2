use std::fs;

use crate::{
    cfg::{cfg::CFG, dot::cfg_to_dot, int::exec_from_cfg},
    ir::{int::exec_from_ir, ir::IR},
};

mod cfg;
mod error;
mod ir;

fn main() {
    let code = fs::read_to_string("box/mandel.bf").unwrap();
    let ir = IR::parse(&code).unwrap();
    //exec_from_ir(&ir);
    let cfg = CFG::new(&ir);
    println!("{}", cfg_to_dot(&cfg));
    //println!("{ir:?}");
    //println!("{cfg:?}");
    //exec_from_cfg(&cfg);
}
