use std::ops::Range;
/// ソート済の列に対して二分法で"境目"を探します。
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソートされた列 `a` の中で `x` **以上**である最初の要素の位置を返します。全ての要素が `x` 未満のときは `a.len()` を返します。
    ///
    /// # Examples
    /// ```
    /// use binary_search::BinarySearch;
    /// let a = vec![1, 2, 2, 3];
    /// assert_eq!(a.lower_bound(&2), 1);
    /// assert_eq!(a.lower_bound(&9), a.len());
    /// ```
    fn lower_bound(&self, x: &T) -> usize {
        if self[0] >= *x {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        // self[lf] < x
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md] < *x {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    /// ソートされた列 `a` の中で `x` **より大きい**最初の要素の位置を返します。全ての要素が `x` 以下のときは `a.len()` を返します。
    ///
    /// # Examples
    /// ```
    /// use binary_search::BinarySearch;
    /// let a = vec![1, 2, 2, 3];
    /// assert_eq!(a.upper_bound(&2), 3);
    /// assert_eq!(a.upper_bound(&3), a.len());
    /// assert_eq!(a.upper_bound(&9), a.len());
    /// ```
    fn upper_bound(&self, x: &T) -> usize {
        if self[0] > *x {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        // self[lf] <= x
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md] <= *x {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    /// ソートされた列 `a` を
    ///
    /// - `x` 未満
    /// - `x` と等しい
    /// - `x` より大きい
    ///
    /// に分ける添字の範囲を tuple で返します。
    ///
    /// # Examples
    /// ```
    /// use binary_search::BinarySearch;
    /// let a = vec![1, 2, 2, 3];
    /// assert_eq!(a.split_by(&0), (0..0, 0..0, 0..a.len()));
    /// assert_eq!(a.split_by(&2), (0..1, 1..3, 3..a.len()));
    /// assert_eq!(a.split_by(&9), (0..a.len(), a.len()..a.len(), a.len()..a.len()));
    /// ```
    fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>) {
        let i = self.lower_bound(x);
        let j = self.upper_bound(x);
        (0..i, i..j, j..self.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_bound_test() {
        let a = vec![1, 2, 2, 3, 3, 3, 5, 5, 5, 5, 5];

        //   1, 2, 2, ...
        // ^
        assert_eq!(a.lower_bound(&0), 0);

        //   1, 2, 2, ...
        // ^
        assert_eq!(a.lower_bound(&1), 0);

        // 1,   2, 2, ...
        //    ^
        assert_eq!(a.lower_bound(&2), 1);

        // 1, 2, 2,   3, 3, 3, ...
        //          ^
        assert_eq!(a.lower_bound(&3), 3);

        // 1, 2, 2, 3, 3, 3,   5, 5, ...
        //                   ^
        assert_eq!(a.lower_bound(&4), 6);

        // 1, 2, 2, 3, 3, 3,   5, 5, ...
        //                   ^
        assert_eq!(a.lower_bound(&5), 6);

        // ..., 3, 5, 5, 5, 5, 5
        //                       ^
        assert_eq!(a.lower_bound(&6), 11);
    }

    #[test]
    fn upper_bound_test() {
        let a = vec![1, 2, 2, 3, 3, 3, 5, 5, 5, 5, 5];

        //   1, 2, 2, ...
        // ^
        assert_eq!(a.upper_bound(&0), 0);

        // 1,   2, 2, ...
        //    ^
        assert_eq!(a.upper_bound(&1), 1);

        // 1, 2, 2,   3, 3, ...
        //          ^
        assert_eq!(a.upper_bound(&2), 3);

        // 1, 2, 2, 3, 3, 3,   5, 5, ...
        //                   ^
        assert_eq!(a.upper_bound(&3), 6);

        // 1, 2, 2, 3, 3, 3,   5, 5, ...
        //                   ^
        assert_eq!(a.upper_bound(&4), 6);

        // ..., 3, 5, 5, 5, 5, 5
        //                       ^
        assert_eq!(a.upper_bound(&5), 11);

        // ..., 3, 5, 5, 5, 5, 5
        //                       ^
        assert_eq!(a.upper_bound(&6), 11);
    }

    #[test]
    fn split_by_test() {
        let a = vec![1, 2, 2, 3, 3, 3, 5, 5, 5, 5, 5];

        // [(1), (2, 2), (3, 3, 3, 5, 5, 5, 5, 5)]
        assert_eq!(a.split_by(&2), (0..1, 1..3, 3..a.len()));

        // [(), (), (1, 2, 2, 3, 3, 3, 5, 5, 5, 5, 5)]
        assert_eq!(a.split_by(&(-123)), (0..0, 0..0, 0..a.len()));

        // [(1, 2, 2, 3, 3, 3, 5, 5, 5, 5, 5), (), ()]
        assert_eq!(
            a.split_by(&123),
            (0..a.len(), a.len()..a.len(), a.len()..a.len())
        );

        // [(1, 2, 2, 3, 3, 3), (), (5, 5, 5, 5, 5)]
        assert_eq!(a.split_by(&4), (0..6, 6..6, 6..a.len()));
    }
}
