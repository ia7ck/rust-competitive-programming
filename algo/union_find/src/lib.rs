//! Union Find（素集合データ構造）のライブラリです。
//!
//! Union Find は互いに素な集合を効率的に管理するデータ構造です。
//! 主に以下の操作を高速に実行できます：
//! - 2つの要素が同じ集合に属するかの判定
//! - 2つの集合の統合
//! - 各集合のサイズの取得
//!
//! # 計算量
//!
//! - 初期化: O(n)
//! - 各操作（unite, find, same, size）: ほぼ O(α(n))
//!   - α(n) は逆アッカーマン関数（実用上は定数）
//!
//! # 最適化技法
//!
//! - **経路圧縮**: find 操作時に親へのパスを圧縮
//! - **union by size**: 統合時に小さい木を大きい木に接続
//!
//! # 用途
//!
//! - グラフの連結性判定
//! - 最小全域木アルゴリズム（Kruskal法）
//! - 同値関係の管理
//! - 競技プログラミングでのグラフ・集合問題
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use union_find::UnionFind;
//!
//! let mut uf = UnionFind::new(5);
//! 
//! // 集合の統合
//! uf.unite(0, 1);
//! uf.unite(2, 3);
//! 
//! // 連結性の確認
//! assert!(uf.same(0, 1));
//! assert!(!uf.same(0, 2));
//! 
//! // 集合のサイズ
//! assert_eq!(uf.size(0), 2); // {0, 1}
//! assert_eq!(uf.size(2), 2); // {2, 3}
//! assert_eq!(uf.size(4), 1); // {4}
//! 
//! // 連結成分の数
//! assert_eq!(uf.count_groups(), 3);
//! ```
//!
//! ## Kruskal法での最小全域木
//!
//! ```
//! use union_find::UnionFind;
//!
//! // 辺を重みでソート: (重み, 始点, 終点)
//! let mut edges = vec![(1, 0, 1), (2, 1, 2), (3, 0, 2), (4, 2, 3)];
//! edges.sort_by_key(|&(w, _, _)| w);
//! 
//! let n = 4; // 頂点数
//! let mut uf = UnionFind::new(n);
//! let mut mst_weight = 0;
//! let mut mst_edges = Vec::new();
//! 
//! for (weight, u, v) in edges {
//!     if uf.unite(u, v) {
//!         mst_weight += weight;
//!         mst_edges.push((u, v));
//!     }
//! }
//! 
//! assert_eq!(mst_weight, 7); // 1 + 2 + 4
//! assert_eq!(mst_edges.len(), n - 1); // n-1 本の辺
//! ```
//!
//! ## グラフの連結成分数の計算
//!
//! ```
//! use union_find::UnionFind;
//!
//! fn count_connected_components(n: usize, edges: &[(usize, usize)]) -> usize {
//!     let mut uf = UnionFind::new(n);
//!     for &(u, v) in edges {
//!         uf.unite(u, v);
//!     }
//!     uf.count_groups()
//! }
//!
//! // グラフの例: 0-1, 2-3 の2つの連結成分
//! let edges = vec![(0, 1), (2, 3)];
//! assert_eq!(count_connected_components(5, &edges), 3);
//! // 連結成分: {0,1}, {2,3}, {4}
//! ```

/// Union Find はグラフの連結成分を管理します。
#[derive(Clone, Debug)]
pub struct UnionFind {
    nodes: Vec<NodeKind>,
    groups: usize,
}

#[derive(Clone, Copy, Debug)]
enum NodeKind {
    Root { size: usize },
    Child { parent: usize },
}

impl UnionFind {
    /// 指定された頂点数で Union Find を初期化します。
    ///
    /// 初期状態では各頂点が独立した連結成分を形成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    ///
    /// let uf = UnionFind::new(5);
    /// // 初期状態: {0}, {1}, {2}, {3}, {4} の5つの連結成分
    /// assert_eq!(uf.count_groups(), 5);
    /// ```
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![NodeKind::Root { size: 1 }; n],
            groups: n,
        }
    }

    /// 頂点 `i` の属する連結成分の代表元を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    ///
    /// // [(0, 1, 2), (3, 4), (5)]
    /// assert_eq!(uf.find(0), uf.find(0));
    /// assert_eq!(uf.find(0), uf.find(1));
    /// assert_eq!(uf.find(1), uf.find(2));
    /// assert_eq!(uf.find(0), uf.find(2));
    /// assert_eq!(uf.find(3), uf.find(4));
    ///
    /// assert_ne!(uf.find(0), uf.find(3));
    /// assert_ne!(uf.find(0), uf.find(5));
    /// ```
    pub fn find(&mut self, i: usize) -> usize {
        assert!(i < self.nodes.len());

        match self.nodes[i] {
            NodeKind::Root { .. } => i,
            NodeKind::Child { parent } => {
                let root = self.find(parent);
                if root == parent {
                    // noop
                } else {
                    // 経路圧縮
                    self.nodes[i] = NodeKind::Child { parent: root };
                }
                root
            }
        }
    }

    /// 頂点 `i` の属する連結成分と頂点 `j` の属する連結成分をつなげます。
    ///
    /// 呼び出し前に別の連結成分だった場合 true を、同じ連結成分だった場合 false を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// assert!(uf.unite(0, 1));
    /// assert!(uf.unite(1, 2));
    /// assert!(uf.unite(3, 4));
    ///
    /// // [(0, 1, 2), (3, 4), (5)]
    /// assert!(!uf.unite(0, 2));
    /// assert!(!uf.unite(3, 3));
    ///
    /// assert!(uf.unite(4, 5));
    /// ```
    pub fn unite(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return false;
        }

        match (self.nodes[i], self.nodes[j]) {
            (NodeKind::Root { size: i_size }, NodeKind::Root { size: j_size }) => {
                let total = i_size + j_size;
                // マージテク
                if i_size >= j_size {
                    self.nodes[j] = NodeKind::Child { parent: i };
                    self.nodes[i] = NodeKind::Root { size: total };
                } else {
                    self.nodes[i] = NodeKind::Child { parent: j };
                    self.nodes[j] = NodeKind::Root { size: total };
                }
            }
            _ => unreachable!(),
        }

        self.groups -= 1;
        true
    }

    /// 頂点 `i` の属する連結成分のサイズ (頂点数) を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    ///
    /// // [(0, 1, 2), (3, 4), (5)]
    /// assert_eq!(uf.size(0), 3);
    /// assert_eq!(uf.size(1), 3);
    /// assert_eq!(uf.size(2), 3);
    /// assert_eq!(uf.size(3), 2);
    /// assert_eq!(uf.size(4), 2);
    /// assert_eq!(uf.size(5), 1);
    /// ```
    pub fn size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        match self.nodes[root] {
            NodeKind::Root { size } => size,
            _ => unreachable!(),
        }
    }

    /// 頂点 `i` と頂点 `j` が同じ連結成分に属するかどうかを返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// assert!(uf.same(0, 0));
    /// assert!(uf.same(3, 3));
    /// assert!(uf.same(5, 5));
    ///
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    ///
    /// // [(0, 1, 2), (3, 4), (5)]
    /// assert!(uf.same(0, 1));
    /// assert!(uf.same(1, 2));
    /// assert!(uf.same(0, 2));
    /// assert!(uf.same(3, 4));
    /// 
    /// assert!(!uf.same(0, 3));
    /// assert!(!uf.same(2, 5));
    /// ```
    ///
    /// ## 競技プログラミングでの応用例
    ///
    /// ```
    /// use union_find::UnionFind;
    ///
    /// // クエリ処理の例
    /// fn process_queries(n: usize, queries: &[(i32, usize, usize)]) -> Vec<bool> {
    ///     let mut uf = UnionFind::new(n);
    ///     let mut results = Vec::new();
    ///     
    ///     for &(query_type, u, v) in queries {
    ///         match query_type {
    ///             1 => { uf.unite(u, v); }, // 統合クエリ
    ///             2 => { results.push(uf.same(u, v)); }, // 判定クエリ
    ///             _ => {}
    ///         }
    ///     }
    ///     results
    /// }
    ///
    /// let queries = vec![(1, 0, 1), (1, 1, 2), (2, 0, 2), (2, 0, 3)];
    /// let results = process_queries(4, &queries);
    /// assert_eq!(results, vec![true, false]); // 0と2は連結、0と3は非連結
    /// ```
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    /// 連結成分数を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    ///
    /// // [(0, 1, 2), (3, 4), (5)]
    /// assert_eq!(uf.count_groups(), 3);
    /// ```
    pub fn count_groups(&self) -> usize {
        self.groups
    }
}
