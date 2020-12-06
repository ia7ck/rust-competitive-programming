use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::{system_test, Solution};
use union_find::UnionFind;

struct S;
impl Solution for S {
    fn solve(&self, input: &str) -> String {
        let mut rd = ProconReader::new(Cursor::new(input));
        let mut res = String::new();
        macro_rules! puts {
            ($v:expr) => {
                writeln!(&mut res, "{}", $v).unwrap();
            };
        }
        let n: usize = rd.get();
        let mut uf = UnionFind::new(n);
        let q: usize = rd.get();
        for _ in 0..q {
            let t: usize = rd.get();
            match t {
                0 => {
                    let u: usize = rd.get();
                    let v: usize = rd.get();
                    uf.unite(u, v);
                }
                1 => {
                    let u: usize = rd.get();
                    let v: usize = rd.get();
                    puts!(uf.same(u, v) as usize);
                }
                _ => unreachable!(),
            }
        }
        res
    }

    fn problem_url(&self) -> &'static str {
        "https://judge.yosupo.jp/problem/unionfind"
    }
}

#[test]
#[ignore]
fn union_find_library_checker() {
    system_test(&S {});
}
