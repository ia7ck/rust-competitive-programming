// oj: https://judge.yosupo.jp/problem/point_set_range_composite
use proconio::{fastout, input};
use segment_tree::SegmentTree;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(u64, u64); n],
    }
    let mo = 998244353;
    let mut seg = SegmentTree::new(n, (1, 0), |(a, b), (c, d)| {
        // c * (ax + b) + d
        // = ca * x + cb + d
        (c * a % mo, (c * b % mo + d) % mo)
    });
    for (i, &(a, b)) in ab.iter().enumerate() {
        seg.update(i, (a, b));
    }
    for _ in 0..q {
        input! {
            t: u8,
        };
        if t == 0 {
            input! {
                p: usize,
                c: u64,
                d: u64,
            }
            seg.update(p, (c, d));
        } else {
            input! {
                l: usize,
                r: usize,
                x: u64,
            }
            let (a, b) = seg.fold(l..r);
            let ans = a * x % mo + b;
            println!("{}", ans % mo);
        }
    }
}
