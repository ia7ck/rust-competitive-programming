//oj:https://judge.yosupo.jp/problem/many_aplusb
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let a: u64 = rd.get();
        let b: u64 = rd.get();
        println!("{}", a + b);
    }
}
