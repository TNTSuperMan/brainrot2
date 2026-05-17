use crate::cfg::cfg::{CFG, CFGEdge};

pub fn compute_block_order(cfg: &CFG) -> Vec<usize> {
    let mut order = vec![];
    let mut visited = vec![false; cfg.0.len()];
    let mut queue = vec![0];

    while let Some(b) = queue.pop() {
        if visited[b] {
            continue;
        }

        order.push(b);
        visited[b] = true;

        match cfg.0[b].edge {
            CFGEdge::Jump(to) => {
                queue.push(to);
            }
            CFGEdge::Branch { pointer: _, zero, nonzero } |
            CFGEdge::BranchWithIRAt { pointer: _, zero, nonzero, ir_at: _ } => {
                queue.push(zero);
                queue.push(nonzero);
            }
            CFGEdge::FindZeroAndJump { jumpto, .. } => {
                queue.push(jumpto);
            }
            CFGEdge::End => {}
        }
    }

    order
}
