use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::ops::Add;

/// グラフの辺を表すトレイトです。
pub trait Edge<T> {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    /// [`from`] までの距離が `d` であり、この辺を辿って [`to`] へ行く最短距離を計算します。
    ///
    /// [`from`]: trait.Edge.html#tymethod.from
    /// [`to`]: trait.Edge.html#tymethod.to
    fn dist(&self, d: T) -> T;
}

/// 長さが定数の辺です。
#[derive(Copy, Clone)]
pub struct ConstEdge<T> {
    from: usize,
    to: usize,
    cost: T,
}

impl<T> ConstEdge<T> {
    pub fn new(from: usize, to: usize, cost: T) -> Self {
        Self { from, to, cost }
    }
}

impl<T> Edge<T> for ConstEdge<T>
where
    T: Copy + Add<Output = T>,
{
    fn from(&self) -> usize {
        self.from
    }
    fn to(&self) -> usize {
        self.to
    }
    fn dist(&self, d: T) -> T {
        d + self.cost
    }
}

/// `dijkstra` はあるひとつの頂点から全ての頂点への最短距離を計算します。
///
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
/// use dijkstra::{Edge, ConstEdge, dijkstra};
/// let edges = vec![
///     ConstEdge::new(0, 1, 1),
///     ConstEdge::new(0, 2, 1),
///     ConstEdge::new(1, 2, 1),
///     ConstEdge::new(2, 3, 1),
/// ];
/// //
/// //     0 -----> 1 -----> 2 -----> 3
/// //     |                 ^
/// //     |                 |
/// //     +-----------------+
/// //
/// let (d, prev) = dijkstra(4, edges.iter().copied(), 0);
/// assert_eq!(d[0], Some(0));
/// assert_eq!(d[1], Some(1));
/// assert_eq!(d[2], Some(1));
/// assert_eq!(d[3], Some(2));
/// assert_eq!(prev[0], None);
/// assert_eq!(prev[1], Some(0));
/// assert_eq!(prev[2], Some(0));
/// assert_eq!(prev[3], Some(2));
/// ```
pub fn dijkstra<I, E, T>(n: usize, edges: I, s: usize) -> (Vec<Option<T>>, Vec<Option<usize>>)
where
    I: Iterator<Item = E>,
    E: Edge<T> + Clone,
    T: Copy + Add<Output = T> + Default + Ord + Debug,
{
    let mut adj = vec![vec![]; n];
    for e in edges {
        adj[e.from()].push(e);
    }
    let mut dist = vec![None; n];
    let mut heap = BinaryHeap::new();
    let mut prev = vec![None; n];
    dist[s] = Some(T::default());
    heap.push((Reverse(T::default()), s));
    while let Some((Reverse(d), v)) = heap.pop() {
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
        for e in &adj[v] {
            let next_d = e.dist(d);
            let to = e.to();
            match dist[to] {
                Some(dt) if dt <= next_d => {
                    continue;
                }
                _ => {
                    dist[to] = Some(next_d);
                    prev[to] = Some(v);
                    heap.push((Reverse(next_d), to));
                }
            }
        }
    }
    (dist, prev)
}

#[cfg(test)]
mod tests {
    use crate::{dijkstra, ConstEdge};
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
                let dd = floyd_warshall(n, &edges);
                let edges = edges.into_iter().map(|(a, b, c)| ConstEdge::new(a, b, c));
                let (d, _) = dijkstra(n, edges, 0);
                for v in 0..n {
                    assert_eq!(d[v].unwrap_or(INF), dd[v]);
                }
            }
        }
    }
}
