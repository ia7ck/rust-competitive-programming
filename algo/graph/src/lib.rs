pub fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
    for &(a, b) in edges {
        assert!(a < n);
        assert!(b < n);
    }

    if n == 0 {
        return true;
    }

    edges.len() == n - 1 && connectivity(n, edges)
}

pub fn connectivity(n: usize, edges: &[(usize, usize)]) -> bool {
    fn dfs(i: usize, g: &[Vec<usize>], visited: &mut Vec<bool>) {
        for &j in &g[i] {
            if visited[j] {
                continue;
            }
            visited[j] = true;
            dfs(j, g, visited);
        }
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, &g, &mut visited);
    visited.iter().filter(|&&f| f).count() == n
}

#[cfg(test)]
mod tests {
    use crate::is_tree;

    #[test]
    fn test_is_tree_small() {
        assert_eq!(is_tree(0, &[]), true);
        assert_eq!(is_tree(1, &[]), true);
        assert_eq!(is_tree(2, &[(0, 1)]), true);
        assert_eq!(is_tree(3, &[(0, 1), (1, 2)]), true);
        assert_eq!(is_tree(4, &[(0, 1), (0, 2), (0, 3)]), true);
        assert_eq!(is_tree(4, &[(0, 1), (1, 2), (0, 3)]), true);
        assert_eq!(is_tree(4, &[(0, 1), (2, 3)]), false);
        assert_eq!(is_tree(4, &[(0, 1), (1, 2), (2, 0)]), false);
    }
}
