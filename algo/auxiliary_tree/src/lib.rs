//! Auxiliary Tree（補助木・仮想木）を構築するライブラリです。
//!
//! Auxiliary Tree は、元の木から指定された頂点集合とそれらの最小共通祖先（LCA）のみを
//! 含む部分木を効率的に構築する手法です。主に木上のクエリ処理の高速化に使用されます。
//!
//! # 計算量
//!
//! グラフの頂点数を n、指定された頂点集合のサイズを k として：
//! - 時間計算量: O(k log n + k log k)
//! - 空間計算量: O(k)
//! 
//! ※ HashMap のコストは無視しています
//!
//! # 用途
//!
//! - 木上での部分集合に対するクエリの高速化
//! - 競技プログラミングでの木構造問題の最適化
//! - 大きな木の中で特定の頂点群に関する計算を効率化
//!
//! # 前提条件
//!
//! - 元のグラフが木構造である
//! - LCA（最小共通祖先）が効率的に計算できる
//! - 各頂点に対して pre-order での訪問順序が与えられている
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use auxiliary_tree::auxiliary_tree;
//! use lowest_common_ancestor::LowestCommonAncestor;
//! use std::collections::HashMap;
//!
//! // 線形の木: 0 -- 1 -- 2 -- 3 -- 4
//! let lca = LowestCommonAncestor::new(5, 0, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
//! let inv_ord = vec![0, 1, 2, 3, 4]; // pre-order での順序
//!
//! // 頂点 {1, 3, 4} に対する Auxiliary Tree を構築
//! let (root, tree) = auxiliary_tree(&[1, 3, 4], &inv_ord, &lca);
//!
//! // ルートは 1（最も早く訪問される頂点）
//! assert_eq!(root, 1);
//! 
//! // 構築された木の構造を確認
//! assert!(tree.contains_key(&1));
//! assert!(tree.contains_key(&3));
//! assert!(tree.contains_key(&4));
//! ```

use std::collections::HashMap;

use lowest_common_ancestor::LowestCommonAncestor;

/// 指定された頂点集合に対する Auxiliary Tree を構築します。
///
/// [Auxiliary Tree](https://noshi91.github.io/algorithm-encyclopedia/auxiliary-tree) は、
/// 元の木から指定された頂点集合とそれらの LCA のみを含む最小の部分木です。
/// 
/// アルゴリズムの詳細は [参考記事](https://smijake3.hatenablog.com/entry/2019/09/15/200200) を参照してください。
///
/// # 引数
///
/// * `nodes`: 対象とする頂点の集合。{0, 1, ..., n-1} の部分集合である必要があります
/// * `inv_ord`: pre-order（行きがけ順）での各頂点の訪問順序
///   - 頂点 `i` は pre-order で `inv_ord[i]` 番目に訪問されます
/// * `lca`: 2頂点間の LCA を計算する構造体。`.get(u, v)` メソッドを持つ必要があります
///
/// # 戻り値
///
/// `(root, graph)` のタプルを返します：
/// - `root`: 構築された Auxiliary Tree のルート頂点
/// - `graph`: HashMap で表現された木構造
///   - `graph.contains_key(&i)`: 頂点 `i` が Auxiliary Tree に含まれる
///   - `graph[&i]`: 頂点 `i` の子頂点のリスト
///   - `!graph.contains_key(&i)`: 頂点 `i` は Auxiliary Tree に含まれない
///
/// # Panics
///
/// `nodes` が空の場合にパニックします。
///
/// # Examples
///
/// ```
/// use auxiliary_tree::auxiliary_tree;
/// use lowest_common_ancestor::LowestCommonAncestor;
///
/// // 線形の木: 0 -- 1 -- 2 -- 3 -- 4
/// let lca = LowestCommonAncestor::new(5, 0, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
/// let inv_ord = vec![0, 1, 2, 3, 4];
///
/// // 単一頂点の場合
/// let (root, tree) = auxiliary_tree(&[2], &inv_ord, &lca);
/// assert_eq!(root, 2);
/// assert_eq!(tree.get(&2), Some(&vec![])); // 子はなし
/// ```
///
/// # 競技プログラミングでの応用例
///
/// ```
/// use auxiliary_tree::auxiliary_tree;
/// use lowest_common_ancestor::LowestCommonAncestor;
/// use std::collections::HashMap;
///
/// // 木上のクエリ問題での使用例
/// // 例：指定された頂点群を含む最小の部分木のサイズを求める
/// fn solve_tree_query(
///     n: usize,
///     edges: &[(usize, usize)], 
///     query_nodes: &[usize]
/// ) -> usize {
///     if query_nodes.is_empty() {
///         return 0;
///     }
///     
///     let lca = LowestCommonAncestor::new(n, 0, edges);
///     
///     // DFS で pre-order を計算（簡略化）
///     let inv_ord: Vec<usize> = (0..n).collect();
///     
///     let (_, aux_tree) = auxiliary_tree(query_nodes, &inv_ord, &lca);
///     
///     // Auxiliary Tree のサイズが答え
///     aux_tree.len()
/// }
///
/// // テスト
/// let edges = vec![(0, 1), (1, 2), (1, 3), (3, 4)];
/// let query = vec![2, 4];
/// let result = solve_tree_query(5, &edges, &query);
/// assert!(result >= 2); // 少なくとも指定された頂点は含まれる
/// ```
pub fn auxiliary_tree(
    nodes: &[usize],
    inv_ord: &[usize],
    lca: &LowestCommonAncestor, // trait にする？
) -> (usize, HashMap<usize, Vec<usize>>) {
    // https://smijake3.hatenablog.com/entry/2019/09/15/200200

    assert!(!nodes.is_empty());

    // nodes.len() < 2 だと .windows(2) が空になるので場合分け
    if nodes.len() == 1 {
        return (nodes[0], HashMap::from([(nodes[0], vec![])]));
    }

    let mut nodes = nodes.to_vec();
    nodes.sort_by_key(|&i| inv_ord[i]);

    let lca_nodes = nodes
        .windows(2)
        .map(|w| lca.get(w[0], w[1]))
        .collect::<Vec<_>>();
    nodes.extend(lca_nodes);
    nodes.sort_by_key(|&i| inv_ord[i]);
    nodes.dedup();

    let mut h = HashMap::<_, Vec<_>>::new();
    for w in nodes.windows(2) {
        // stack 使わずにこれでよさそう
        let x = lca.get(w[0], w[1]);
        h.entry(x).or_insert_with(Vec::new).push(w[1]);
        assert!(!h.contains_key(&w[1]));
        h.insert(w[1], vec![]);
    }
    (nodes[0], h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        //                  *         *
        // 0 (root) -- 1 -- 2 -- 3 -- 4
        assert_eq!(
            auxiliary_tree(
                &[2, 4],
                &[0, 1, 2, 3, 4],
                &LowestCommonAncestor::new(5, 0, &[(0, 1), (1, 2), (2, 3), (3, 4)])
            ),
            (2, HashMap::from([(2, vec![4]), (4, vec![])]))
        );
    }
}
