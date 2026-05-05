use std::{collections::HashMap, num::TryFromIntError, sync::{Arc, mpsc::{self, Receiver}}, thread::{self, JoinHandle}};

use crate::{bytecode::{build::build_bytecode, bytecode::Bytecode}, cfg::cfg::CFG, ir::ir::IR, timeline};

pub struct BytecodeComputePoller {
    join_handle: JoinHandle<()>,
    rx: Receiver<Result<(Vec<Bytecode>, HashMap<usize, usize>), TryFromIntError>>,
    ended: bool,
    result: Option<(Vec<Bytecode>, HashMap<usize, usize>)>,
}
impl BytecodeComputePoller {
    pub fn init(ir_arc: Arc<Vec<IR>>) -> Self {
        
        let (tx, rx) = mpsc::channel();

        let join_handle = thread::spawn(move || {
            timeline!("building cfg");

            let mut cfg = CFG::new(&ir_arc);
            timeline!("cfg builded, optimizing");

            cfg.optimize_heavy();
            timeline!("optimized, computing range");

            let offset_ranges = cfg.compute_offset_ranges();
            timeline!("range computed, building bytecode");
            
            let bytecode_result = build_bytecode(&cfg, &offset_ranges);
            timeline!("bytecode builded");

            tx.send(bytecode_result).unwrap();
        });

        Self {
            join_handle, rx, ended: false, result: None,
        }
    }
    pub fn poll(&mut self, pc: usize) -> Option<(Vec<Bytecode>, usize)> {
        if self.ended {
            None
        } else if self.result.is_some() {
            if let Some(to) = self.result.as_ref().unwrap().1.get(&pc).copied() {
                let (bytecode, _) = self.result.take().unwrap();
                Some((bytecode, to))
            } else {
                None
            }
        } else {
            match self.rx.try_recv() {
                Err(mpsc::TryRecvError::Empty) => None,
                Ok(Ok(result)) => {
                    if let Some(to) = result.1.get(&pc).copied() {
                        Some((result.0, to))
                    } else {
                        self.result = Some(result);
                        None
                    }
                }
                Ok(Err(_)) | Err(_) => {
                    self.ended = true;
                    None
                }
            }
        }
    }
}
