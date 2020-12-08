use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::{system_test, Solution};
use z_algorithm::z_algorithm;

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
        let s: String = rd.get();
        let s: Vec<char> = s.chars().collect();

        let a = z_algorithm(&s);
        let a = a
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        puts!(a);
        res
    }

    fn problem_url(&self) -> &'static str {
        "https://judge.yosupo.jp/problem/zalgorithm"
    }
}

#[test]
#[ignore]
fn zalgorithm_library_checker() {
    system_test(&S {});
}
