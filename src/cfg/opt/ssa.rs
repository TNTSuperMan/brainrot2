use std::collections::{HashMap, HashSet};

use crate::cfg::cfg::{CFG, CFGExpr, CFGOp, CFGValue};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SSAVersion(i16, usize);

#[derive(Clone)]
enum SSAVal {
    Ver(SSAVersion),
    Const(u8),
}

#[derive(Clone)]
enum SSAExpr {
    Val(SSAVal),
    Add(SSAVal, SSAVal),
    Sub(SSAVal, SSAVal),
    Mul(SSAVal, SSAVal),
    MulAdd(SSAVal, SSAVal, u8),
}
impl SSAExpr {
    fn vals(&self) -> Vec<&SSAVal> {
        match self {
            Self::Val(v) => vec![v],
            Self::Add(a, b) |
            Self::Sub(a, b) |
            Self::Mul(a, b) |
            Self::MulAdd(a, b, _) => vec![a, b],
        }
    }
}

fn mk_ssa(insts: &[CFGOp]) -> Option<HashMap<i16, Vec<SSAExpr>>> {
    let mut map = HashMap::new();

    macro_rules! ver {
        ($ptr: expr) => {
            map.get($ptr).map_or(0usize, |e: &Vec<SSAExpr>| e.len())
        };
    }
    macro_rules! val {
        ($val: expr) => {
            match $val {
                CFGValue::Const(c) => SSAVal::Const(*c),
                CFGValue::Load(v) => SSAVal::Ver(SSAVersion(*v, ver!(v))),
            }
        };
    }

    for inst in insts {
        if let CFGOp::Assign(ptr, expr) = inst {
            let expr = match expr {
                CFGExpr::Value(v) => SSAExpr::Val(val!(v)),
                CFGExpr::Add(a, b) => SSAExpr::Add(val!(a), val!(b)),
                CFGExpr::Sub(a, b) => SSAExpr::Sub(val!(a), val!(b)),
                CFGExpr::Mul(a, b) => SSAExpr::Mul(val!(a), val!(b)),
                CFGExpr::MulAdd(a, b, c) => SSAExpr::MulAdd(val!(a), val!(b), *c),
                CFGExpr::In => return None,
            };

            if let Some(e) = map.get_mut(ptr) {
                e.push(expr);
            } else {
                map.insert(*ptr, vec![expr]);
            }
        } else {
            return None;
        }
    }

    Some(map)
}

impl CFG {
    pub fn ssa_opt(&mut self) {
        for b in &mut self.0 {
            if !b.alive { continue }
            let mut ssa = match mk_ssa(&b.insts) {
                Some(ssa) => ssa,
                None => continue,
            };

            let mut dfs_stack: Vec<SSAVersion> = ssa.iter().map(|(k, v)| SSAVersion(*k, v.len())).collect();
            let mut visited = HashSet::new();

            while let Some(ver) = dfs_stack.pop() {
                if visited.contains(&ver) {
                    continue;
                }
                visited.insert(ver);

                match ssa[&ver.0][ver.1].clone() {
                    SSAExpr::Val(SSAVal::Ver(v)) => {
                        ssa.get_mut(&ver.0).unwrap()[ver.1] = ssa[&v.0][v.1].clone();
                    }
                    _ => {}
                }

                for val in ssa[&ver.0][ver.1].vals() {
                    if let SSAVal::Ver(v) = val {
                        dfs_stack.push(*v);
                    }
                }
            }
        }
    }
}
