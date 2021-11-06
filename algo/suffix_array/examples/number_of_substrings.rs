// problem: https://judge.yosupo.jp/problem/number_of_substrings
use proconio::marker::Chars;
use proconio::{fastout, input};
use suffix_array::{lcp_array, suffix_array};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let sa = suffix_array(&s);
    let lcp = lcp_array(&s, &sa);
    let ans = s.len() * (s.len() + 1) / 2 - lcp.iter().sum::<usize>();
    println!("{}", ans);
}
