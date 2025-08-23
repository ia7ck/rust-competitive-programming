/// 強連結成分分解です。[参考](https://manabitimes.jp/math/1250)
///
/// 返り値を `components` とすると `components` の各要素は強連結成分をなす頂点のベクタです。
pub fn strongly_connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    for &(u, v) in edges {
        graph[u].push(v);
    }

    let mut seen = vec![false; n];
    let mut order = Vec::new();
    let mut order_pushed = vec![false; n];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        let mut stack = Vec::new();
        stack.push(v);
        while let Some(x) = stack.pop() {
            seen[x] = true;
            stack.push(x);
            let mut pushed = false;
            for &y in &graph[x] {
                if !seen[y] {
                    stack.push(y);
                    pushed = true;
                }
            }
            if !pushed {
                debug_assert_eq!(stack.last(), Some(&x));
                stack.pop();
                if !order_pushed[x] {
                    order_pushed[x] = true;
                    order.push(x);
                }
            }
        }
    }
    assert_eq!(order.len(), n);

    let mut rev_graph = vec![vec![]; n];
    #[allow(clippy::needless_range_loop)]
    for u in 0..n {
        for &v in &graph[u] {
            rev_graph[v].push(u);
        }
    }

    let mut seen = vec![false; n];
    let mut component_id = vec![0; n];
    let mut id = 0;
    for &v in order.iter().rev() {
        if seen[v] {
            continue;
        }
        let mut stack = Vec::new();
        stack.push(v);
        while let Some(x) = stack.pop() {
            seen[x] = true;
            component_id[x] = id;
            for &y in &rev_graph[x] {
                if !seen[y] {
                    stack.push(y);
                }
            }
        }
        id += 1;
    }

    let mut components = vec![vec![]; id];
    for v in 0..n {
        components[component_id[v]].push(v);
    }
    components
}

#[cfg(test)]
mod tests {
    use crate::strongly_connected_components;

    #[test]
    fn test_single_node() {
        let scc = strongly_connected_components(1, &[]);
        assert_eq!(scc, vec![vec![0]]);
    }

    #[test]
    fn test_small() {
        // 0 -> 1
        assert_eq!(
            strongly_connected_components(2, &[(0, 1)]),
            vec![vec![0], vec![1]]
        );

        // 0 -> 1
        // 0 -> 1
        assert_eq!(
            strongly_connected_components(2, &[(0, 1), (0, 1)]),
            vec![vec![0], vec![1]]
        );

        // 0 <-> 1
        let mut scc = strongly_connected_components(2, &[(0, 1), (1, 0)]);
        for com in &mut scc {
            com.sort();
        }
        assert_eq!(scc, vec![vec![0, 1]]);
    }
}
