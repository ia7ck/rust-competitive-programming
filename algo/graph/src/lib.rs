use std::mem;

pub fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
    for &(a, b) in edges {
        assert!(a < n);
        assert!(b < n);
        assert_ne!(a, b);
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

pub fn tree_drop_parent(
    n: usize,
    root: usize,
    edges: &[(usize, usize)],
) -> (Vec<Vec<usize>>, Vec<usize>) {
    debug_assert!(is_tree(n, edges));

    fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, parent: &mut Vec<usize>) {
        parent[i] = p;
        for &j in &g[i] {
            if j == p {
                continue;
            }
            dfs(j, i, g, parent);
        }
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut parent = vec![usize::MAX; n];
    dfs(root, root, &g, &mut parent);

    for i in 0..n {
        g[i] = mem::take(&mut g[i])
            .into_iter()
            .filter(|&j| j != parent[i])
            .collect();
    }

    (g, parent)
}

#[cfg(test)]
mod tests {
    use crate::{is_tree, tree_drop_parent};

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

    #[test]
    fn test_drop_parent() {
        assert_eq!(
            // 0 (root) -- 1 -- 2 -- 3
            tree_drop_parent(4, 0, &[(0, 1), (1, 2), (2, 3)]),
            (
                vec![vec![1], vec![2], vec![3], vec![]], // g
                vec![0, 0, 1, 2],                        // parent
            )
        );
    }
}
