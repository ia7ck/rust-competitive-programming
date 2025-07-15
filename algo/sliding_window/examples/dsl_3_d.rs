// problem: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_D

use proconio::input;

use sliding_window::sliding_window_minimum;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [u32; n],
    }
    let minimums = sliding_window_minimum(&a, l);
    println!(
        "{}",
        minimums
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
