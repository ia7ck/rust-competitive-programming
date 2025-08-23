// problem: https://judge.yosupo.jp/problem/zalgorithm

use proconio::marker::Chars;
use proconio::{fastout, input};
use z_algorithm::z_algorithm;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let a = z_algorithm(&s);
    println!(
        "{}",
        a.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
