//! グラフの閉路検出アルゴリズムのライブラリです。
//!
//! 無向グラフと有向グラフの両方でサイクル（閉路）を効率的に検出します。
//! 検出されたサイクルは辺のインデックスのリストとして返されます。
//!
//! # 計算量
//!
//! - 時間計算量: O(V + E)
//! - 空間計算量: O(V + E)
//!
//! ここで V は頂点数、E は辺数です。
//!
//! # アルゴリズム
//!
//! - **無向グラフ**: DFS による後退辺の検出
//! - **有向グラフ**: DFS による閉路検出（グレイ頂点への辺）
//!
//! # 用途
//!
//! - グラフの閉路検出
//! - DAG（有向非循環グラフ）の判定
//! - 競技プログラミングでのグラフ問題
//! - トポロジカルソートの前処理
//!
//! # Examples
//!
//! ## 無向グラフでの閉路検出
//!
//! ```
//! use detect_cycle::detect_cycle_undirected;
//!
//! // 三角形グラフ: 0-1-2-0
//! let cycle = detect_cycle_undirected(3, &[(0, 1), (1, 2), (2, 0)]);
//! assert!(cycle.is_some());
//! assert_eq!(cycle.unwrap().len(), 3); // 3本の辺からなる閉路
//!
//! // 木構造（閉路なし）
//! let no_cycle = detect_cycle_undirected(3, &[(0, 1), (1, 2)]);
//! assert!(no_cycle.is_none());
//! ```
//!
//! ## 有向グラフでの閉路検出
//!
//! ```
//! use detect_cycle::detect_cycle_directed;
//!
//! // 有向三角形: 0→1→2→0
//! let cycle = detect_cycle_directed(3, &[(0, 1), (1, 2), (2, 0)]);
//! assert!(cycle.is_some());
//! assert_eq!(cycle.unwrap().len(), 3);
//!
//! // DAG（閉路なし）
//! let no_cycle = detect_cycle_directed(3, &[(0, 1), (0, 2), (1, 2)]);
//! assert!(no_cycle.is_none());
//! ```

/// 無向グラフの閉路を求めます。
/// 無向グラフの閉路を検出します。
///
/// DFS を使用して無向グラフ内の閉路を検出し、閉路を構成する辺のインデックスを返します。
/// 閉路が複数存在する場合、そのうちの1つを返します。
///
/// # 引数
///
/// - `n`: 頂点数（頂点は 0, 1, ..., n-1 で番号付けされます）
/// - `edges`: 無向辺のリスト。各要素 `(u, v)` は頂点 u と頂点 v を結ぶ辺を表します
///
/// # 戻り値
///
/// - `Some(Vec<usize>)`: 閉路が存在する場合、閉路を構成する辺のインデックスのリスト
/// - `None`: 閉路が存在しない場合（つまり、グラフが森である場合）
///
/// 返される辺のインデックスは、`edges` スライス内での位置を示します。
///
/// # Examples
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
///
/// ## 競技プログラミングでの応用例
///
/// ```
/// use detect_cycle::detect_cycle_undirected;
///
/// // グラフが木かどうかの判定
/// fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
///     // 木の条件: 連結 かつ 辺数 = 頂点数 - 1 かつ 閉路なし
///     edges.len() == n - 1 && detect_cycle_undirected(n, edges).is_none()
/// }
///
/// // テストケース
/// assert!(is_tree(4, &[(0, 1), (1, 2), (1, 3)])); // 木
/// assert!(!is_tree(4, &[(0, 1), (1, 2), (2, 3), (3, 0)])); // 閉路あり
/// assert!(!is_tree(4, &[(0, 1), (2, 3)])); // 非連結
/// ```
pub fn detect_cycle_undirected(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
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

/// 有向グラフの閉路を求めます。
/// 有向グラフの閉路を検出します。
///
/// DFS を使用して有向グラフ内の閉路を検出し、閉路を構成する辺のインデックスを返します。
/// 閉路が複数存在する場合、そのうちの1つを返します。
///
/// # 引数
///
/// - `n`: 頂点数（頂点は 0, 1, ..., n-1 で番号付けされます）
/// - `edges`: 有向辺のリスト。各要素 `(u, v)` は頂点 u から頂点 v への辺を表します
///
/// # 戻り値
///
/// - `Some(Vec<usize>)`: 閉路が存在する場合、閉路を構成する辺のインデックスのリスト
/// - `None`: 閉路が存在しない場合（つまり、グラフが DAG である場合）
///
/// 返される辺のインデックスは、`edges` スライス内での位置を示します。
/// 辺のリストは閉路の順序で並んでいます。
///
/// # Examples
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
///
/// ## 競技プログラミングでの応用例
///
/// ```
/// use detect_cycle::detect_cycle_directed;
///
/// // DAG判定とトポロジカルソートの前処理
/// fn is_dag(n: usize, edges: &[(usize, usize)]) -> bool {
///     detect_cycle_directed(n, edges).is_none()
/// }
///
/// // 依存関係グラフでの循環依存検出
/// fn has_circular_dependency(
///     tasks: usize,
///     dependencies: &[(usize, usize)]
/// ) -> Option<Vec<usize>> {
///     // (a, b) = タスク a がタスク b に依存
///     detect_cycle_directed(tasks, dependencies)
/// }
///
/// // テストケース
/// assert!(is_dag(3, &[(0, 1), (1, 2)])); // DAG
/// assert!(!is_dag(3, &[(0, 1), (1, 2), (2, 0)])); // 閉路あり
///
/// // 循環依存のあるタスク
/// let deps = vec![(0, 1), (1, 2), (2, 0)]; // 0→1→2→0
/// assert!(has_circular_dependency(3, &deps).is_some());
/// ```
///
/// # アルゴリズムの詳細
///
/// このアルゴリズムは DFS を使用して「グレイ」状態の頂点（現在の DFS パス上にある頂点）
/// への辺を検出することで閉路を見つけます。これは有向グラフでの標準的な閉路検出手法です。
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
    use crate::detect_cycle_directed;

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
