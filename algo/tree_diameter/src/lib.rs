use graph::is_tree;

pub fn tree_diameter(n: usize, edges: &[(usize, usize, u64)]) -> (u64, Vec<usize>) {
    if n == 0 {
        return (0, Vec::new());
    }

    assert!(is_tree(
        n,
        &edges
            .iter()
            .copied()
            .map(|(u, v, _)| (u, v))
            .collect::<Vec<_>>()
    ));

    for &(_, _, c) in edges {
        assert!(c >= 1);
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, c) in edges {
        graph[u].push((v, c));
        graph[v].push((u, c));
    }

    fn dfs(
        i: usize,
        p: usize,
        g: &[Vec<(usize, u64)>],
        dist: &mut Vec<u64>,
        parent: &mut Vec<usize>,
    ) {
        parent[i] = p;
        for &(j, c) in &g[i] {
            if j == p {
                continue;
            }
            dist[j] = dist[i] + c;
            dfs(j, i, g, dist, parent);
        }
    }

    let mut dist = vec![0; n];
    let mut parent = vec![0; n];
    dfs(0, 0, &graph, &mut dist, &mut parent);

    let max_dist = dist.iter().max().copied().unwrap();
    let s = (0..n).position(|i| dist[i] == max_dist).unwrap();
    dist = vec![0; n];
    parent = vec![s; n];
    dfs(s, s, &graph, &mut dist, &mut parent);

    let diameter = dist.iter().max().copied().unwrap();
    let t = (0..n).position(|i| dist[i] == diameter).unwrap();

    let mut cur = t;
    let mut path = Vec::new();
    path.push(cur);
    while cur != parent[cur] {
        cur = parent[cur];
        path.push(cur);
    }
    (diameter, path)
}

#[cfg(test)]
mod tests {
    use super::tree_diameter;

    #[test]
    fn test_small() {
        assert_eq!(tree_diameter(0, &[]), (0, vec![]));
        assert_eq!(tree_diameter(1, &[]), (0, vec![0]));

        let (diameter, path) = tree_diameter(2, &[(0, 1, 1)]);
        assert_eq!(diameter, 1);
        assert!(path == vec![0, 1] || path == vec![1, 0]);

        let (diameter, path) = tree_diameter(3, &[(0, 1, 100), (0, 2, 100)]);
        assert_eq!(diameter, 200);
        assert!(path == vec![1, 0, 2] || path == vec![2, 0, 1]);
    }
}
