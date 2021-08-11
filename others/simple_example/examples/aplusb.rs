// oj: https://judge.yosupo.jp/problem/aplusb
use proconio::input;
use simple_example::add;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let ans = add(a, b);
    println!("{}", ans);
}
