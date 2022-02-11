// problem: https://judge.yosupo.jp/problem/aplusb

use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (a, b) = scan!((u32, u32));
    println!("{}", a + b);
}
