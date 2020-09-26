pub struct FenwickTree<T> {
    n: usize,
    e: T,
    dat: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy,
    T: std::ops::AddAssign,
    T: std::ops::Sub<Output = T>,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
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
        let mut k = (k + 1) as i32;
        while k <= self.n as i32 {
            self.dat[k as usize] += x;
            k += k & (-k);
        }
    }
    // 1-indexed
    // a[1] + a[2] + ... + a[r]
    fn _sum(&self, r: usize) -> T {
        assert!(r <= self.n);
        let mut result = self.e;
        let mut k = r as i32;
        while k >= 1 {
            result += self.dat[k as usize];
            k -= k & (-k);
        }
        result
    }
    // 0-indexed
    // a[l] + a[l + 1] + ... + a[r - 1]
    pub fn sum(&self, range: std::ops::Range<usize>) -> T {
        let (l, r) = (range.start, range.end);
        assert!(r <= self.n);
        self._sum(r) - self._sum(l)
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
            let mut a = vec![0; n];
            let mut ft = FenwickTree::new(n, 0);
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
