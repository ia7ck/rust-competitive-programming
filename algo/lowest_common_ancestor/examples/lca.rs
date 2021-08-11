// oj: https://judge.yosupo.jp/problem/lca
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut g = vec![vec![]; n];
    for i in 1..n {
        input! {
            p: usize,
        }
        g[p].push(i);
        g[i].push(p);
    }
    let lca = LowestCommonAncestor::new(&g);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", lca.get(u, v));
    }
}
