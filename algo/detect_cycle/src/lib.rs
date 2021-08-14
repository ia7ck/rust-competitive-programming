/// 無向グラフの閉路を求めます。
///
/// - `n`: 頂点数
/// - `edges`: 辺
///
/// 返り値は、閉路をなす辺の index のベクタです。
///
/// # Example
/// ```
/// use detect_cycle::detect_cycle_undirected;
/// //      0       1       3
/// // (0) --- (1) --- (2) --- (5)
/// //          |       |
/// //        5 |       | 2
/// //          |       |
/// //         (4) --- (3)
/// //              4
///
/// let edges = vec![(0, 1), (1, 2), (2, 3), (2, 5), (3, 4), (4, 1)];
/// let cycle = detect_cycle_undirected(6, edges.iter().copied()).unwrap();
/// let candidates = vec![
///     vec![1, 2, 4, 5],
///     vec![2, 4, 5, 1],
///     vec![4, 5, 1, 2],
///     vec![5, 1, 2, 4],
///     vec![1, 5, 4, 2],
///     vec![5, 4, 2, 1],
///     vec![4, 2, 1, 5],
///     vec![2, 1, 5, 4],
/// ];
/// assert!(candidates.contains(&cycle));
/// ```
pub fn detect_cycle_undirected(
    n: usize,
    edges: impl Iterator<Item = (usize, usize)>,
) -> Option<Vec<usize>> {
    fn dfs(
        curr: usize,
        prev: usize,
        g: &[Vec<(usize, usize)>],
        seen: &mut Vec<bool>,
        parent: &mut Vec<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        seen[curr] = true;
        for &(nxt, idx) in &g[curr] {
            if nxt == prev {
                continue;
            }
            if seen[nxt] {
                return Some((nxt, curr));
            }
            parent[nxt] = (curr, idx);
            if let Some((start, end)) = dfs(nxt, curr, g, seen, parent) {
                return Some((start, end));
            }
        }
        None
    }

    let edges: Vec<(usize, usize)> = edges.collect();
    let mut g = vec![vec![]; n];
    for (idx, &(u, v)) in edges.iter().enumerate() {
        g[u].push((v, idx));
        g[v].push((u, idx));
    }
    let mut seen = vec![false; n];
    let mut parent = vec![(!0, !0); n];

    for v in 0..n {
        if seen[v] {
            continue;
        }
        if let Some((cycle_start, cycle_end)) = dfs(v, !0, &g, &mut seen, &mut parent) {
            let mut cycle = Vec::new();
            {
                let mut curr = cycle_end;
                while curr != cycle_start {
                    let (par, idx) = parent[curr];
                    cycle.push(idx);
                    curr = par;
                }
            }
            // cycle_end <- parent[cycle_end] <- parent[parent[cycle_end]] <- ... <- cycle_start
            for (idx, &(u, v)) in edges.iter().enumerate() {
                if (u, v) == (cycle_start, cycle_end) || (u, v) == (cycle_end, cycle_start) {
                    cycle.push(idx);
                    return Some(cycle);
                }
            }
            unreachable!();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
