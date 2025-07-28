//! グラフの基本操作を提供するライブラリです。
//!
//! このライブラリは競技プログラミングでよく使われるグラフ操作を効率的に実行するための
//! 関数群を提供します。特に木の判定、連結性の確認、根付き木への変換などの基本的な
//! グラフアルゴリズムを含んでいます。
//!
//! # 主な機能
//!
//! - **木の判定**: グラフが木構造かどうかを O(E) で判定
//! - **連結性確認**: グラフが連結かどうかを DFS で確認
//! - **根付き木変換**: 無向グラフを指定した根を持つ根付き木に変換
//!
//! # 使用例
//!
//! ```
//! use graph::{is_tree, connectivity, tree_drop_parent};
//!
//! // 木の判定
//! let edges = vec![(0, 1), (1, 2), (2, 3)];
//! assert!(is_tree(4, &edges));
//!
//! // 連結性の確認
//! assert!(connectivity(4, &edges));
//!
//! // 根付き木への変換
//! let (children, parent) = tree_drop_parent(4, 0, &edges);
//! assert_eq!(parent, vec![0, 0, 1, 2]); // 各ノードの親
//! ```
//!
//! # 計算量
//!
//! - `is_tree`: O(E) (E: 辺数)
//! - `connectivity`: O(V + E) (V: 頂点数, E: 辺数)  
//! - `tree_drop_parent`: O(V + E)

use std::mem;

/// グラフが木かどうかを判定します。
///
/// 木の条件：
/// - 連結である
/// - 辺の数が頂点数 - 1 である
/// - 閉路を持たない
///
/// # 引数
///
/// - `n`: 頂点数
/// - `edges`: 辺のリスト。各辺は `(u, v)` の形式で表現
///
/// # 戻り値
///
/// グラフが木の場合 `true`、そうでなければ `false`
///
/// # 計算量
///
/// O(E) (E: 辺数)
///
/// # Examples
///
/// ```
/// use graph::is_tree;
///
/// // 単純なパス: 0-1-2-3
/// let edges = vec![(0, 1), (1, 2), (2, 3)];
/// assert!(is_tree(4, &edges));
///
/// // 星型グラフ: 中央ノード0から1,2,3に接続
/// let edges = vec![(0, 1), (0, 2), (0, 3)];
/// assert!(is_tree(4, &edges));
///
/// // 閉路があるため木ではない
/// let edges = vec![(0, 1), (1, 2), (2, 0)];
/// assert!(!is_tree(3, &edges));
///
/// // 非連結のため木ではない
/// let edges = vec![(0, 1), (2, 3)];
/// assert!(!is_tree(4, &edges));
/// ```
pub fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
    for &(a, b) in edges {
        assert!(a < n);
        assert!(b < n);
        assert_ne!(a, b);
    }

    if n == 0 {
        return true;
    }

    edges.len() == n - 1 && connectivity(n, edges)
}

/// グラフが連結かどうかを判定します。
///
/// DFS（深さ優先探索）を使用して、ノード0から全てのノードに到達できるかを確認します。
///
/// # 引数
///
/// - `n`: 頂点数
/// - `edges`: 辺のリスト。各辺は `(u, v)` の形式で表現
///
/// # 戻り値
///
/// グラフが連結の場合 `true`、そうでなければ `false`
///
/// # 計算量
///
/// O(V + E) (V: 頂点数, E: 辺数)
///
/// # Examples
///
/// ```
/// use graph::connectivity;
///
/// // 連結グラフ
/// let edges = vec![(0, 1), (1, 2), (2, 3)];
/// assert!(connectivity(4, &edges));
///
/// // 非連結グラフ（2つの成分に分かれている）
/// let edges = vec![(0, 1), (2, 3)];
/// assert!(!connectivity(4, &edges));
///
/// // 星型グラフ（連結）
/// let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
/// assert!(connectivity(5, &edges));
/// ```
pub fn connectivity(n: usize, edges: &[(usize, usize)]) -> bool {
    fn dfs(i: usize, g: &[Vec<usize>], visited: &mut Vec<bool>) {
        for &j in &g[i] {
            if visited[j] {
                continue;
            }
            visited[j] = true;
            dfs(j, g, visited);
        }
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, &g, &mut visited);
    visited.iter().filter(|&&f| f).count() == n
}

/// 無向グラフを根付き木に変換します。
///
/// 指定された根ノードを基準として、無向グラフを根付き木に変換し、
/// 各ノードの子ノードリストと親ノードを返します。
///
/// # 引数
///
/// - `n`: 頂点数
/// - `root`: 根ノードのインデックス
/// - `edges`: 辺のリスト。木構造を表現している必要があります
///
/// # 戻り値
///
/// タプル `(children, parent)`:
/// - `children`: 各ノードの子ノードのリスト
/// - `parent`: 各ノードの親ノード。根ノードの親は自分自身
///
/// # 計算量
///
/// O(V + E) (V: 頂点数, E: 辺数)
///
/// # パニック条件
///
/// 入力が木構造でない場合、debug モードでパニックします。
///
/// # Examples
///
/// ```
/// use graph::tree_drop_parent;
///
/// // 線形な木: 0-1-2-3 (根: 0)
/// let edges = vec![(0, 1), (1, 2), (2, 3)];
/// let (children, parent) = tree_drop_parent(4, 0, &edges);
/// assert_eq!(children, vec![vec![1], vec![2], vec![3], vec![]]);
/// assert_eq!(parent, vec![0, 0, 1, 2]);
///
/// // 星型グラフ: 中央ノード1が根
/// let edges = vec![(1, 0), (1, 2), (1, 3)];
/// let (children, parent) = tree_drop_parent(4, 1, &edges);
/// assert_eq!(children, vec![vec![], vec![0, 2, 3], vec![], vec![]]);
/// assert_eq!(parent, vec![1, 1, 1, 1]);
///
/// // 二分木の例
/// //     0
/// //    / \
/// //   1   2
/// //  /
/// // 3
/// let edges = vec![(0, 1), (0, 2), (1, 3)];
/// let (children, parent) = tree_drop_parent(4, 0, &edges);
/// assert_eq!(children, vec![vec![1, 2], vec![3], vec![], vec![]]);
/// assert_eq!(parent, vec![0, 0, 0, 1]);
/// ```
pub fn tree_drop_parent(
    n: usize,
    root: usize,
    edges: &[(usize, usize)],
) -> (Vec<Vec<usize>>, Vec<usize>) {
    debug_assert!(is_tree(n, edges));

    fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, parent: &mut Vec<usize>) {
        parent[i] = p;
        for &j in &g[i] {
            if j == p {
                continue;
            }
            dfs(j, i, g, parent);
        }
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut parent = vec![usize::MAX; n];
    dfs(root, root, &g, &mut parent);

    for i in 0..n {
        g[i] = mem::take(&mut g[i])
            .into_iter()
            .filter(|&j| j != parent[i])
            .collect();
    }

    (g, parent)
}

#[cfg(test)]
mod tests {
    use crate::{is_tree, tree_drop_parent};

    #[test]
    fn test_is_tree_small() {
        assert_eq!(is_tree(0, &[]), true);
        assert_eq!(is_tree(1, &[]), true);
        assert_eq!(is_tree(2, &[(0, 1)]), true);
        assert_eq!(is_tree(3, &[(0, 1), (1, 2)]), true);
        assert_eq!(is_tree(4, &[(0, 1), (0, 2), (0, 3)]), true);
        assert_eq!(is_tree(4, &[(0, 1), (1, 2), (0, 3)]), true);
        assert_eq!(is_tree(4, &[(0, 1), (2, 3)]), false);
        assert_eq!(is_tree(4, &[(0, 1), (1, 2), (2, 0)]), false);
    }

    #[test]
    fn test_drop_parent() {
        assert_eq!(
            // 0 (root) -- 1 -- 2 -- 3
            tree_drop_parent(4, 0, &[(0, 1), (1, 2), (2, 3)]),
            (
                vec![vec![1], vec![2], vec![3], vec![]], // g
                vec![0, 0, 1, 2],                        // parent
            )
        );
    }
}
