use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let x = i32::from_str(buf.trim_end()).unwrap();
    println!("{}", x * 2);
}
