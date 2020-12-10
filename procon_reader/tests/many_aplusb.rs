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
    let t: usize = rd.get();
    for _ in 0..t {
        let a: u64 = rd.get();
        let b: u64 = rd.get();
        puts!(a + b);
    }
}

#[test]
#[ignore]
fn many_aplusb() {
    system_test(solve, "https://judge.yosupo.jp/problem/many_aplusb");
}
