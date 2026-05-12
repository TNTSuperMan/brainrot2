use crate::ssa::defines::{block::SSAEdge, program::SSAProgram};

pub fn ssa_to_dot(ssa: &SSAProgram) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    for (i, block) in ssa.0.iter().enumerate() {
        if !block.alive {
            continue;
        }
        dot.push_str(&format!("    n{i} [\n        label=\"n{i}\\l"));
        for (ptr, (ver, args)) in &block.phis {
            dot.push_str(&format!(
                "${ptr}#{ver} = φ({})",
                args.iter()
                    .map(|a| format!("{a}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
        for inst in &block.insts {
            dot.push_str(&format!("{inst}\\l"));
        }
        if let Some(offset) = block.offset {
            dot.push_str(&format!("offset {offset}\\l"));
        }
        dot.push_str(&format!("{}\\l\"\n", block.edge));

        if block.offset.is_some() {
            dot.push_str(&format!("        shape=octagon\n"));
        } else if block.insts.len() != 0 || block.offset.is_some() {
            dot.push_str(&format!("        shape=box\n"));
        }
        dot.push_str(&format!("    ]\n"));
    }

    for (i, node) in ssa.0.iter().enumerate() {
        if !node.alive {
            continue;
        }
        match node.edge {
            SSAEdge::Branch { zero, nonzero, .. } | SSAEdge::BranchLoad { zero, nonzero, .. } => {
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
