// problem: https://judge.yosupo.jp/problem/predecessor_problem
use proconio::{fastout, input, marker::Chars};
use treap::Treap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        t: Chars,
        queries: [(u8, usize); q],
    };

    let mut treap = Treap::default();
    for i in 0..n {
        if t[i] == '1' {
            treap.insert(i);
        }
    }

    for (op, k) in queries {
        match op {
            0 => {
                treap.insert(k);
            }
            1 => {
                treap.remove(&k);
            }
            2 => {
                if treap.contains(&k) {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            3 => match treap.ge(&k) {
                None => {
                    println!("-1");
                }
                Some(x) => {
                    println!("{}", x);
                }
            },
            4 => match treap.le(&k) {
                None => {
                    println!("-1");
                }
                Some(x) => {
                    println!("{}", x);
                }
            },
            _ => unreachable!(),
        }
    }
}
