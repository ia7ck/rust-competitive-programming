// oj: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        input! {
            k: usize,
            children: [usize; k],
        }
        for c in children {
            g[i].push(c);
            g[c].push(i);
        }
    }
    let lca = LowestCommonAncestor::new(&g);
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
