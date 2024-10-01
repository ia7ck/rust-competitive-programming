use std::collections::HashMap;

use lowest_common_ancestor::LowestCommonAncestor;

/// [auxiliary tree](https://noshi91.github.io/algorithm-encyclopedia/auxiliary-tree) です。
///
/// # 計算量
///
/// hash map のコストは無視する。
///
/// グラフの頂点数を n, `nodes.len()` を k として、O(klogn + klogk)
///
/// # 引数
///
/// * `nodes`： {0, 1, ..., n - 1} の部分集合
/// * `inv_ord`: pre-order (行きがけ順) の列の添字と値を反対にしたもの
///     * 頂点 `i` は行きがけ順で `inv_ord[i]` 番目に現われる
/// * `lca`: 2頂点の LCA を得る `.get(u, v)` を実装した構造体
///
/// # 返り値
///
/// 返り値を `g` とすると
///
/// * ` g.contains_key(&i)`: 頂点 `i` が圧縮後の木に含まれて、子のリストが `g[&i]` である
/// * `!g.contains_key(&i)`: 頂点 `i` が圧縮後の木に含まれない
pub fn auxiliary_tree(
    nodes: &[usize],
    inv_ord: &[usize],
    lca: &LowestCommonAncestor, // trait にする？
) -> HashMap<usize, Vec<usize>> {
    // https://smijake3.hatenablog.com/entry/2019/09/15/200200

    // nodes.len() < 2 だと .windows(2) が空になるので場合分け
    if nodes.is_empty() {
        return HashMap::new();
    }
    if nodes.len() == 1 {
        return HashMap::from([(nodes[0], vec![])]);
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
    h
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
            HashMap::from([(2, vec![4]), (4, vec![])])
        );
    }
}
