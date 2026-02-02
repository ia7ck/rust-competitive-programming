use std::cmp::Eq;

/// [run length encoding](https://ja.wikipedia.org/wiki/%E9%80%A3%E9%95%B7%E5%9C%A7%E7%B8%AE) です。
///
/// ```
/// use run_length::RunLength;
///
/// let a = vec![1, 1, 2, 3, 4, 4, 4];
/// let mut iter = RunLength::new(&a);
/// assert_eq!(iter.next(), Some((&1, 2)));
/// assert_eq!(iter.next(), Some((&2, 1)));
/// assert_eq!(iter.next(), Some((&3, 1)));
/// assert_eq!(iter.next(), Some((&4, 3)));
/// assert_eq!(iter.next(), None);
/// ```
pub struct RunLength<'a, T> {
    items: &'a Vec<T>,
    start: usize,
    end: usize,
}

impl<'a, T> RunLength<'a, T> {
    pub fn new(items: &'a Vec<T>) -> Self {
        Self {
            items,
            start: 0,
            end: items.len(),
        }
    }
}

impl<'a, T> Iterator for RunLength<'a, T>
where
    T: Eq,
{
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let x = &self.items[self.start];
        let mut len = 0;
        while self.start + len < self.end && &self.items[self.start + len] == x {
            len += 1;
        }
        self.start += len;
        Some((x, len))
    }
}

impl<'a, T> DoubleEndedIterator for RunLength<'a, T>
where
    T: Eq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let x = &self.items[self.end - 1];
        let mut len = 0;
        while self.start < self.end - len && &self.items[self.end - len - 1] == x {
            len += 1;
        }
        self.end -= len;
        Some((x, len))
    }
}

#[cfg(test)]
mod tests {
    use ::proptest::{collection, prelude::*};

    use super::RunLength;

    #[test]
    fn test() {
        let a = vec![3, 1, 1, 4, 1, 5, 5, 5, 9, 9, 9, 2, 2];
        let mut iter = RunLength::new(&a);
        assert_eq!(iter.next(), Some((&3, 1)));
        assert_eq!(iter.next(), Some((&1, 2)));
        assert_eq!(iter.next(), Some((&4, 1)));
        assert_eq!(iter.next(), Some((&1, 1)));
        assert_eq!(iter.next(), Some((&5, 3)));
        assert_eq!(iter.next_back(), Some((&2, 2)));
        assert_eq!(iter.next_back(), Some((&9, 3)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_all() {
        let a = vec![7, 7, 7];
        let mut iter = RunLength::new(&a);
        assert_eq!(iter.next(), Some((&7, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty() {
        let a = Vec::<i32>::new();
        let mut iter = RunLength::new(&a);
        assert_eq!(iter.next(), None);
    }

    proptest! {
        #[test]
        fn round_trip(items in collection::vec(proptest::char::range('a', 'z'), 0..=20)) {
            let rle = RunLength::new(&items);

            let concat = rle.fold(Vec::new(), |mut acc, (&c, l)| {
                acc.append(&mut vec![c; l]);
                acc
            });

            prop_assert_eq!(items, concat);
        }

        #[test]
        fn adjacent_runs_differ(items in collection::vec(proptest::num::i32::ANY, 0..=20)) {
            let rle = RunLength::new(&items).collect::<Vec<_>>();

            for w in rle.windows(2) {
                let (c0, _) = w[0];
                let (c1, _) = w[1];
                prop_assert_ne!(c0, c1);
            }
        }
    }
}
