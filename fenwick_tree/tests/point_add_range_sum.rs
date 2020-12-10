use fenwick_tree::FenwickTree;
use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::system_test;

fn solve(input: &str, res: &mut String) {
    let mut rd = ProconReader::new(Cursor::new(input));
    macro_rules! puts {
        ($v:expr) => {
            writeln!(res, "{}", $v).unwrap();
        };
    }
    let n: usize = rd.get();
    let q: usize = rd.get();
    let a: Vec<i64> = rd.get_vec(n);
    let mut ft = FenwickTree::new(n, 0);
    for i in 0..n {
        ft.add(i, a[i]);
    }
    for _ in 0..q {
        let t: usize = rd.get();
        match t {
            0 => {
                let p: usize = rd.get();
                let x: i64 = rd.get();
                ft.add(p, x);
            }
            1 => {
                let l: usize = rd.get();
                let r: usize = rd.get();
                let sum = ft.sum(l..r);
                puts!(sum);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
#[ignore]
fn point_add_range_sum() {
    system_test(solve, "https://judge.yosupo.jp/problem/point_add_range_sum");
}
