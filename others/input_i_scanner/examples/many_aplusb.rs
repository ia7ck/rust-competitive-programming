//oj:https://judge.yosupo.jp/problem/many_aplusb
use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let t = scan_with!(_i_i, usize);
    for _ in 0..t {
        let (a, b) = scan_with!(_i_i, (u64, u64));
        println!("{}", a + b);
    }
}
