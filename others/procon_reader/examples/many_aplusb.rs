//oj:https://judge.yosupo.jp/problem/many_aplusb
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.scan();
    for _ in 0..t {
        let a: u64 = rd.scan();
        let b: u64 = rd.scan();
        println!("{}", a + b);
    }
}
