use std::collections::VecDeque;

/// 根付き木の LCA です。
///
/// # Examples
/// ```
/// use lowest_common_ancestor::LowestCommonAncestor;
///
/// // 0 -- 2 -- 4
/// // |    |
/// // 1    3
///
/// let lca = LowestCommonAncestor::new(5, 0, &[(0, 1), (0, 2), (2, 3), (2, 4)]);
/// assert_eq!(lca.get(0, 1), 0);
/// assert_eq!(lca.get(0, 4), 0);
/// assert_eq!(lca.get(1, 1), 1);
/// assert_eq!(lca.get(1, 2), 0);
/// assert_eq!(lca.get(2, 3), 2);
/// assert_eq!(lca.get(3, 4), 2);
/// ```
#[derive(Debug, Clone)]
pub struct LowestCommonAncestor {
    n: usize,
    // ancestor[i][v] := v から根の方向に 2^i 進んだ頂点
    ancestor: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl LowestCommonAncestor {
    /// 頂点数 `n`, 根 `root`, 木をなす無向辺の集合 `edges` を渡します。
    pub fn new(n: usize, root: usize, edges: &[(usize, usize)]) -> Self {
        assert!(root < n);
        let mut g = vec![vec![]; n];
        for &(u, v) in edges {
            assert!(u < n);
            assert!(v < n);
            g[u].push(v);
            g[v].push(u);
        }

        let mut depth = vec![0; n];
        let mut parent = vec![None; n];
        let mut que = VecDeque::new();
        depth[root] = 0;
        parent[root] = None;
        que.push_back((root, None));
        while let Some((curr, prev)) = que.pop_front() {
            for &next in &g[curr] {
                if prev.is_some_and(|prev| prev == next) {
                    continue;
                }
                depth[next] = depth[curr] + 1;
                parent[next] = Some(curr);
                que.push_back((next, Some(curr)));
            }
        }

        let table_size = if n == 1 {
            1
        } else {
            // log2(n) の切り上げ
            n.ilog2() as usize + usize::from(!n.is_power_of_two())
        };

        let mut ancestor = vec![vec![None; n]; table_size];
        ancestor[0] = parent;
        for i in 1..table_size {
            ancestor[i] = (0..n)
                .map(|v| ancestor[i - 1][v].and_then(|a| ancestor[i - 1][a]))
                .collect();
        }

        Self { n, ancestor, depth }
    }

    /// `u` と `v` の LCA を返します。
    pub fn get(&self, u: usize, v: usize) -> usize {
        assert!(u < self.n);
        assert!(v < self.n);
        let (mut u, mut v) = if self.depth[u] >= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[u] >= self.depth[v]);

        for i in 0..self.ancestor.len() {
            let depth_diff = self.depth[u] - self.depth[v];
            if depth_diff == 0 {
                break;
            }
            if depth_diff >> i & 1 == 1 {
                u = self.ancestor[i][u].unwrap();
            }
        }
        assert_eq!(self.depth[u], self.depth[v]);

        if u == v {
            return u;
        }

        for i in (0..self.ancestor.len()).rev() {
            match (self.ancestor[i][u], self.ancestor[i][v]) {
                (Some(au), Some(av)) if au != av => {
                    u = au;
                    v = av;
                }
                _ => {}
            }
        }
        self.ancestor[0][u].unwrap()
    }

    /// `u` と `v` の距離 (頂点間にある辺の数) を返します。
    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        let w = self.get(u, v);
        self.depth[u] + self.depth[v] - self.depth[w] * 2
    }

    /// 頂点 `u` の深さを返します。
    pub fn depth(&self, u: usize) -> usize {
        self.depth[u]
    }

    /// 頂点 `u` から根の方向に `k` 本の辺を登って着く頂点を返します。
    pub fn kth_parent(&self, u: usize, k: usize) -> Option<usize> {
        assert!(u < self.n);
        if k > self.depth[u] {
            return None;
        }
        let mut u = u;
        for i in 0..self.ancestor.len() {
            if k >> i & 1 == 1 {
                u = self.ancestor[i][u].unwrap();
            }
        }
        Some(u)
    }
}

#[cfg(test)]
mod tests {
    use crate::LowestCommonAncestor;

    #[test]
    fn single_node_test() {
        let lca = LowestCommonAncestor::new(1, 0, &[]);
        assert_eq!(lca.get(0, 0), 0);
    }

    #[test]
    fn test_kth_parent() {
        let lca = LowestCommonAncestor::new(5, 0, &[(0, 1), (1, 2), (2, 3), (3, 4)]);

        assert_eq!(lca.kth_parent(0, 0), Some(0));
        assert_eq!(lca.kth_parent(0, 1), None);

        assert_eq!(lca.kth_parent(1, 0), Some(1));
        assert_eq!(lca.kth_parent(1, 1), Some(0));
        assert_eq!(lca.kth_parent(1, 2), None);

        assert_eq!(lca.kth_parent(2, 0), Some(2));
        assert_eq!(lca.kth_parent(2, 1), Some(1));
        assert_eq!(lca.kth_parent(2, 2), Some(0));
        assert_eq!(lca.kth_parent(2, 3), None);

        assert_eq!(lca.kth_parent(3, 0), Some(3));
        assert_eq!(lca.kth_parent(3, 1), Some(2));
        assert_eq!(lca.kth_parent(3, 2), Some(1));
        assert_eq!(lca.kth_parent(3, 3), Some(0));
        assert_eq!(lca.kth_parent(3, 4), None);

        assert_eq!(lca.kth_parent(4, 0), Some(4));
        assert_eq!(lca.kth_parent(4, 1), Some(3));
        assert_eq!(lca.kth_parent(4, 2), Some(2));
        assert_eq!(lca.kth_parent(4, 3), Some(1));
        assert_eq!(lca.kth_parent(4, 4), Some(0));
        assert_eq!(lca.kth_parent(4, 5), None);
    }
}
