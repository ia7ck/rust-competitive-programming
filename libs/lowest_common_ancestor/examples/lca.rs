// problem: https://judge.yosupo.jp/problem/lca
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut edges = Vec::new();
    for i in 1..n {
        input! {
            p: usize,
        }
        edges.push((i, p));
    }
    let lca = LowestCommonAncestor::new(n, 0, &edges);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", lca.get(u, v));
    }
}
