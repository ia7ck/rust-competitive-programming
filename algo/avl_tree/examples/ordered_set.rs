// problem: https://judge.yosupo.jp/problem/ordered_set
use avl_tree::AvlTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(u8, u64); q],
    };

    let mut avl = AvlTree::default();
    for x in a {
        avl.insert(x);
    }

    for (op, x) in queries {
        match op {
            0 => {
                avl.insert(x);
            }
            1 => {
                avl.remove(&x);
            }
            2 => {
                if let Some(ans) = avl.nth((x - 1) as usize) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            3 => {
                let ans = match avl.position(&x) {
                    Ok(p) => p + 1,
                    Err(p) => p,
                };
                println!("{}", ans);
            }
            4 => {
                if let Some(ans) = avl.le(&x) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            5 => {
                if let Some(ans) = avl.ge(&x) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
