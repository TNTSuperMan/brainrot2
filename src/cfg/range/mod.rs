use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use crate::{
    cfg::{cfg::{CFG, CFGEdge}, range::range::extend_range},
};

mod block;
mod range;

pub use range::OffsetRange;

impl CFG {
    /// ブロックから到達可能なすべてのブロックへのアクセス範囲をキャッシュ付きで計算
    fn compute_access_range_cached(
        &self,
        block_i: usize,
        cache: &mut HashMap<usize, Option<RangeInclusive<i16>>>,
        visited: &mut HashSet<usize>,
    ) -> Option<RangeInclusive<i16>> {
        // キャッシュをチェック
        if let Some(cached) = cache.get(&block_i) {
            return cached.clone();
        }

        // 循環参照を防ぐため、訪問中マークをつける
        if visited.contains(&block_i) {
            return None;
        }
        visited.insert(block_i);

        let mut range = None;
        let block = &self.0[block_i];

        // ブロック内の命令でアクセス範囲を拡張
        block.extend_access_range(&mut range);

        // オフセット確定ポイント到達まで後続ブロックを処理
        if !block.has_offset() {
            for succ in block.edge.successor() {
                let succ_range =
                    self.compute_access_range_cached(succ, cache, visited);
                if let Some(succ_range) = succ_range {
                    range = if let Some(current_range) = range {
                        Some(min(*current_range.start(), *succ_range.start())
                            ..=max(*current_range.end(), *succ_range.end()))
                    } else {
                        Some(succ_range)
                    };
                }
            }
        }

        visited.remove(&block_i);
        cache.insert(block_i, range.clone());
        range
    }

    fn compute_access_range_from_edge_cached(
        &self,
        block_i: usize,
        cache: &mut HashMap<usize, Option<RangeInclusive<i16>>>,
    ) -> Option<RangeInclusive<i16>> {
        let mut range = None;
        let block = &self.0[block_i];

        // エッジのポインタから範囲を初期化
        if let CFGEdge::Branch {
            pointer, ..
        } | CFGEdge::BranchWithIRAt {
            pointer, ..
        } = &block.edge
        {
            range = extend_range(&range, *pointer);
        }

        // 後続ブロックから範囲を取得
        for succ in block.edge.successor() {
            let mut visited = HashSet::new();
            let succ_range = self.compute_access_range_cached(succ, cache, &mut visited);
            if let Some(succ_range) = succ_range {
                range = if let Some(current_range) = range {
                    Some(min(*current_range.start(), *succ_range.start())
                        ..=max(*current_range.end(), *succ_range.end()))
                } else {
                    Some(succ_range)
                };
            }
        }

        range
    }

    pub fn compute_offset_ranges(&self) -> HashMap<usize, OffsetRange> {
        let mut map = HashMap::new();
        let mut visited = HashSet::new();
        // グローバルキャッシュで重複計算を排除
        let mut access_range_cache: HashMap<usize, Option<RangeInclusive<i16>>> =
            HashMap::new();

        let mut dfs_stack = vec![0];
        while let Some(b) = dfs_stack.pop() {
            if visited.contains(&b) {
                continue;
            }
            visited.insert(b);
            let block = &self.0[b];

            if b == 0 {
                // ブロック0は特別な処理
                let mut visit_set = HashSet::new();
                if let Some(r) = self.compute_access_range_cached(0, &mut access_range_cache, &mut visit_set) {
                    map.insert(0, OffsetRange::from(r));
                }
            } else if block.offset.is_some()
                || matches!(block.edge, CFGEdge::FindZeroAndJump { .. })
            {
                // オフセット確定ポイント
                if let Some(r) = self.compute_access_range_from_edge_cached(b, &mut access_range_cache) {
                    map.insert(b, OffsetRange::from(r));
                }
            }

            dfs_stack.append(&mut block.edge.successor());
        }

        map
    }
}
