use crate::ssa::ssa::{SSAEdge, SSAProgram};

pub fn ssa_to_dot(program: &SSAProgram) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    for (i, node) in program.0.iter().enumerate() {
        dot.push_str(&format!("    n{i} [\n        label=\"n{i}\\l"));
        for phi in &node.phis {
            dot.push_str(&format!("{phi:?}\\l"));
        }
        for inst in &node.insts {
            dot.push_str(&format!("{inst:?}\\l"));
        }
        if let Some(offset) = node.offset {
            dot.push_str(&format!("offset {offset}\\l"));
        }
        dot.push_str(&format!("{:?}\\l\"\n", node.edge));
        if node.offset.is_some() {
            dot.push_str(&format!("        shape=octagon\n"));
        } else if node.insts.len() != 0 || node.offset.is_some() {
            dot.push_str(&format!("        shape=box\n"));
        }
        dot.push_str(&format!("    ]\n"));
    }

    for (i, node) in program.0.iter().enumerate() {
        match node.edge {
            SSAEdge::Branch {
                version: _,
                zero,
                nonzero,
            } => {
                dot.push_str(&format!("    n{i} -> n{zero}\n    n{i} -> n{nonzero}\n"));
            }
            SSAEdge::Jump(addr) => {
                dot.push_str(&format!("    n{i} -> n{addr}\n"));
            }
            SSAEdge::End => {}
        }
    }

    dot.push_str("}\n");
    dot
}
