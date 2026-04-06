use crate::cfg::cfg::{CFG, CFGEdge};

pub fn cfg_to_dot(cfg: &CFG) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    for (i, node) in cfg.0.iter().enumerate() {
        dot.push_str(&format!("    n{i} [\n"));
        dot.push_str(&format!("        label=\"n{i}\\l"));
        for inst in &node.insts {
            dot.push_str(&format!("{inst:?}\\l"));
        }
        if let Some(offset) = node.offset {
            dot.push_str(&format!("offset {offset}\\l"));
        }
        match &node.edge {
            CFGEdge::Jump(addr) => {
                dot.push_str(&format!("jump n{addr}\\l"));
            }
            CFGEdge::End => {}
            CFGEdge::Branch {
                pointer,
                zero,
                nonzero,
            } => {
                dot.push_str(&format!("branch ${pointer}, n{nonzero}, n{zero}\\l"));
            }
        }
        dot.push_str("\"\n");
        if node.offset.is_some() {
            dot.push_str(&format!("        shape=octagon\n"));
        } else if node.insts.len() != 0 || node.offset.is_some() {
            dot.push_str(&format!("        shape=box\n"));
        }
        dot.push_str(&format!("    ]\n"));
    }

    for (i, node) in cfg.0.iter().enumerate() {
        match node.edge {
            CFGEdge::Branch {
                pointer: _,
                zero,
                nonzero,
            } => {
                dot.push_str(&format!("    n{i} -> n{zero}\n    n{i} -> n{nonzero}\n"));
            }
            CFGEdge::Jump(addr) => {
                dot.push_str(&format!("    n{i} -> n{addr}\n"));
            }
            CFGEdge::End => {}
        }
    }

    dot.push_str("}\n");
    dot
}
