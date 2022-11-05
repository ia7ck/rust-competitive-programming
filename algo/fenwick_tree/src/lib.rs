use std::ops::Range;

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
        let n = n.next_power_of_two();
        let mut dat = vec![e; n + 1];
        for i in (2..=n).step_by(2) {
            let j = 1 << (i.trailing_zeros() - 1);
            let mut y = dat[j];
            y += dat[j];
            dat[i] = y; // dat[j] * 2
        }
        Self { n, e, dat }
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
    // a[l] + a[l + 1] + ... + a[r - 1]
    pub fn sum(&self, range: Range<usize>) -> T {
        let (l, r) = (range.start, range.end);
        assert!(r <= self.n);
        let mut result = self._sum(r);
        result -= self._sum(l);
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
        for _ in 0..100 {
            let n: usize = rng.gen_range(1, 20);
            let e = rng.gen_range(-100, 100);
            let mut a = vec![e; n];
            let mut ft = FenwickTree::new(n, e);
            for _ in 0..100 {
                let i: usize = rng.gen_range(0, n);
                let x: i32 = rng.gen_range(-100, 100);
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
}
