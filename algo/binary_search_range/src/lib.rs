use std::ops;

/// ソート済みの列を検索します。
pub trait BinarySearchRange<T> {
    fn range(&self, range: ops::Range<T>) -> ops::Range<usize>;
}

impl<T: Ord> BinarySearchRange<T> for [T] {
    /// ソート済みの列に対して値の範囲に対応した index の範囲を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_search_range::BinarySearchRange;
    ///
    /// let a = vec![3, 3, 4, 5, 7, 7];
    /// assert_eq!(a.range(0..5), 0..3); // [3, 3, 4         ]
    /// assert_eq!(a.range(4..5), 2..3); // [      4         ]
    /// assert_eq!(a.range(4..6), 2..4); // [      4, 5      ]
    /// assert_eq!(a.range(4..8), 2..6); // [      4, 5, 7, 7]
    /// ```
    fn range(&self, range: ops::Range<T>) -> ops::Range<usize> {
        assert!(!self.is_empty());
        assert!(range.start < range.end);

        let first_ge = |x: &T| {
            if x.le(&self[0]) {
                return 0;
            }
            let mut left = 0;
            // self[left] < x
            let mut right = self.len();
            while right - left > 1 {
                let mid = (right + left) / 2;
                if self[mid].lt(x) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            assert!(self[left].lt(x));
            assert!(right <= self.len());
            if right < self.len() {
                assert!(x.le(&self[right]));
            }
            right
        };

        let start_index = first_ge(&range.start);
        let end_index = first_ge(&range.end);
        start_index..end_index
    }
}

#[cfg(test)]
mod tests {
    use crate::BinarySearchRange;

    #[test]
    #[should_panic]
    fn test_panic() {
        vec![].range(0..10);
    }

    #[test]
    #[should_panic]
    fn test_panic2() {
        vec![1, 2, 3].range(2..2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(vec![3, 4, 8].range(0..3), 0..0);
        assert_eq!(vec![3, 4, 8].range(5..8), 2..2);
        assert_eq!(vec![3, 4, 8].range(9..12), 3..3);
    }

    #[test]
    fn test_whole() {
        let a = vec![3, 4, 5];
        assert_eq!(a.range(3..6), 0..a.len());
        assert_eq!(a.range(1..6), 0..a.len());
        assert_eq!(a.range(3..7), 0..a.len());
        assert_eq!(a.range(0..8), 0..a.len());
    }

    #[test]
    fn test() {
        let a = vec![2, 2, 5, 5, 6, 7, 10];
        assert_eq!(a.range(0..3), 0..2);
        assert_eq!(a.range(0..5), 0..2);
        assert_eq!(a.range(0..6), 0..4);
        assert_eq!(a.range(0..7), 0..5);
        assert_eq!(a.range(0..8), 0..6);
        assert_eq!(a.range(3..5), 2..2);
        assert_eq!(a.range(3..6), 2..4);
        assert_eq!(a.range(3..7), 2..5);
        assert_eq!(a.range(3..8), 2..6);
    }
}
