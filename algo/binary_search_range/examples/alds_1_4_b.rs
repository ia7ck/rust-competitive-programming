// problem:https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B

use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u32; n],
        q: usize,
        t: [u32; q],
    }

    let mut ans = 0;
    for t in t {
        let eq = s.range(t..(t + 1));
        if eq.len() >= 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
