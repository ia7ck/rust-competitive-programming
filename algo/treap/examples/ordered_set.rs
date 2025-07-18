// problem: https://judge.yosupo.jp/problem/ordered_set
use proconio::{fastout, input};
use treap::Treap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(u8, u64); q],
    };

    let mut treap = Treap::default();
    for x in a {
        treap.insert(x);
    }

    for (op, x) in queries {
        match op {
            0 => {
                treap.insert(x);
            }
            1 => {
                treap.remove(&(x));
            }
            2 => {
                if let Some(ans) = treap.nth((x - 1) as usize) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            3 => {
                let ans = match treap.position(&x) {
                    Ok(p) => p + 1,
                    Err(p) => p,
                };
                println!("{}", ans);
            }
            4 => {
                if let Some(ans) = treap.le(&x) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            5 => {
                if let Some(ans) = treap.ge(&x) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
