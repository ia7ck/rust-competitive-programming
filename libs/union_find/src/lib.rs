/// Union Find はグラフの連結成分を管理します。
#[derive(Clone, Debug)]
pub struct UnionFind {
    nodes: Vec<NodeKind>,
    groups: usize,
}

#[derive(Clone, Debug)]
pub struct UniteResult {
    pub new_root: usize,
    pub child: usize,
}

#[derive(Clone, Copy, Debug)]
enum NodeKind {
    Root { size: usize },
    Child { parent: usize },
}

impl UnionFind {
    /// 頂点数を `n` として初期化します。
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
    /// もともと `i` と `j` が同じ連結成分だった場合は `None` を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// assert!(uf.unite(0, 1).is_some());
    /// assert!(uf.unite(1, 2).is_some());
    /// assert!(uf.unite(3, 4).is_some());
    ///
    /// // [(0, 1, 2), (3, 4), (5)]
    /// assert!(uf.unite(0, 2).is_none());
    /// assert!(uf.unite(3, 3).is_none());
    ///
    /// assert!(uf.unite(4, 5).is_some());
    /// ```
    pub fn unite(&mut self, i: usize, j: usize) -> Option<UniteResult> {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return None;
        }

        match (self.nodes[i], self.nodes[j]) {
            (NodeKind::Root { size: i_size }, NodeKind::Root { size: j_size }) => {
                self.groups -= 1;
                let total = i_size + j_size;
                // マージテク
                if i_size >= j_size {
                    self.nodes[j] = NodeKind::Child { parent: i };
                    self.nodes[i] = NodeKind::Root { size: total };
                    Some(UniteResult {
                        new_root: i,
                        child: j,
                    })
                } else {
                    self.nodes[i] = NodeKind::Child { parent: j };
                    self.nodes[j] = NodeKind::Root { size: total };
                    Some(UniteResult {
                        new_root: j,
                        child: i,
                    })
                }
            }
            _ => unreachable!(),
        }
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
