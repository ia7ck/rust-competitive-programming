/// 有向グラフの頂点をトポロジカル順に並べて返します。グラフが DAG でなければ None を返します。
///
/// # Examples
/// ```
/// use topological_sort::topological_sort;
///
/// let order = topological_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
/// assert!(order == Some(vec![0, 1, 2, 3]) || order == Some(vec![0, 2, 1, 3]));
/// ```
pub fn topological_sort(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    use std::collections::VecDeque;

    let mut g = vec![vec![]; n];
    let mut in_deg = vec![0; n];
    for &(s, t) in edges {
        g[s].push(t);
        in_deg[t] += 1;
    }

    let mut order = Vec::new();
    let mut que = VecDeque::new();
    #[allow(clippy::needless_range_loop)]
    for s in 0..n {
        if in_deg[s] == 0 {
            order.push(s);
            que.push_back(s);
        }
    }
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                order.push(v);
                que.push_back(v);
            }
        }
    }
    assert!(order.len() <= n);
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::topological_sort;

    #[test]
    fn two_ways() {
        let order = topological_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        assert!(order == Some(vec![0, 1, 2, 3]) || order == Some(vec![0, 2, 1, 3]));
    }

    #[test]
    fn line() {
        let order = topological_sort(4, &[(0, 1), (1, 2), (2, 3)]);
        assert_eq!(order, Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn contain_cycle() {
        let order = topological_sort(5, &[(0, 1), (1, 2), (2, 3), (3, 1), (3, 4)]);
        assert_eq!(order, None);
    }
}
