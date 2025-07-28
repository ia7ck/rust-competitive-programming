//! 強連結成分分解 (Strongly Connected Components) を行うライブラリです。
//!
//! 有向グラフの強連結成分分解を Kosaraju のアルゴリズムで実装しています。
//! 強連結成分とは、任意の2頂点間で相互に到達可能な頂点の集合です。
//!
//! # 計算量
//!
//! - 時間計算量: O(V + E)
//! - 空間計算量: O(V + E)
//!
//! ここで V は頂点数、E は辺数です。
//!
//! # 用途
//!
//! - 有向グラフの強連結成分への分解
//! - 2-SATの充足可能性判定
//! - 有向グラフの凝縮（DAG化）
//! - 競技プログラミングでのグラフ問題解決
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use strongly_connected_components::strongly_connected_components;
//!
//! // 3つの頂点からなる循環グラフ: 0 -> 1 -> 2 -> 0
//! let components = strongly_connected_components(3, &[(0, 1), (1, 2), (2, 0)]);
//! // 全体が1つの強連結成分
//! assert_eq!(components.len(), 1);
//! let mut component = components[0].clone();
//! component.sort();
//! assert_eq!(component, vec![0, 1, 2]);
//! ```
//!
//! ## 複数の強連結成分を持つグラフ
//!
//! ```
//! use strongly_connected_components::strongly_connected_components;
//!
//! // グラフ: 0 -> 1 <-> 2, 3 -> 4
//! let components = strongly_connected_components(5, &[(0, 1), (1, 2), (2, 1), (3, 4)]);
//! assert_eq!(components.len(), 4); // 4つの強連結成分
//! 
//! // 各成分のサイズを確認
//! let sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
//! assert!(sizes.contains(&1)); // 単独頂点の成分
//! assert!(sizes.contains(&2)); // 2頂点の成分 {1, 2}
//! ```

/// 強連結成分分解を行います。[参考](https://manabitimes.jp/math/1250)
///
/// Kosaraju のアルゴリズムを使用して、有向グラフを強連結成分に分解します。
/// 
/// # 引数
/// 
/// - `n`: 頂点数（頂点は 0, 1, ..., n-1 で番号付けされます）
/// - `edges`: 有向辺のリスト。各要素 `(u, v)` は頂点 u から頂点 v への辺を表します
///
/// # 戻り値
///
/// `Vec<Vec<usize>>` を返します。各内側のベクタは1つの強連結成分を構成する頂点のリストです。
/// 強連結成分はトポロジカル順序の逆順で返されます（後ろの成分から前の成分への辺は存在しません）。
///
/// # Examples
///
/// ```
/// use strongly_connected_components::strongly_connected_components;
///
/// // 単純な循環グラフ
/// let scc = strongly_connected_components(3, &[(0, 1), (1, 2), (2, 0)]);
/// assert_eq!(scc.len(), 1); // 全体が1つの強連結成分
/// 
/// // 一方向のパス
/// let scc = strongly_connected_components(3, &[(0, 1), (1, 2)]);
/// assert_eq!(scc.len(), 3); // 各頂点が独立した強連結成分
/// ```
///
/// # 2-SAT での使用例
///
/// ```
/// use strongly_connected_components::strongly_connected_components;
///
/// // 2-SAT問題の例: (x1 ∨ ¬x2) ∧ (¬x1 ∨ x2)
/// // 変数の数
/// let n_vars = 2;
/// // 含意グラフを構築: ¬A → B は (A ∨ B) と等価
/// let mut edges = Vec::new();
/// 
/// // (x1 ∨ ¬x2) から ¬x1 → ¬x2, x2 → x1
/// edges.push((1, 3)); // ¬x1(1) → ¬x2(3)  
/// edges.push((2, 0)); // x2(2) → x1(0)
/// 
/// // (¬x1 ∨ x2) から x1 → x2, ¬x2 → ¬x1
/// edges.push((0, 2)); // x1(0) → x2(2)
/// edges.push((3, 1)); // ¬x2(3) → ¬x1(1)
/// 
/// let components = strongly_connected_components(2 * n_vars, &edges);
/// 
/// // 各変数 xi について、xi と ¬xi が同じ強連結成分にあるかチェック
/// let mut satisfiable = true;
/// for i in 0..n_vars {
///     let xi = i * 2;      // xi のインデックス
///     let not_xi = i * 2 + 1; // ¬xi のインデックス
///     
///     // xi と ¬xi が同じ強連結成分にあると充足不可能
///     let xi_component = components.iter().position(|c| c.contains(&xi)).unwrap();
///     let not_xi_component = components.iter().position(|c| c.contains(&not_xi)).unwrap();
///     
///     if xi_component == not_xi_component {
///         satisfiable = false;
///         break;
///     }
/// }
/// 
/// // この例では充足可能
/// assert!(satisfiable);
/// ```
pub fn strongly_connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    for &(u, v) in edges {
        graph[u].push(v);
    }

    let mut seen = vec![false; n];
    let mut order = Vec::new();
    let mut order_pushed = vec![false; n];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        let mut stack = Vec::new();
        stack.push(v);
        while let Some(x) = stack.pop() {
            seen[x] = true;
            stack.push(x);
            let mut pushed = false;
            for &y in &graph[x] {
                if !seen[y] {
                    stack.push(y);
                    pushed = true;
                }
            }
            if !pushed {
                debug_assert_eq!(stack.last(), Some(&x));
                stack.pop();
                if !order_pushed[x] {
                    order_pushed[x] = true;
                    order.push(x);
                }
            }
        }
    }
    assert_eq!(order.len(), n);

    let mut rev_graph = vec![vec![]; n];
    #[allow(clippy::needless_range_loop)]
    for u in 0..n {
        for &v in &graph[u] {
            rev_graph[v].push(u);
        }
    }

    let mut seen = vec![false; n];
    let mut component_id = vec![0; n];
    let mut id = 0;
    for &v in order.iter().rev() {
        if seen[v] {
            continue;
        }
        let mut stack = Vec::new();
        stack.push(v);
        while let Some(x) = stack.pop() {
            seen[x] = true;
            component_id[x] = id;
            for &y in &rev_graph[x] {
                if !seen[y] {
                    stack.push(y);
                }
            }
        }
        id += 1;
    }

    let mut components = vec![vec![]; id];
    for v in 0..n {
        components[component_id[v]].push(v);
    }
    components
}

#[cfg(test)]
mod tests {
    use crate::strongly_connected_components;

    #[test]
    fn test_single_node() {
        let scc = strongly_connected_components(1, &[]);
        assert_eq!(scc, vec![vec![0]]);
    }

    #[test]
    fn test_small() {
        // 0 -> 1
        assert_eq!(
            strongly_connected_components(2, &[(0, 1)]),
            vec![vec![0], vec![1]]
        );

        // 0 -> 1
        // 0 -> 1
        assert_eq!(
            strongly_connected_components(2, &[(0, 1), (0, 1)]),
            vec![vec![0], vec![1]]
        );

        // 0 <-> 1
        let mut scc = strongly_connected_components(2, &[(0, 1), (1, 0)]);
        for com in &mut scc {
            com.sort();
        }
        assert_eq!(scc, vec![vec![0, 1]]);
    }
}
