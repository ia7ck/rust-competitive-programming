// problem:https://judge.yosupo.jp/problem/point_add_range_sum
use fenwick_tree::FenwickTree;
use join::Join;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut ft = FenwickTree::new(n, 0);
    for i in 0..n {
        ft.add(i, a[i]);
    }
    let mut ans = Vec::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            0 => {
                input! {
                    p: usize,
                    x: i64,
                }
                ft.add(p, x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let sum = ft.sum(l..r);
                ans.push(sum);
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans.iter().join("\n"));
}
