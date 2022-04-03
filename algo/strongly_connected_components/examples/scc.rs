// problem: https://judge.yosupo.jp/problem/scc
// judge_program_rs: ./judge_scc.rs

use join::Join;
use proconio::input;

use strongly_connected_components::strongly_connected_components;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    };

    let scc = strongly_connected_components(n, &edges);
    println!("{}", scc.len());
    for com in scc {
        print!("{} ", com.len());
        println!("{}", com.iter().join(" "));
    }
}
