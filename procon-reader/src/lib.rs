pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
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
