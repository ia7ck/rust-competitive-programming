use procon_reader::ProconReader;
use std::fmt::Write;
use std::io::Cursor;
use system_test_tool::system_test;
use z_algorithm::z_algorithm;

fn solve(input: &str, res: &mut String) {
    let mut rd = ProconReader::new(Cursor::new(input));
    macro_rules! puts {
        ($v:expr) => {
            writeln!(res, "{}", $v).unwrap();
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
}

#[test]
#[ignore]
fn zalgorithm_library_checker() {
    system_test(solve, "https://judge.yosupo.jp/problem/zalgorithm")
}
