use std::{
    fmt,
    io::{self, BufReader, Cursor},
    str,
};

pub struct Scanner<R>
where
    R: io::BufRead,
{
    reader: R,
    buf: String,
    pos: usize,
}

impl Scanner<BufReader<io::StdinLock<'static>>> {
    /// Creates a scanner that reads from standard input.
    pub fn stdin_lock() -> Self {
        Self {
            reader: BufReader::new(io::stdin().lock()),
            buf: String::new(),
            pos: 0,
        }
    }
}

impl<T> Scanner<Cursor<T>>
where
    T: AsRef<[u8]>,
{
    /// Creates a scanner that reads from a string or byte slice.
    pub fn cursor(inner: T) -> Self {
        Self {
            reader: Cursor::new(inner),
            buf: String::new(),
            pos: 0,
        }
    }
}

impl<R> Scanner<R>
where
    R: io::BufRead,
{
    /// Scans and parses the next token from the input.
    ///
    /// For more convenient input scanning with variable declarations, see [`scan!`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::Scanner;
    /// 
    /// let mut scanner = Scanner::cursor("-10 20");
    /// 
    /// let x = scanner.scan::<i32>();
    /// let y = scanner.scan::<i32>();
    /// 
    /// assert_eq!(x, -10);
    /// assert_eq!(y, 20);
    /// ```
    pub fn scan<T>(&mut self) -> T
    where
        T: str::FromStr,
        T::Err: fmt::Debug,
    {
        // skip whitespace
        loop {
            match self.buf[self.pos..].find(|ch| !char::is_ascii_whitespace(&ch)) {
                Some(j) => {
                    self.pos += j;
                    break;
                }
                None => {
                    let num_bytes = self
                        .reader
                        .read_line(&mut self.buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                }
            }
        }

        let rest = &self.buf[self.pos..];
        let token_len = rest
            .find(|ch| char::is_ascii_whitespace(&ch))
            .unwrap_or(rest.len());
        let value = rest[..token_len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.pos += token_len;

        value
    }
}

/// Macro for convenient input scanning with variable declarations.
///
/// For direct token scanning, see [`Scanner::scan()`].
///
/// # Examples
///
/// ```
/// use scanner::{Scanner, scan};
/// 
/// let mut scanner = Scanner::cursor("3 10\n1 2 3");
/// 
/// scan! {
///     via scanner,
///     (n, k): (usize, usize),
///     a: [i32; n],
/// };
/// 
/// assert_eq!((n, k), (3, 10));
/// assert_eq!(a, vec![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! scan {
    (via $scanner:expr, $($rest:tt)*) => {
        $crate::scan!(@via [$scanner] @rest $($rest)*);
    };

    (@via [$via:expr] @rest) => {};
    (@via [$via:expr] @rest ,) => {};

    (@via [$via:expr] @rest mut $($rest:tt)*) => {
        $crate::scan!(@via [$via] @mut [mut] @rest $($rest)*);
    };
    (@via [$via:expr] @rest $($rest:tt)*) => {
        $crate::scan!(@via [$via] @mut [] @rest $($rest)*);
    };

    (@via [$via:expr] @mut [$($mut:tt)?] @rest $var:tt : $t:tt) => {
        let $($mut)? $var = $crate::scan_inner!(via $via, $t);
    };
    (@via [$via:expr] @mut [$($mut:tt)?] @rest $var:tt : $t:tt , $($rest:tt)*) => {
        $crate::scan!(@via [$via] @mut [$($mut)?] @rest $var : $t);
        $crate::scan!(@via [$via] @rest $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! scan_inner {
    // (i32, i32)
    (via $scanner:expr, ( $($t:tt),* )) => {
        ( $($crate::scan_inner!(via $scanner, $t)),* )
    };

    // [i32; n]
    (via $scanner:expr, [ $t:tt ; $len:expr ]) => {
        ::std::iter::repeat_with(|| $crate::scan_inner!(via $scanner, $t)).take($len).collect::<Vec<_>>()
    };

    // i32
    (via $scanner:expr, $ty:ty) => {
        $scanner.scan::<$ty>()
    };
}

#[cfg(test)]
mod tests {
    use crate::Scanner;

    #[test]
    fn scan_test() {
        let mut scanner = Scanner::cursor("42 123\n456\r\nABC");
        assert_eq!(scanner.scan::<i32>(), 42);
        assert_eq!(scanner.scan::<i32>(), 123);
        assert_eq!(scanner.scan::<i32>(), 456);
        assert_eq!(scanner.scan::<String>(), String::from("ABC"));
    }

    #[test]
    fn scan_macro_test() {
        let mut scanner = Scanner::cursor(
            r#"
3 10
4
1 2 3
a 1
b 2
c 3
d 4
        "#,
        );
        scan! {
            via scanner,
            (n, k): (usize, usize),
            mut q: usize,
            a: [i32; n],
            queries: [(char, i32); q],
        };

        assert_eq!((n, k), (3, 10));
        assert_eq!(q, 4);
        // test mutable
        q += 1;
        assert_eq!(q, 5);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(queries, vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
    }
}
