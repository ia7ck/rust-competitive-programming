// problem: https://judge.yosupo.jp/problem/aplusb
// judge_program_rs: ./judge.rs
use proconio::input;
use special_judge_example::add;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let ans = add(a, b);
    println!("{}", ans);
}
