use std::fmt;
use std::io;
use std::str;

/// 空白・改行区切りの入力を読みます。
pub struct Scanner<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: io::BufRead> Scanner<R> {
    /// 標準入力から読み込みたいときの構築例です。ファイルからの読み込みは [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) の Examples を参考にしてください。
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use scanner::Scanner;
    ///
    /// let stdin = io::stdin();
    /// let mut scanner = Scanner::new(io::BufReader::new(stdin));
    /// // Scanner::new(io::BufReader::new(stdin.lock())); のほうが気持ち速いです
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
    /// use scanner::Scanner;
    ///
    /// let mut sc = Scanner::from("123 abc\nx");
    /// let n = sc.scan::<usize>();
    /// assert_eq!(n, 123);
    /// let s: String = sc.scan();
    /// assert_eq!(s, "abc");
    /// let ch: char = sc.scan();
    /// assert_eq!(ch, 'x');
    /// ```
    ///
    /// タプルやベクタを scan する例です。
    ///
    /// ```
    /// use scanner::Scanner;
    ///
    /// let mut sc = Scanner::from(r#"
    /// 42
    /// 123 abc
    /// 9 8 7 6 5
    /// x -1
    /// y -2
    /// z -3
    /// "#);
    ///
    /// macro_rules! scan {
    ///     (($($t: ty),+)) => { // scan!((i32, i32))
    ///         ($(scan!($t)),+)
    ///     };
    ///     ($t: ty) => { // scan!(i32)
    ///         sc.scan::<$t>() as $t
    ///     };
    ///     (($($t: ty),+); $n: expr) => { // scan((i32, i32); 100)
    ///         std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
    ///     };
    ///     ($t: ty; $n: expr) => { // scan!(i32; 100)
    ///         std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
    ///     };
    /// }
    ///
    /// assert_eq!(scan!(i32), 42);
    /// assert_eq!(scan!((i32, String)), (123, "abc".to_string()));
    /// assert_eq!(scan!(i32; 5), vec![9, 8, 7, 6, 5]);
    /// assert_eq!(scan!((char, i32); 3), vec![('x', -1), ('y', -2), ('z', -3)]);
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

impl<'a> From<&'a str> for Scanner<&'a [u8]> {
    fn from(s: &'a str) -> Self {
        Self::new(s.as_bytes())
    }
}

impl<'a> From<io::StdinLock<'a>> for Scanner<io::BufReader<io::StdinLock<'a>>> {
    fn from(stdin: io::StdinLock<'a>) -> Self {
        Self::new(io::BufReader::new(stdin))
    }
}

#[cfg(test)]
mod tests {
    use crate::Scanner;

    #[test]
    fn test_single() {
        let mut sc = Scanner::from("42");
        assert_eq!(sc.scan::<i32>(), 42);
        let mut sc = Scanner::from("a");
        assert_eq!(sc.scan::<char>(), 'a');
        let mut sc = Scanner::from("abc");
        assert_eq!(sc.scan::<String>(), "abc");
    }

    #[test]
    fn test_space_separated() {
        let mut sc = Scanner::from("123 -123 a abc");
        assert_eq!(sc.scan::<usize>(), 123);
        assert_eq!(sc.scan::<i32>(), -123);
        assert_eq!(sc.scan::<char>(), 'a');
        assert_eq!(sc.scan::<String>(), "abc");
    }

    #[test]
    fn test_line_separated() {
        let mut sc = Scanner::from("123\n-123\n\n\na\r\nabc");
        assert_eq!(sc.scan::<usize>(), 123);
        assert_eq!(sc.scan::<i32>(), -123);
        assert_eq!(sc.scan::<char>(), 'a');
        assert_eq!(sc.scan::<String>(), "abc");
    }

    #[test]
    #[should_panic(expected = "reached EOF")]
    fn too_many_scan() {
        let mut sc = Scanner::from("123");
        assert_eq!(sc.scan::<usize>(), 123);
        sc.scan::<usize>();
    }

    #[test]
    #[should_panic]
    fn cannot_parse_string_as_char() {
        let mut sc = Scanner::from("abc");
        sc.scan::<char>(); // mismatch type
    }
}
