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

pub fn detect_cycle_directed(
    n: usize,
    edges: impl Iterator<Item = (usize, usize)>,
) -> Option<Vec<usize>> {
    fn dfs(
        curr: usize,
        g: &[Vec<(usize, usize)>],
        seen: &mut Vec<bool>,
        on_path: &mut Vec<bool>,
    ) -> Option<(usize, Vec<usize>, bool)> {
        seen[curr] = true;
        on_path[curr] = true;
        for &(nxt, idx) in &g[curr] {
            if on_path[nxt] {
                assert!(seen[nxt]);
                return Some((nxt, vec![idx], true));
            }
            if seen[nxt] {
                continue;
            }
            if let Some((start_node, mut cycle, in_cycle)) = dfs(nxt, g, seen, on_path) {
                return if in_cycle {
                    cycle.push(idx);
                    if curr == start_node {
                        cycle.reverse();
                        Some((start_node, cycle, false))
                    } else {
                        Some((start_node, cycle, true))
                    }
                } else {
                    Some((start_node, cycle, false))
                };
            }
        }
        on_path[curr] = false;
        None
    }

    let mut g = vec![vec![]; n];
    for (idx, (u, v)) in edges.enumerate() {
        g[u].push((v, idx));
    }
    let mut seen = vec![false; n];
    let mut on_path = vec![false; n];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        if let Some((_, cycle, in_cycle)) = dfs(v, &g, &mut seen, &mut on_path) {
            assert!(!in_cycle);
            return Some(cycle);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::detect_cycle_directed;

    #[test]
    fn test_directed_triangle() {
        let cycle = detect_cycle_directed(3, [(0, 2), (2, 1), (1, 0)].iter().copied());
        assert_eq!(cycle, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_directed_v() {
        let cycle = detect_cycle_directed(3, [(0, 2), (0, 1)].iter().copied());
        assert_eq!(cycle, None);
    }
}
