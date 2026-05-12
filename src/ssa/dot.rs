use crate::ssa::defines::{block::SSAEdge, op::SSAOp, program::SSAProgram, value::SSAValue};

pub fn ssa_to_dot(ssa: &SSAProgram) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    let mut id = 0;

    macro_rules! d {
        ($($arg:tt)*) => {
            dot.push_str(&format!($($arg)*))
        };
    }

    for block in &ssa.0 {
        if !block.alive {
            continue;
        }
        for (ptr, (ver, args)) in &block.phis {
            let args_str = args
                .iter()
                .map(|a| format!("{a}"))
                .collect::<Vec<String>>()
                .join(", ");
            d!("v{ver} [ shape=box label=\"${ptr}#{ver} = φ({args_str})\" ]\n");
            for arg in args {
                if let SSAValue::Version(v) = arg {
                    d!("v{} -> v{}\n", v.version, ver);
                }
            }
        }
        for inst in &block.insts {
            match inst {
                SSAOp::Out(val) => {
                    if let SSAValue::Version(ver) = val {
                        d!("o{id} [ shape=box label=\"stdout < {ver}\" ]\n");
                        d!("v{} -> o{id}\n", ver.version);
                        id += 1;
                    }
                }
                SSAOp::In(ver) => {
                    d!("v{} [ shape=box label=\"{ver} < stdin\" ]\n", ver.version);
                }
                SSAOp::Assign(ver, expr) => {
                    d!("v{} [ shape=box label=\"{ver} = {expr}\" ]\n", ver.version);
                    for read in inst.reads() {
                        d!("v{} -> v{}\n", read.version, ver.version);
                    }
                }
                SSAOp::Hint(ver, val) => {
                    d!(
                        "v{} [ shape=box label=\"{ver} = {val} (hint)\" ]\n",
                        ver.version
                    );
                    if let SSAValue::Version(v) = val {
                        d!("v{} -> v{}\n", v.version, ver.version);
                    }
                }
            }
        }
        if let SSAEdge::Branch {
            version,
            zero,
            nonzero,
            ..
        } = &block.edge
        {
            d!("o{id} [ shape=box label=\"branch {} ? {nonzero} : {zero}\" ]\n");
            d!("v{} -> o{id}\n", version.version);
            id += 1;
        }
    }

    dot.push('}');

    dot
}
