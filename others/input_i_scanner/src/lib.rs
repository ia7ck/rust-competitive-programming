use std::fmt;
use std::io;
use std::str;

/// 空白・改行区切りの入力を読みます。
pub struct InputIScanner<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: io::BufRead> InputIScanner<R> {
    /// 標準入力から読み込みたいときの構築例です。ファイルからの読み込みは [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) の Examples を参考にしてください。
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use input_i_scanner::InputIScanner;
    ///
    /// let stdin = io::stdin();
    /// let mut scanner = InputIScanner::new(io::BufReader::new(stdin));
    /// // InputIScanner::new(io::BufReader::new(stdin.lock())); のほうが気持ち速いです
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }

    /// 空白・改行で区切られている値を取得します。適宜、型アノテーションをつけてください。
    ///
    /// # Examples
    /// ```
    /// use input_i_scanner::InputIScanner;
    ///
    /// let mut sc = InputIScanner::from("123 abc\nx");
    /// let n = sc.scan::<usize>();
    /// assert_eq!(n, 123);
    /// let s: String = sc.scan();
    /// assert_eq!(s, "abc");
    /// let ch: char = sc.scan();
    /// assert_eq!(ch, 'x');
    /// ```
    pub fn scan<T>(&mut self) -> T
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
}

impl<'a> From<&'a str> for InputIScanner<&'a [u8]> {
    fn from(s: &'a str) -> Self {
        Self::new(s.as_bytes())
    }
}

impl<'a> From<io::StdinLock<'a>> for InputIScanner<io::BufReader<io::StdinLock<'a>>> {
    fn from(stdin: io::StdinLock<'a>) -> Self {
        Self::new(io::BufReader::new(stdin))
    }
}

#[macro_export]
/// [`scan`] がまどろっこしいという方へおすすめのマクロです。
///
/// ```
/// use input_i_scanner::{InputIScanner, scan_with};
///
/// let mut _i_i = InputIScanner::from(r#"
/// 42
/// 123 abc
/// 9 8 7 6 5
/// x -1
/// y -2
/// z -3
/// "#);
/// assert_eq!(scan_with!(_i_i, i32), 42);
/// assert_eq!(scan_with!(_i_i, (i32, String)), (123, "abc".to_string()));
/// assert_eq!(scan_with!(_i_i, i32; 5), vec![9, 8, 7, 6, 5]);
/// assert_eq!(scan_with!(_i_i, (char, i32); 3), vec![('x', -1), ('y', -2), ('z', -3)]);
/// ```
///
/// [`scan`]: struct.InputIScanner.html#method.scan
macro_rules! scan_with {
    ($scanner: expr, ($($t: ty),+)) => { // scan_with!(_sc, (i32, i32))
        ($(scan_with!($scanner, $t)),+)
    };
    ($scanner: expr, $t: ty) => { // scan_with!(_sc, i32)
        $scanner.scan::<$t>()
    };
    ($scanner: expr, ($($t: ty),+); $n: expr) => { // scan_with!(_sc, (i32, i32); 100)
        std::iter::repeat_with(|| scan_with!($scanner, ($($t),+))).take($n).collect::<Vec<_>>()
    };
    ($scanner: expr, $t: ty; $n: expr) => { // scan_with!(_sc, i32; 100)
        std::iter::repeat_with(|| scan_with!($scanner, $t)).take($n).collect::<Vec<_>>()
    };
}

#[cfg(test)]
mod tests {
    use crate::InputIScanner;

    #[test]
    fn test_single() {
        let mut _i_i = InputIScanner::from("42");
        assert_eq!(scan_with!(_i_i, i32), 42);
        let mut _i_i = InputIScanner::from("a");
        assert_eq!(scan_with!(_i_i, char), 'a');
        let mut _i_i = InputIScanner::from("abc");
        assert_eq!(scan_with!(_i_i, String), "abc");
    }

    #[test]
    fn test_space_separated() {
        let mut sc = InputIScanner::from("123 -123 a abc");
        assert_eq!(sc.scan::<usize>(), 123);
        assert_eq!(sc.scan::<i32>(), -123);
        assert_eq!(sc.scan::<char>(), 'a');
        assert_eq!(sc.scan::<String>(), "abc");
    }

    #[test]
    fn test_line_separated() {
        let mut sc = InputIScanner::from("123\n-123\n\n\na\r\nabc");
        assert_eq!(sc.scan::<usize>(), 123);
        assert_eq!(sc.scan::<i32>(), -123);
        assert_eq!(sc.scan::<char>(), 'a');
        assert_eq!(sc.scan::<String>(), "abc");
    }

    #[test]
    fn test_scan_vec() {
        let mut _i_i = InputIScanner::from("1 23 -456");
        assert_eq!(scan_with!(_i_i, i32; 3), vec![1, 23, -456]);
        let mut _i_i = InputIScanner::from("abc\nde\nf");
        assert_eq!(scan_with!(_i_i, String; 3), vec!["abc", "de", "f"]);
    }

    #[test]
    fn test_scan_vec_of_tuple() {
        let mut _i_i = InputIScanner::from("a 12\nb 3");
        assert_eq!(scan_with!(_i_i, (char, i32); 2), vec![('a', 12), ('b', 3)]);
    }

    #[test]
    #[should_panic(expected = "reached EOF")]
    fn too_many_scan() {
        let mut sc = InputIScanner::from("123");
        assert_eq!(sc.scan::<usize>(), 123);
        sc.scan::<usize>();
    }

    #[test]
    #[should_panic]
    fn cannot_parse_string_as_char() {
        let mut sc = InputIScanner::from("abc");
        sc.scan::<char>(); // mismatch type
    }
}
