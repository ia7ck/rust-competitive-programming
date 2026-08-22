// problem: https://judge.yosupo.jp/problem/cycle_detection_undirected
// judge_program_rs: ./judge_cycle_detection_undirected.rs

use detect_cycle::detect_cycle_undirected;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    let Some(cycle) = detect_cycle_undirected(n, &edges) else {
        println!("-1");
        return;
    };

    let (u, v) = edges[cycle[0]];
    let mut current = if cycle.len() == 1 {
        u
    } else {
        let (next_u, next_v) = edges[cycle[1]];
        if v == next_u || v == next_v { u } else { v }
    };
    let mut vertices = Vec::with_capacity(cycle.len());
    for &edge_id in &cycle {
        vertices.push(current);
        let (u, v) = edges[edge_id];
        current = if current == u { v } else { u };
    }

    println!("{}", cycle.len());
    println!(
        "{}",
        vertices
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "{}",
        cycle
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
