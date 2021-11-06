// problem: https://judge.yosupo.jp/problem/zalgorithm

use join::Join;
use proconio::marker::Chars;
use proconio::{fastout, input};
use z_algorithm::z_algorithm;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let a = z_algorithm(&s);
    println!("{}", a.iter().join(" "));
}
