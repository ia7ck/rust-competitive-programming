use std::io::BufRead;
use std::str::FromStr;

/// 競技プログラミングで、入力値を読むパートをラクにします。
pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: BufRead> ProconReader<R> {
    /// 標準入力から読み込みたいときの例です。
    /// ファイルからの読み込みは [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html) の Examples を参考にしてください。
    ///
    /// # Examples
    /// ```
    /// use std::io::{stdin, BufReader};
    /// use procon_reader::ProconReader;
    ///
    /// let stdin = stdin();
    /// let mut rd = ProconReader::new(BufReader::new(stdin));
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
    /// use std::io::Cursor;
    /// use procon_reader::ProconReader;
    ///
    /// let mut rd = ProconReader::new(Cursor::new("123 abc\nx"));
    /// let n = rd.get::<usize>();
    /// assert_eq!(n, 123);
    /// let s: String = rd.get();
    /// assert_eq!(s, "abc");
    /// let ch: char = rd.get();
    /// assert_eq!(ch, 'x');
    /// ```
    pub fn get<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
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
    /// 空白・改行区切りの値を `n` 個読みます。
    ///
    /// # Examples
    /// ```
    /// use std::io::Cursor;
    /// use procon_reader::ProconReader;
    ///
    /// let mut rd = ProconReader::new(Cursor::new("123 45 -6"));
    /// let a: Vec<i32> = rd.get_vec(3);
    /// assert_eq!(a, vec![123, 45, -6]);
    /// ```
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }

    /// 1 行の文字列を `char` のベクタとして読みます。
    ///
    /// # Examples
    /// ```
    /// use std::io::Cursor;
    /// use procon_reader::ProconReader;
    ///
    /// let mut rd = ProconReader::new(Cursor::new("abcd"));
    /// let a: Vec<char> = rd.get_chars();
    /// assert_eq!(a, vec!['a', 'b', 'c', 'd']);
    /// ```
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::ProconReader;
    use std::fmt::Debug;
    use std::io::Cursor;
    use std::str::FromStr;

    fn get<T>(input: &str) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        ProconReader::new(Cursor::new(input)).get()
    }

    #[test]
    fn test_single() {
        assert_eq!(get::<usize>("123"), 123);
        assert_eq!(get::<i32>("-123"), -123);
        assert_eq!(get::<char>("a"), 'a');
        assert_eq!(get::<String>("abc"), "abc");
    }

    #[test]
    fn test_space_separated() {
        let input = "123 -123 a abc";
        let mut rd = ProconReader::new(Cursor::new(input));
        assert_eq!(rd.get::<usize>(), 123);
        assert_eq!(rd.get::<i32>(), -123);
        assert_eq!(rd.get::<char>(), 'a');
        assert_eq!(rd.get::<String>(), "abc");
    }

    #[test]
    fn test_line_separated() {
        let input = "123\n-123\n\n\na\r\nabc";
        let mut rd = ProconReader::new(Cursor::new(input));
        assert_eq!(rd.get::<usize>(), 123);
        assert_eq!(rd.get::<i32>(), -123);
        assert_eq!(rd.get::<char>(), 'a');
        assert_eq!(rd.get::<String>(), "abc");
    }

    fn get_vec<T>(input: &str, n: usize) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        ProconReader::new(Cursor::new(input)).get_vec(n)
    }

    #[test]
    fn test_get_vec() {
        assert_eq!(get_vec::<i32>("1 23 -456", 3), vec![1, 23, -456]);
        assert_eq!(get_vec::<String>("abc\nde\nf", 3), vec!["abc", "de", "f"]);
    }

    fn get_chars(input: &str) -> Vec<char> {
        ProconReader::new(Cursor::new(input)).get_chars()
    }

    #[test]
    fn test_get_chars() {
        assert_eq!(get_chars("abcd"), vec!['a', 'b', 'c', 'd']);
        assert_eq!(get_chars("1234"), vec!['1', '2', '3', '4']);
    }

    #[test]
    #[should_panic(expected = "reached EOF")]
    fn too_many_get() {
        let input = "123";
        let mut rd = ProconReader::new(Cursor::new(input));
        rd.get::<usize>(); // 123
        rd.get::<usize>();
    }

    #[test]
    #[should_panic]
    fn cannot_parse_string_as_char() {
        let input = "abc";
        let mut rd = ProconReader::new(Cursor::new(input));
        rd.get::<char>(); // mismatch type
    }
}
