// problem: https://judge.yosupo.jp/problem/tree_path_composite_sum
use proconio::input;
use re_rooting_dp::re_rooting_dp;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        edges: [(usize, usize, u64, u64); n - 1],
    };

    const M: u64 = 998244353;

    let edges = edges
        .into_iter()
        .map(|(u, v, b, c)| (u, v, E { b, c }))
        .collect::<Vec<_>>();

    let ans = re_rooting_dp(
        n,
        &edges,
        |i| V { val: a[i], size: 1 },
        |p, ch, e| {
            // Σ_j e.b * P(ch, j) + e.c
            // = e.c * ch.size + e.b * Σ_j P(ch, j)

            V {
                val: (p.val + e.c * ch.size % M + e.b * ch.val % M) % M,
                size: p.size + ch.size,
            }
        },
    );

    let ans = ans
        .iter()
        .map(|v| v.val.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}

#[derive(Debug)]
struct E {
    b: u64,
    c: u64,
}

#[derive(Debug, Clone)]
struct V {
    // i: usize,
    val: u64,  // Σ P(i, j)
    size: u64, // 部分木のサイズ
}
