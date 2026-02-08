use std::collections::VecDeque;

use doubling::{Doubling, Transition};

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
    doubling: Doubling<()>,
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

        let sentinel = n;
        let doubling = Doubling::new(n + 1, (n - 1).max(1), |i| {
            if i < n {
                let next = parent[i].unwrap_or(sentinel);
                Transition::new(next, ())
            } else {
                Transition::new(sentinel, ())
            }
        });

        Self { n, doubling, depth }
    }

    /// `u` と `v` の LCA を返します。
    pub fn get(&self, u: usize, v: usize) -> usize {
        assert!(u < self.n);
        assert!(v < self.n);

        if self.n == 1 {
            assert_eq!(u, 0);
            assert_eq!(v, 0);
            return 0;
        }

        let (u, v) = if self.depth[u] >= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[u] >= self.depth[v]);

        let u = self
            .doubling
            .fold(u, self.depth[u] - self.depth[v], u, |_, t| t.next);

        assert_eq!(self.depth[u], self.depth[v]);

        if u == v {
            return u;
        }

        let (mut u, mut v) = (u, v);
        let log = self.n.ilog2() as usize + usize::from(!self.n.is_power_of_two());
        for k in (0..log).rev() {
            let au = self.doubling.get(u, k).next;
            let av = self.doubling.get(v, k).next;
            if au != av {
                u = au;
                v = av;
            }
        }

        let lca = self.doubling.get(u, 0).next;
        assert_ne!(lca, self.n);

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

    /// 頂点 `u` から根の方向に `k` 本の辺を登って着く頂点を返します。
    pub fn kth_parent(&self, u: usize, k: usize) -> Option<usize> {
        assert!(u < self.n);
        if k > self.depth[u] {
            return None;
        }

        let result = self.doubling.fold(u, k, u, |_, t| t.next);
        // n is sentinel
        if result == self.n { None } else { Some(result) }
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
