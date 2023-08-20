use std::{cmp::Eq, iter::Peekable};

/// [run length encoding](https://ja.wikipedia.org/wiki/%E9%80%A3%E9%95%B7%E5%9C%A7%E7%B8%AE) です。
///
/// ```
/// use run_length::RunLength;
///
/// let a = vec![1, 1, 2, 3, 4, 4, 4];
/// let mut iter = RunLength::new(a.iter());
/// assert_eq!(iter.next(), Some((&1, 2)));
/// assert_eq!(iter.next(), Some((&2, 1)));
/// assert_eq!(iter.next(), Some((&3, 1)));
/// assert_eq!(iter.next(), Some((&4, 3)));
/// assert_eq!(iter.next(), None);
/// ```
pub struct RunLength<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
}

impl<I> RunLength<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<I> Iterator for RunLength<I>
where
    I: Iterator,
    I::Item: Eq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        let mut len = 1;
        while let Some(y) = self.iter.peek() {
            if &x == y {
                len += 1;
                self.iter.next(); // y
            } else {
                break;
            }
        }
        Some((x, len))
    }
}

#[cfg(test)]
mod tests {
    use super::RunLength;

    #[test]
    fn test() {
        let a = vec![3, 1, 1, 4, 1, 5, 5, 5];
        let mut iter = RunLength::new(a.iter());
        assert_eq!(iter.next(), Some((&3, 1)));
        assert_eq!(iter.next(), Some((&1, 2)));
        assert_eq!(iter.next(), Some((&4, 1)));
        assert_eq!(iter.next(), Some((&1, 1)));
        assert_eq!(iter.next(), Some((&5, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_all() {
        let a = vec![7, 7, 7];
        let mut iter = RunLength::new(a.iter());
        assert_eq!(iter.next(), Some((&7, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty() {
        let a = Vec::<i32>::new();
        let mut iter = RunLength::new(a.iter());
        assert_eq!(iter.next(), None);
    }
}
