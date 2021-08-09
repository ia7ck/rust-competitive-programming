use std::fmt::Write;
use suffix_array::suffix_array;
use system_test_tool::system_test;

fn solve(input: &str, res: &mut String) {
    let s: Vec<char> = input.trim_end().chars().collect();
    let sa = suffix_array(&s);
    write!(res, "{}", sa[0]).unwrap();
    for sa in &sa[1..] {
        write!(res, " {}", sa).unwrap();
    }
    writeln!(res, "").unwrap();
}

#[test]
fn test_sample() {
    let tests = vec![
        ("abcbcba", "6 0 5 3 1 4 2"),
        ("mississippi", "10 7 4 1 0 9 8 6 3 5 2"),
    ];
    for (input, expected) in tests {
        let mut actual = String::new();
        solve(input, &mut actual);
        assert_eq!(actual.trim_end(), expected.trim_end());
    }
}

#[test]
#[ignore]
fn suffix_array_library_checker() {
    system_test(solve, "https://judge.yosupo.jp/problem/suffixarray");
}
