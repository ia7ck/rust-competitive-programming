// problem: https://judge.yosupo.jp/problem/jump_on_tree
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        edges: [(usize, usize); n - 1],
        queries: [(usize, usize, usize); q],
    }

    let lca = LowestCommonAncestor::new(n, 0, &edges);
    for (s, t, i) in queries {
        let u = lca.get(s, t);
        let d_su = lca.depth(s) - lca.depth(u);
        let d_ut = lca.depth(t) - lca.depth(u);
        if i <= d_su {
            println!("{}", lca.kth_parent(s, i).unwrap());
        } else if i <= d_su + d_ut {
            println!("{}", lca.kth_parent(t, d_su + d_ut - i).unwrap());
        } else {
            println!("-1");
        }
    }
}
