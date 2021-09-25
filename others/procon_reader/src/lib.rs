use std::fmt;
use std::io;
use std::str;

/// 空白・改行区切りの入力を読みます。
pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: io::BufRead> ProconReader<R> {
    /// 標準入力から読み込みたいときの構築例です。ファイルからの読み込みは [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) の Examples を参考にしてください。
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use procon_reader::ProconReader;
    ///
    /// let stdin = io::stdin();
    /// let mut reader = ProconReader::new(io::BufReader::new(stdin));
    /// // ProconReader::new(stdin.lock()); のほうが気持ち速いです
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
    /// use procon_reader::ProconReader;
    ///
    /// let mut reader = ProconReader::from("123 abc\nx");
    /// let n = reader.scan::<usize>();
    /// assert_eq!(n, 123);
    /// let s: String = reader.scan();
    /// assert_eq!(s, "abc");
    /// let ch: char = reader.scan();
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

impl<'a> From<&'a str> for ProconReader<&'a [u8]> {
    fn from(s: &'a str) -> Self {
        Self::new(s.as_bytes())
    }
}

impl<'a> From<io::StdinLock<'a>> for ProconReader<io::BufReader<io::StdinLock<'a>>> {
    fn from(stdin: io::StdinLock<'a>) -> Self {
        Self::new(io::BufReader::new(stdin))
    }
}

#[allow(unused)]
macro_rules! scan_with {
    ($reader: expr, ($($t: ty),+)) => { // scan_with!(_r, (i32, i32))
        ($(scan_with!($reader, $t)),+)
    };
    ($reader: expr, $t: ty) => { // scan_with!(_r, i32)
        $reader.scan::<$t>()
    };
    ($reader: expr, ($($t: ty),+); $n: expr) => { // scan_with!(_r, (i32, i32); 100)
        std::iter::repeat_with(|| scan_with!($reader, ($($t),+))).take($n).collect::<Vec<_>>()
    };
    ($reader: expr, $t: ty; $n: expr) => { // scan_with!(_r, i32; 100)
        std::iter::repeat_with(|| scan_with!($reader, $t)).take($n).collect::<Vec<_>>()
    };
}

#[cfg(test)]
mod tests {
    use crate::ProconReader;

    #[test]
    fn test_single() {
        let mut _r = ProconReader::from("42");
        assert_eq!(scan_with!(_r, i32), 42);
        let mut _r = ProconReader::from("a");
        assert_eq!(scan_with!(_r, char), 'a');
        let mut _r = ProconReader::from("abc");
        assert_eq!(scan_with!(_r, String), "abc");
    }

    #[test]
    fn test_space_separated() {
        let mut reader = ProconReader::from("123 -123 a abc");
        assert_eq!(reader.scan::<usize>(), 123);
        assert_eq!(reader.scan::<i32>(), -123);
        assert_eq!(reader.scan::<char>(), 'a');
        assert_eq!(reader.scan::<String>(), "abc");
    }

    #[test]
    fn test_line_separated() {
        let mut reader = ProconReader::from("123\n-123\n\n\na\r\nabc");
        assert_eq!(reader.scan::<usize>(), 123);
        assert_eq!(reader.scan::<i32>(), -123);
        assert_eq!(reader.scan::<char>(), 'a');
        assert_eq!(reader.scan::<String>(), "abc");
    }

    #[test]
    fn test_scan_vec() {
        let mut _r = ProconReader::from("1 23 -456");
        assert_eq!(scan_with!(_r, i32; 3), vec![1, 23, -456]);
        let mut _r = ProconReader::from("abc\nde\nf");
        assert_eq!(scan_with!(_r, String; 3), vec!["abc", "de", "f"]);
    }

    #[test]
    fn test_scan_vec_of_tuple() {
        let mut _r = ProconReader::from("a 12\nb 3");
        assert_eq!(scan_with!(_r, (char, i32); 2), vec![('a', 12), ('b', 3)]);
    }

    #[test]
    #[should_panic(expected = "reached EOF")]
    fn too_many_scan() {
        let mut rd = ProconReader::from("123");
        rd.scan::<usize>(); // 123
        rd.scan::<usize>();
    }

    #[test]
    #[should_panic]
    fn cannot_parse_string_as_char() {
        let mut rd = ProconReader::from("abc");
        rd.scan::<char>(); // mismatch type
    }
}
