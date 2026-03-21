use crate::cfg::cfg::{CFG, CFGEdge};

pub fn cfg_to_dot(cfg: &CFG) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    for (i, node) in cfg.0.iter().enumerate() {
        if node.insts.len() != 0 {
            dot.push_str(&format!("    n{i} [shape=box]"));
        }
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
            CFGEdge::JumpNext => {
                dot.push_str(&format!("    n{i} -> n{}\n", i + 1));
            }
            CFGEdge::End => {}
        }
    }

    dot.push_str("}\n");
    dot
}
