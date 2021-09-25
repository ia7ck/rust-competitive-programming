//oj: https://judge.yosupo.jp/problem/shortest_path
use dijkstra::{dijkstra, ConstEdge};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
    }
    let mut edges = Vec::new();
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: u64,
        }
        edges.push(ConstEdge::new(a, b, c));
    }
    let (d, prev) = dijkstra(n, edges.iter().copied(), s);
    if d[t].is_none() {
        println!("{}", -1);
        return;
    }
    let mut ans = vec![t];
    let mut v = t;
    while let Some(u) = prev[v] {
        ans.push(u);
        v = u;
    }
    println!("{} {}", d[t].unwrap(), ans.len() - 1);
    ans.reverse();
    for w in ans.windows(2) {
        println!("{} {}", w[0], w[1]);
    }
}
