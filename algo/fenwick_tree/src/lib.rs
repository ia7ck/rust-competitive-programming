use std::ops::{Bound, RangeBounds};

/// Fenwick Tree (Binary Indexed Tree) [http://hos.ac/slides/20140319_bit.pdf](http://hos.ac/slides/20140319_bit.pdf)
///
/// # Examples
/// ```
/// use fenwick_tree::FenwickTree;
/// let mut ft = FenwickTree::new(5, 0);
/// ft.add(0, 1);
/// ft.add(2, 10);
/// ft.add(4, 100);
/// // [1, 0, 10, 0, 100]
/// assert_eq!(ft.sum(0..1), 1);
/// assert_eq!(ft.sum(0..2), 1);
/// assert_eq!(ft.sum(0..3), 11);
/// assert_eq!(ft.sum(2..4), 10);
/// assert_eq!(ft.sum(2..5), 110);
/// assert_eq!(ft.sum(0..5), 111);
/// ```
#[derive(Clone, Debug)]
pub struct FenwickTree<T> {
    n: usize,
    e: T,
    dat: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy,
    T: std::ops::AddAssign,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize, e: T) -> Self {
        Self {
            n,
            e,
            dat: vec![e; n + 1],
        }
    }
    // 0-indexed
    // a[k] += x
    pub fn add(&mut self, k: usize, x: T) {
        assert!(k < self.n);
        let mut k = k + 1;
        while k <= self.n {
            self.dat[k] += x;
            k += 1 << k.trailing_zeros();
        }
    }
    // 1-indexed
    // a[1] + a[2] + ... + a[r]
    fn _sum(&self, r: usize) -> T {
        assert!(r <= self.n);
        let mut result = self.e;
        let mut k = r;
        while k >= 1 {
            result += self.dat[k];
            k -= 1 << k.trailing_zeros();
        }
        result
    }
    // 0-indexed
    pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.n,
        };
        assert!(end <= self.n);
        let mut result = self._sum(end);
        result -= self._sum(start);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;
    use rand::prelude::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        for n in 1..=20 {
            let mut a = vec![0; n];
            let mut ft = FenwickTree::new(n, 0);
            for _ in 0..100 {
                let i = rng.gen_range(0, n);
                let x = rng.gen_range(-100, 100);
                a[i] += x;
                ft.add(i, x);
                for (l, r) in (0..n).zip(1..=n) {
                    if l <= r {
                        assert_eq!(a[l..r].iter().sum::<i32>(), ft.sum(l..r))
                    }
                }
            }
        }
    }

    #[test]
    fn test_single() {
        let mut f = FenwickTree::new(1, 0);
        f.add(0, 123);
        assert_eq!(f.sum(0..1), 123);
    }
}
