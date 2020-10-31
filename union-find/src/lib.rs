/// Union Find はグラフの連結成分を管理します。
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// グラフの頂点数 `n` を渡します。
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).map(|i| i).collect::<Vec<_>>(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            self.par[i]
        } else {
            self.par[i] = self.find(self.par[i]);
            self.par[i]
        }
    }
    /// 頂点 `i` の属する連結成分と頂点 `j` の属する連結成分をつなげます。
    pub fn unite(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        let (i, j) = if self.size[i] >= self.size[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.par[j] = i;
        self.size[i] += self.size[j];
    }
    /// 頂点 `i` の属する連結成分のサイズ (頂点数) を返します。
    ///
    /// # Examples
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    /// assert_eq!(uf.get_size(0), 3);
    /// assert_eq!(uf.get_size(1), 3);
    /// assert_eq!(uf.get_size(2), 3);
    /// assert_eq!(uf.get_size(3), 2);
    /// assert_eq!(uf.get_size(4), 2);
    /// assert_eq!(uf.get_size(5), 1);
    /// ```
    pub fn get_size(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
    /// 頂点 `i` と頂点 `j` が同じ連結成分に属するかどうかを返します。
    ///  
    /// # Examples
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// assert!(uf.same(0, 0));
    /// assert!(uf.same(3, 3));
    /// assert!(uf.same(5, 5));
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    /// assert!(uf.same(0, 1));
    /// assert!(uf.same(1, 2));
    /// assert!(uf.same(0, 2));
    /// assert!(uf.same(3, 4));
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
    /// 各連結成分の代表元をベクタで返します。
    ///
    /// # Examples
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    /// let leaders = uf.leaders();
    /// for &i in &leaders {
    ///     for &j in &leaders {
    ///         if i != j {
    ///             assert!(!uf.same(i, j));
    ///         }
    ///     }
    /// }
    /// ```
    pub fn leaders(&mut self) -> Vec<usize> {
        let n = self.par.len();
        let mut seen = vec![false; n];
        (0..n)
            .filter(|&i| {
                let p = self.find(i);
                if seen[p] {
                    return false;
                }
                seen[p] = true;
                true
            })
            .collect()
    }
}
