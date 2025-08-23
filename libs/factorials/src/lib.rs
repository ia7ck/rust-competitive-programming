/// 階乗とその乗法逆元、そして二項係数を扱います。
pub struct Factorial {
    factorial: Vec<u64>,
    inversion_of_factorial: Vec<u64>,
    modulo: u64,
}

impl Factorial {
    /// `1` 以上 `size` 未満の `n` について、`n` の階乗 (mod `modulo`) と、その乗法逆元を O(`size`) 時間で計算します。[参考](https://drken1215.hatenablog.com/entry/2018/06/08/210000)
    ///
    /// 逆元を正しく計算するためには
    ///
    /// - `modulo` が素数
    /// - `modulo >= size`
    ///
    /// である必要があります。
    ///
    /// # Examples
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let modulo = 1_000_000_000 + 7;
    /// let f = Factorial::new(100, modulo);
    /// for i in 1..100 {
    ///     assert_eq!(f.factorial(i) * f.inversion(i) % modulo, 1);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// `modulo` が `size` より小さい場合パニックです。
    ///
    /// ```should_panic
    /// use factorials::Factorial;
    ///
    /// let size = 100;
    /// let modulo = 97;
    /// Factorial::new(size, modulo);
    /// ```
    pub fn new(size: usize, modulo: u64) -> Self {
        assert!(modulo >= size as u64);
        let mut fac = vec![0; size];
        let mut inv = vec![0; size];
        let mut inv_of_fac = vec![0; size];
        fac[0] = 1;
        fac[1] = 1;
        inv[1] = 1;
        inv_of_fac[0] = 1;
        inv_of_fac[1] = 1;
        for i in 2..size {
            let i_u64 = i as u64;
            fac[i] = fac[i - 1] * i_u64 % modulo;
            inv[i] = ((modulo - inv[(modulo as usize) % i]) * (modulo / i_u64)).rem_euclid(modulo);
            inv_of_fac[i] = inv_of_fac[i - 1] * inv[i] % modulo;
        }
        Self {
            factorial: fac,
            inversion_of_factorial: inv_of_fac,
            modulo,
        }
    }

    /// `modulo` が素数でない場合パニックです。素数判定に O(sqrt(`modulo`)) 時間かかります。
    ///
    /// # Panics
    ///
    /// ```should_panic
    /// use factorials::Factorial;
    ///
    /// let modulo = 42;
    /// Factorial::new_checking_modulo_prime(10, 42);
    /// ```
    pub fn new_checking_modulo_prime(size: usize, modulo: u64) -> Self {
        assert!((2..modulo)
            .take_while(|&x| x * x <= modulo)
            .all(|x| modulo % x != 0));
        Self::new(size, modulo)
    }

    pub fn factorial(&self, n: usize) -> u64 {
        assert!(n < self.factorial.len());
        self.factorial[n]
    }

    pub fn inversion(&self, n: usize) -> u64 {
        assert!(n < self.inversion_of_factorial.len());
        self.inversion_of_factorial[n]
    }

    /// 二項係数を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let f = Factorial::new_checking_modulo_prime(5, 107);
    /// assert_eq!(f.binomial(4, 0), 1);
    /// assert_eq!(f.binomial(4, 1), 4);
    /// assert_eq!(f.binomial(4, 2), 6);
    /// assert_eq!(f.binomial(4, 3), 4);
    /// assert_eq!(f.binomial(4, 4), 1);
    /// ```
    ///
    /// # Panics
    ///
    /// 以下の少なくともひとつが成り立つ場合パニックです。
    ///
    /// - `n` が構築時の `size` 以上
    /// - `k` が構築時の `size` 以上
    /// - `n` が `k` より小さい
    ///
    /// ```should_panic
    /// use factorials::Factorial;
    ///
    /// let f = Factorial::new_checking_modulo_prime(5, 107);
    /// f.binomial(3, 4); // n < k
    /// ```
    pub fn binomial(&self, n: usize, k: usize) -> u64 {
        assert!(n < self.factorial.len());
        assert!(k < self.inversion_of_factorial.len());
        assert!(n >= k);
        self.factorial(n) * self.inversion(k) % self.modulo * self.inversion(n - k) % self.modulo
    }

    /// [`binomial`] とほとんど同じですが `n` が `k` より小さいときパニックせずに `0` を返します。
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let f = Factorial::new_checking_modulo_prime(5, 107);
    /// assert_eq!(f.binomial_or_zero(3, 4), 0);
    /// ```
    ///
    /// [`binomial`]: struct.Factorial.html#method.binomial
    pub fn binomial_or_zero(&self, n: usize, k: usize) -> u64 {
        assert!(n < self.factorial.len());
        assert!(k < self.inversion_of_factorial.len());
        if n < k {
            return 0;
        }
        self.binomial(n, k)
    }
}

#[cfg(test)]
mod tests {
    use super::Factorial;
    #[test]
    fn test_mod_is_103() {
        let p = 103;
        let f = Factorial::new(100, p);
        for i in 1..100 {
            assert_eq!(f.factorial(i) * f.inversion(i) % p, 1);
        }
    }

    #[test]
    fn test_binomial() {
        let f = Factorial::new(6, 1_000_000_000 + 7);
        let b: Vec<Vec<u64>> = (0..6)
            .map(|n| (0..6).map(|k| f.binomial_or_zero(n, k)).collect())
            .collect();
        assert_eq!(
            b,
            vec![
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0],
                vec![1, 2, 1, 0, 0, 0],
                vec![1, 3, 3, 1, 0, 0],
                vec![1, 4, 6, 4, 1, 0],
                vec![1, 5, 10, 10, 5, 1],
            ]
        )
    }
}
