use std::fmt;
use std::ops::{Bound, Index, RangeBounds};

/// __注意⚠__ この実装は遅いので time limit の厳しい問題には代わりに ACL のセグメントツリーを使うこと。
///
/// セグメントツリーです。
#[derive(Clone)]
pub struct SegmentTree<T, F> {
    original_n: usize,
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

// https://hcpc-hokudai.github.io/archive/structure_segtree_001.pdf
impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    /// 長さ `n` の列を初期値 `e` で初期化します。
    ///
    /// `multiply` は fold に使う二項演算です。
    pub fn new(n: usize, e: T, multiply: F) -> Self {
        let original_n = n;
        let n = n.next_power_of_two();
        Self {
            original_n,
            n,
            dat: vec![e.clone(); n * 2], // dat[0] is unused
            e,
            multiply,
        }
    }

    /// 列の `i` 番目の要素を取得します。
    pub fn get(&self, i: usize) -> &T {
        assert!(i < self.original_n);
        &self.dat[i + self.n]
    }

    /// 列の `i` 番目の要素を `x` で更新します。
    pub fn set(&mut self, i: usize, x: T) {
        self.update(i, |_| x);
    }

    /// 列の `i` 番目の要素を `f` で更新します。
    pub fn update<U>(&mut self, i: usize, f: U)
    where
        U: FnOnce(&T) -> T,
    {
        assert!(i < self.original_n);
        let mut k = i + self.n;
        self.dat[k] = f(&self.dat[k]);
        while k > 1 {
            k >>= 1;
            self.dat[k] = (self.multiply)(&self.dat[k << 1], &self.dat[k << 1 | 1]);
        }
    }

    /// `range` が `l..r` として、`multiply(l番目の要素, multiply(..., multiply(r-2番目の要素, r-1番目の要素)))` の値を返します。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.original_n,
        };
        assert!(start <= end && end <= self.original_n);
        self._fold(start, end)
    }

    /// `f(fold(l..r)) = true` となる最大の `r` を返します。
    ///
    /// # Panics
    ///
    /// if `f(e) = false`
    pub fn max_right<P>(&self, l: usize, f: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        assert!(l <= self.original_n);
        assert!(f(&self.e), "f(e) must be true");

        if l == self.original_n {
            return self.original_n;
        }

        let mut l = l + self.n;
        let mut sum = self.e.clone();

        loop {
            // l を含む区間の右端まで進む
            while l % 2 == 0 {
                l >>= 1;
            }

            let new_sum = (self.multiply)(&sum, &self.dat[l]);
            if !f(&new_sum) {
                while l < self.n {
                    l <<= 1;
                    let new_sum = (self.multiply)(&sum, &self.dat[l]);
                    if f(&new_sum) {
                        sum = new_sum;
                        l += 1;
                    }
                }
                return l - self.n;
            }

            sum = new_sum;
            l += 1;

            if (l & (l.wrapping_neg())) == l {
                break;
            }
        }

        self.original_n
    }

    /// `f(fold(l..r)) = true` となる最小の `l` を返します。
    ///
    /// # Panics
    ///
    /// if `f(e) = false`
    pub fn min_left<P>(&self, r: usize, f: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        assert!(r <= self.original_n);
        assert!(f(&self.e), "f(e) must be true");

        if r == 0 {
            return 0;
        }

        let mut r = r + self.n;
        let mut sum = self.e.clone();

        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }

            let new_sum = (self.multiply)(&self.dat[r], &sum);
            if !f(&new_sum) {
                while r < self.n {
                    r = r * 2 + 1;
                    let new_sum = (self.multiply)(&self.dat[r], &sum);
                    if f(&new_sum) {
                        sum = new_sum;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }

            sum = new_sum;

            if (r & (r.wrapping_neg())) == r {
                break;
            }
        }

        0
    }

    fn _fold(&self, mut l: usize, mut r: usize) -> T {
        let mut acc_l = self.e.clone();
        let mut acc_r = self.e.clone();
        l += self.n;
        r += self.n;
        while l < r {
            if l & 1 == 1 {
                // 右の子だったらいま足しておかないといけない
                // 左の子だったら祖先のどれかで足されるのでよい
                acc_l = (self.multiply)(&acc_l, &self.dat[l]);
                l += 1;
            }
            if r & 1 == 1 {
                // r が exclusive であることに注意する
                r -= 1;
                acc_r = (self.multiply)(&self.dat[r], &acc_r);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.multiply)(&acc_l, &acc_r)
    }
}

impl<T, F> Index<usize> for SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T, F> fmt::Debug for SegmentTree<T, F>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.dat[self.n..])
    }
}

#[cfg(test)]
mod tests {
    use crate::SegmentTree;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let mut seg = SegmentTree::new(s.len(), String::new(), |a, b| format!("{a}{b}"));
        for (i, c) in s.chars().enumerate() {
            seg.set(i, c.to_string());
        }

        for i in 0..s.len() {
            assert_eq!(s[..i], seg.fold(..i));
            assert_eq!(s[i..], seg.fold(i..));
        }

        for i in 0..s.len() {
            for j in i..s.len() {
                assert_eq!(s[i..j], seg.fold(i..j));
                if j + 1 < s.len() {
                    assert_eq!(s[i..=j], seg.fold(i..=j));
                }
            }
        }
    }

    #[test]
    fn single_element() {
        let mut seg = SegmentTree::new(1, 0, |a, b| a + b);
        assert_eq!(seg[0], 0);
        seg.set(0, 42);
        assert_eq!(seg[0], 42);
    }

    #[test]
    fn test_max_right() {
        let n = 9;
        let mut seg = SegmentTree::new(n, 0, |a, b| a + b);
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        for (i, &v) in values.iter().enumerate() {
            seg.set(i, v);
        }

        // 区間和
        assert_eq!(seg.max_right(0, |&sum| sum < 9), 3); // 3 + 1 + 4 = 8
        assert_eq!(seg.max_right(0, |&sum| sum <= 9), 4); // 3 + 1 + 4 + 1 = 9

        assert_eq!(seg.max_right(1, |&sum| sum < 11), 4); // 1 + 4 + 1 = 6
        assert_eq!(seg.max_right(1, |&sum| sum <= 11), 5); // 1 + 4 + 1 + 5 = 11

        assert_eq!(seg.max_right(2, |&sum| sum < 4), 2);
        assert_eq!(seg.max_right(2, |&sum| sum <= 4), 3);
        assert_eq!(seg.max_right(2, |&sum| sum <= 100), n);

        assert_eq!(seg.max_right(n, |&sum| sum <= 0), n);
        assert_eq!(seg.max_right(n, |&sum| sum <= 100), n);
    }

    #[test]
    fn test_min_left() {
        let n = 9;
        let mut seg = SegmentTree::new(n, 0, |a, b| a + b);
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        for (i, &v) in values.iter().enumerate() {
            seg.set(i, v);
        }

        // 区間和
        assert_eq!(seg.min_left(n, |&sum| sum <= 22), 5); // 9 + 2 + 6 + 5 = 22
        assert_eq!(seg.min_left(n, |&sum| sum < 22), 6); // 2 + 6 + 5 = 13

        assert_eq!(seg.min_left(n - 1, |&sum| sum <= 27), 2); // 4 + 1 + 5 + 9 + 2 + 6 = 27
        assert_eq!(seg.min_left(n - 1, |&sum| sum < 27), 3); // 1 + 5 + 9 + 2 + 6 = 23
        assert_eq!(seg.min_left(n - 1, |&sum| sum < 100), 0);

        assert_eq!(seg.min_left(0, |&sum| sum <= 0), 0);
        assert_eq!(seg.min_left(0, |&sum| sum <= 100), 0);
    }
}
