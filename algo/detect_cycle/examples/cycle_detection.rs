// oj: https://judge.yosupo.jp/problem/cycle_detection
// oj_judge_rs_program: ./judge_cycle_detection.rs

use detect_cycle::detect_cycle_directed;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    if let Some(cycle) = detect_cycle_directed(n, edges.iter().copied()) {
        println!("{}", cycle.len());
        for i in cycle {
            println!("{}", i);
        }
    } else {
        println!("-1");
    }
}