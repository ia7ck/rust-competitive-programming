use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::{system_test, Solution};

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
        let t: usize = rd.get();
        for _ in 0..t {
            let a: u64 = rd.get();
            let b: u64 = rd.get();
            puts!(a + b);
        }
        res
    }

    fn problem_url(&self) -> &'static str {
        "https://judge.yosupo.jp/problem/many_aplusb"
    }
}

#[test]
#[ignore]
fn many_aplusb() {
    system_test(&S {});
}
