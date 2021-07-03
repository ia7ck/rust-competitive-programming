use lowest_common_ancestor::LowestCommonAncestor;
use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::system_test;

fn solve_library_checker(input: &str, res: &mut String) {
    let mut rd = ProconReader::new(Cursor::new(input));

    let n: usize = rd.get();
    let q: usize = rd.get();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        let p: usize = rd.get();
        g[p].push(i);
        g[i].push(p);
    }
    let lca = LowestCommonAncestor::new(&g);
    for _ in 0..q {
        let u: usize = rd.get();
        let v: usize = rd.get();
        writeln!(res, "{}", lca.get(u, v)).unwrap();
    }
}

fn solve_aoj(input: &str, res: &mut String) {
    let mut rd = ProconReader::new(Cursor::new(input));

    let n: usize = rd.get();
    let mut g = vec![vec![]; n];
    for i in 0..n {
        let k: usize = rd.get();
        let children: Vec<usize> = rd.get_vec(k);
        for c in children {
            g[i].push(c);
            g[c].push(i);
        }
    }
    let lca = LowestCommonAncestor::new(&g);
    let q: usize = rd.get();
    for _ in 0..q {
        let u: usize = rd.get();
        let v: usize = rd.get();
        writeln!(res, "{}", lca.get(u, v)).unwrap();
    }
}

#[test]
#[ignore]
fn lca_library_checker() {
    system_test(solve_library_checker, "https://judge.yosupo.jp/problem/lca")
}

#[test]
#[ignore]
fn lca_aoj() {
    system_test(
        solve_aoj,
        "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C",
    )
}
