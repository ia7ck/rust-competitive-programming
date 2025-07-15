// problem: https://judge.yosupo.jp/problem/suffixarray
use proconio::marker::Chars;
use proconio::{fastout, input};
use suffix_array::suffix_array;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let sa = suffix_array(&s);
    println!(
        "{}",
        sa.iter()
            .map(|sa| sa.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
