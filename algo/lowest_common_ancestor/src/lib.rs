use ceil_log2::CeilLog2;

/// 頂点 `0` を根とする根付き木の LCA を求めます。
///
/// # Examples
/// ```
/// use lowest_common_ancestor::LowestCommonAncestor;
///
/// // 0 -- 2 -- 4
/// // |    |
/// // 1    3
///
/// let lca = LowestCommonAncestor::new(5, &[(0, 1), (0, 2), (2, 3), (2, 4)]);
/// assert_eq!(lca.get(0, 1), 0);
/// assert_eq!(lca.get(0, 4), 0);
/// assert_eq!(lca.get(1, 1), 1);
/// assert_eq!(lca.get(1, 2), 0);
/// assert_eq!(lca.get(2, 3), 2);
/// assert_eq!(lca.get(3, 4), 2);
/// ```
pub struct LowestCommonAncestor {
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

const ILLEGAL: usize = std::usize::MAX;

impl LowestCommonAncestor {
    /// 頂点数と `n` と木をなす無向辺の集合 `edges` を渡します。
    pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = vec![vec![]; n];
        for &(u, v) in edges {
            g[u].push(v);
            g[v].push(u);
        }
        let mut depth = vec![0; n];
        let mut parent = vec![ILLEGAL; n];
        let mut stack = vec![(0, ILLEGAL)];
        while let Some((u, p)) = stack.pop() {
            for &v in &g[u] {
                if v != p {
                    depth[v] = depth[u] + 1;
                    parent[v] = u;
                    stack.push((v, u));
                }
            }
        }
        let table_size = n.ceil_log2().max(1);
        let mut ancestor = vec![vec![ILLEGAL; n]; table_size];
        ancestor[0] = parent;
        for i in 1..table_size {
            ancestor[i] = (0..n)
                .map(|v| {
                    if ancestor[i - 1][v] == ILLEGAL {
                        ILLEGAL
                    } else {
                        ancestor[i - 1][ancestor[i - 1][v]]
                    }
                })
                .collect();
        }
        Self { ancestor, depth }
    }

    /// `u` と `v` の LCA を返します。
    pub fn get(&self, u: usize, v: usize) -> usize {
        assert!(u < self.depth.len());
        assert!(v < self.depth.len());
        let (mut u, mut v) = if self.depth[u] >= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[u] >= self.depth[v]);
        let depth_diff = self.depth[u] - self.depth[v];
        for i in 0..self.ancestor.len() {
            if depth_diff >> i & 1 == 1 {
                u = self.ancestor[i][u];
            }
        }
        if u == v {
            return u;
        }
        for i in (0..self.ancestor.len()).rev() {
            if self.ancestor[i][u] != self.ancestor[i][v] {
                u = self.ancestor[i][u];
                v = self.ancestor[i][v];
            }
        }
        let lca = self.ancestor[0][u];
        assert_ne!(lca, ILLEGAL);
        lca
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

    /// 頂点 `u` から根の方向に `2^i` 本の辺を登って着く頂点を返します。
    pub fn ancestor(&self, i: usize, u: usize) -> Option<usize> {
        if i >= self.ancestor.len() {
            None
        } else {
            let a = self.ancestor[i][u];
            if a == ILLEGAL {
                None
            } else {
                Some(a)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LowestCommonAncestor;

    #[test]
    fn single_node_test() {
        let lca = LowestCommonAncestor::new(1, &[]);
        assert_eq!(lca.get(0, 0), 0);
    }
}
