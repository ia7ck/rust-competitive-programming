// oj: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut edges = Vec::new();
    for i in 0..n {
        input! {
            k: usize,
            children: [usize; k],
        }
        for c in children {
            edges.push((i, c));
        }
    }
    let lca = LowestCommonAncestor::new(n, edges.iter().copied());
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", lca.get(u, v));
    }
}
