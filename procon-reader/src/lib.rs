pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    /// 標準入力から読み込みたいときの例です。
    /// ファイルからの読み込みは [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html) の Examples を参考にしてください。
    /// # Examples
    /// ```
    /// use std::io;
    /// use procon_reader::ProconReader;
    ///
    /// let stdin = io::stdin();
    /// let mut rd = ProconReader::new(io::BufReader::new(stdin));
    /// // ProconReader::new(stdin.lock()); のほうが気持ち速いです
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    /// 空白・改行で区切られている値を取得します。適宜、型アノテーションをつけてください。
    /// # Examples
    /// ```
    /// use std::io;
    /// use procon_reader::ProconReader;
    ///
    /// let mut rd = ProconReader::new(io::Cursor::new("123 abc\nx"));
    /// let n = rd.get::<usize>();
    /// assert_eq!(n, 123);
    /// let s: String = rd.get();
    /// assert_eq!(s, "abc");
    /// let ch: char = rd.get();
    /// assert_eq!(ch, 'x');
    /// ```
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or(line.len());
        let s = &line[..end];
        self.i += end;
        s.parse().expect(&format!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).expect("not valid utf-8");
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
                }
            }
        }
    }
    /// よくある「空白区切りの数値を `n` 個」取得したいときなどに使えます。
    /// # Examples
    /// ```
    /// use std::io;
    /// use procon_reader::ProconReader;
    ///
    /// let mut rd = ProconReader::new(io::Cursor::new("123 45 -6"));
    /// let a: Vec<i32> = rd.get_vec(3);
    /// assert_eq!(a, vec![123, 45, -6]);
    /// ```
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::ProconReader;
    use std::io::Cursor;

    fn get<T>(input: &str) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
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
        let input = "123\n-123\n\n\na\nabc";
        let mut rd = ProconReader::new(Cursor::new(input));
        assert_eq!(rd.get::<usize>(), 123);
        assert_eq!(rd.get::<i32>(), -123);
        assert_eq!(rd.get::<char>(), 'a');
        assert_eq!(rd.get::<String>(), "abc");
    }

    fn get_vec<T>(input: &str, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        ProconReader::new(Cursor::new(input)).get_vec(n)
    }

    #[test]
    fn test_get_vec() {
        assert_eq!(get_vec::<i32>("1 23 -456", 3), vec![1, 23, -456]);
        assert_eq!(get_vec::<String>("abc\nde\nf", 3), vec!["abc", "de", "f"]);
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
