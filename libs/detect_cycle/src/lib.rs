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
/// let cycle = detect_cycle_undirected(6, &[(0, 1), (1, 2), (2, 3), (2, 5), (3, 4), (4, 1)]).unwrap();
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
pub fn detect_cycle_undirected(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    fn dfs(
        curr: usize,
        prev_edge: Option<usize>,
        g: &[Vec<(usize, usize)>],
        depth: &mut [Option<usize>],
        path: &mut Vec<usize>,
    ) -> Option<Vec<usize>> {
        depth[curr] = Some(path.len());
        for &(nxt, idx) in &g[curr] {
            if Some(idx) == prev_edge {
                continue;
            }
            if let Some(nxt_depth) = depth[nxt] {
                let mut cycle = path[nxt_depth..].to_vec();
                cycle.push(idx);
                return Some(cycle);
            }
            path.push(idx);
            if let Some(cycle) = dfs(nxt, Some(idx), g, depth, path) {
                return Some(cycle);
            }
            path.pop();
        }
        None
    }

    let mut g = vec![vec![]; n];
    for (idx, &(u, v)) in edges.iter().enumerate() {
        g[u].push((v, idx));
        g[v].push((u, idx));
    }
    let mut depth = vec![None; n];
    let mut path = Vec::new();

    for v in 0..n {
        if depth[v].is_some() {
            continue;
        }
        if let Some(cycle) = dfs(v, None, &g, &mut depth, &mut path) {
            return Some(cycle);
        }
    }
    None
}

/// 有向グラフの閉路を求めます。
///
/// - `n`: 頂点数
/// - `edges`: 辺
///
/// 返り値は、閉路をなす辺の index のベクタです。
///
/// # Example
/// ```
/// use detect_cycle::detect_cycle_directed;
///
/// //      0       1       3
/// // (0) --> (1) --> (2) --> (5)
/// //          ^       |
/// //        5 |       | 2
/// //          |       v
/// //         (4) <-- (3)
/// //              4
///
/// let cycle = detect_cycle_directed(6, &[(0, 1), (1, 2), (2, 3), (2, 5), (3, 4), (4, 1)]);
/// assert_eq!(cycle, Some(vec![1, 2, 4, 5]));
/// ```
pub fn detect_cycle_directed(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
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
    for (idx, &(u, v)) in edges.iter().enumerate() {
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
    use crate::{detect_cycle_directed, detect_cycle_undirected};

    #[test]
    fn test_undirected_triangle() {
        let cycle = detect_cycle_undirected(3, &[(0, 2), (2, 1), (1, 0)]).unwrap();
        assert!(
            [
                vec![0, 1, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![2, 1, 0],
                vec![1, 0, 2],
            ]
            .contains(&cycle)
        );
    }

    #[test]
    fn test_undirected_v() {
        let cycle = detect_cycle_undirected(3, &[(0, 2), (0, 1)]);
        assert_eq!(cycle, None);
    }

    #[test]
    fn test_undirected_parallel_edges() {
        let cycle = detect_cycle_undirected(2, &[(0, 1), (0, 1)]).unwrap();
        assert!([vec![0, 1], vec![1, 0]].contains(&cycle));
    }

    #[test]
    fn test_undirected_self_loop() {
        let cycle = detect_cycle_undirected(1, &[(0, 0)]);
        assert_eq!(cycle, Some(vec![0]));
    }

    #[test]
    fn test_directed_triangle() {
        let cycle = detect_cycle_directed(3, &[(0, 2), (2, 1), (1, 0)]);
        assert_eq!(cycle, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_directed_v() {
        let cycle = detect_cycle_directed(3, &[(0, 2), (0, 1)]);
        assert_eq!(cycle, None);
    }
}
