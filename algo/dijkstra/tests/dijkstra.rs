use dijkstra::{dijkstra, Edge};
use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::system_test;

fn solve(input: &str, res: &mut String) {
    let mut rd = ProconReader::new(Cursor::new(input));
    macro_rules! puts {
        ($fmt:expr, $($v:expr),*) => {
            writeln!(res, $fmt, $($v),*).unwrap();
        };
    }
    let n: usize = rd.get();
    let m: usize = rd.get();
    let s: usize = rd.get();
    let t: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let c: u64 = rd.get();
        g[a].push(Edge { to: b, cost: c });
    }
    let (d, prev) = dijkstra(&g, s);
    if d[t].is_none() {
        puts!("{}", -1);
        return;
    }
    let mut ans = vec![t];
    let mut v = t;
    while let Some(u) = prev[v] {
        ans.push(u);
        v = u;
    }
    puts!("{} {}", d[t].unwrap(), ans.len() - 1);
    ans.reverse();
    for w in ans.windows(2) {
        puts!("{} {}", w[0], w[1]);
    }
}

#[test]
#[ignore]
fn shortest_path_library_checker() {
    system_test(solve, "https://judge.yosupo.jp/problem/shortest_path");
}
