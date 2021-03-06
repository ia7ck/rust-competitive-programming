use ceil_log2::CeilLog2;

/// 頂点 `0` を根とする根付き木の LCA を求めます。
pub struct LowestCommonAncestor {
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

const ILLEGAL: usize = std::usize::MAX;

impl LowestCommonAncestor {
    /// 木を隣接グラフ形式で渡します。
    pub fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut depth = vec![0; n];
        let mut parent = vec![ILLEGAL; n];
        let mut stack = Vec::new();
        stack.push((0, ILLEGAL));
        while let Some((u, p)) = stack.pop() {
            for &v in &g[u] {
                if v != p {
                    depth[v] = depth[u] + 1;
                    parent[v] = u;
                    stack.push((v, u));
                }
            }
        }
        let n = g.len();
        let table_size = n.ceil_log2();
        let mut ancestor = vec![parent];
        for i in 1..table_size {
            let a: Vec<usize> = (0..n)
                .map(|v| {
                    if ancestor[i - 1][v] == ILLEGAL {
                        ILLEGAL
                    } else {
                        ancestor[i - 1][ancestor[i - 1][v]]
                    }
                })
                .collect();
            ancestor.push(a);
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

    pub fn depth(&self) -> &Vec<usize> {
        &self.depth
    }

    pub fn ancestor(&self) -> &Vec<Vec<usize>> {
        &self.ancestor
    }
}

#[cfg(test)]
mod tests {
    use crate::LowestCommonAncestor;

    #[test]
    fn single_node_test() {
        let g = vec![vec![]];
        let lca = LowestCommonAncestor::new(&g);
        assert_eq!(lca.get(0, 0), 0);
    }
}
