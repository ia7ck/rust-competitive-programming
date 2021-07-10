use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    /// 行き先の頂点です。
    pub to: usize,
    /// 移動にかかるコストです。
    pub cost: u64,
}

impl Edge {
    pub fn new(to: usize, cost: u64) -> Self {
        Self { to, cost }
    }
}

/// `dijkstra` はあるひとつの頂点から全ての頂点への最短距離を計算します。
///
/// 隣接リスト `g` とスタートの頂点 `s` を渡します。
/// 返り値 `(d, prev)` はそれぞれ以下です。
///
/// - `d[t]`: `s` から `t` までの最短距離
/// - `prev[t]`: `s` を根とする最短経路木における `t` の親頂点
///
/// `prev` をゴールの頂点からたどることで、最短経路を復元できます。
///
/// `s` から `t` への経路が存在しない場合 `d[t]`、`prev[t]` は `None` です。
///
/// # Examples
/// ```
/// use dijkstra::{Edge, dijkstra};
/// let mut g = vec![vec![]; 4];
/// //
/// //     0 -----> 1 -----> 2 -----> 3
/// //     |                 ^
/// //     |                 |
/// //     +-----------------+
/// //
/// g[0].push(Edge::new(1, 1));
/// g[0].push(Edge::new(2, 1));
/// g[1].push(Edge::new(2, 1));
/// g[2].push(Edge::new(3, 1));
/// let (d, prev) = dijkstra(&g, 0);
/// assert_eq!(d[0], Some(0));
/// assert_eq!(d[1], Some(1));
/// assert_eq!(d[2], Some(1));
/// assert_eq!(d[3], Some(2));
/// assert_eq!(prev[0], None);
/// assert_eq!(prev[1], Some(0));
/// assert_eq!(prev[2], Some(0));
/// assert_eq!(prev[3], Some(2));
/// ```
#[allow(clippy::many_single_char_names)]
pub fn dijkstra(g: &[Vec<Edge>], s: usize) -> (Vec<Option<u64>>, Vec<Option<usize>>) {
    let n = g.len();
    let mut dist = vec![None; n];
    let mut q = BinaryHeap::new();
    let mut prev = vec![None; n];
    dist[s] = Some(0);
    q.push((Reverse(0), s));
    while let Some((Reverse(d), v)) = q.pop() {
        match dist[v] {
            Some(dv) => {
                if dv < d {
                    continue;
                } else {
                    assert_eq!(dv, d);
                }
            }
            None => unreachable!(),
        }
        for e in &g[v] {
            let next_d = d + e.cost;
            match dist[e.to] {
                Some(dt) if dt <= next_d => {
                    continue;
                }
                _ => {
                    dist[e.to] = Some(next_d);
                    prev[e.to] = Some(v);
                    q.push((Reverse(next_d), e.to));
                }
            }
        }
    }
    (dist, prev)
}

#[cfg(test)]
mod tests {
    use crate::{dijkstra, Edge};
    use rand::distributions::Uniform;
    use rand::prelude::*;

    #[allow(clippy::many_single_char_names)]
    fn generate(n: usize, m: usize) -> Vec<(usize, usize, u64)> {
        let nodes = Uniform::from(0..n);
        let costs = Uniform::from(0..=1_000_000_000);
        let mut rng = thread_rng();
        (0..m)
            .map(|_| {
                let a = nodes.sample(&mut rng);
                let b = nodes.sample(&mut rng);
                let c = costs.sample(&mut rng);
                (a, b, c)
            })
            .take(m)
            .collect()
    }

    const INF: u64 = std::u64::MAX;

    fn floyd_warshall(n: usize, edges: &Vec<(usize, usize, u64)>) -> Vec<u64> {
        let mut d = vec![vec![INF; n]; n];
        for i in 0..n {
            d[i][i] = 0;
        }
        for &(a, b, c) in edges {
            d[a][b] = d[a][b].min(c);
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    d[i][j] = d[i][j].min(d[i][k].saturating_add(d[k][j]));
                }
            }
        }
        d[0].clone()
    }

    #[test]
    fn random_test() {
        for n in 1..=10 {
            for m in 0..=n * n {
                let edges = generate(n, m);
                let mut g = vec![vec![]; n];
                for &(a, b, c) in &edges {
                    g[a].push(Edge::new(b, c));
                }
                let dd = floyd_warshall(n, &edges);
                let (d, _) = dijkstra(&g, 0);
                let d = d.iter().map(|&d| d.unwrap_or(INF)).collect::<Vec<_>>();
                assert_eq!(dd, d);
            }
        }
    }
}
