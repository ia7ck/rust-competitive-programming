use mod_int::{ModInt, Modulo};

pub trait BinomialCoefficient {
    type Output;
    /// 二項係数「`n` 個の物から `k` 個を選ぶ通り数」を返します。
    fn get(&self, n: usize, k: usize) -> Self::Output;
}

pub struct Binom {
    size: usize,
    mo: i64,
    fac: Vec<i64>,
    inv_fac: Vec<i64>,
}

impl Binom {
    /// `0` 以上 `size` 未満の `n` について
    ///
    /// - `fac[n]`: `n * (n - 1) * ... * 2 * 1 % mo`
    /// - `inv_fac[n]`: `fac[n]` の (乗法に関する) 逆元
    ///     - つまり `fac[n] * inv_fac[n] % mo == 1`
    ///
    /// を線形時間で構築します。この前計算で `BinomialCoefficient::get(n, k)` を `O(1)` にします。
    ///
    /// `mo` は素数にしてください。
    pub fn new(size: usize, mo: i64) -> Self {
        let mut fac = vec![0; size];
        let mut inv = vec![0; size];
        let mut inv_fac = vec![0; size];
        fac[0] = 1;
        fac[1] = 1;
        inv[1] = 1;
        inv_fac[0] = 1;
        inv_fac[1] = 1;
        for i in 2..size {
            fac[i] = fac[i - 1] * (i as i64) % mo;
            inv[i] = (-inv[(mo as usize) % i] * (mo / (i as i64))).rem_euclid(mo);
            inv_fac[i] = inv_fac[i - 1] * inv[i] % mo;
        }
        Self {
            size,
            mo,
            fac,
            inv_fac,
        }
    }
}

impl BinomialCoefficient for Binom {
    type Output = i64;
    /// 二項係数 `% mo` を定数時間で計算します。法 `mo` は構築時に与えたパラメータです。
    ///
    /// # Panics
    /// 構築時の `size` 以上の `n` を与えると `panic` です。
    ///
    /// ```should_panic
    /// use combination::{Binom, BinomialCoefficient};
    /// let binom = Binom::new(20, 1000000007);
    /// println!(
    ///     "There are {} ways to choose 10 items from 20 items.",
    ///     binom.get(20, 10)
    /// ); // panic
    /// ```
    fn get(&self, n: usize, k: usize) -> Self::Output {
        assert!(n < self.size);
        if n < k {
            return 0;
        }
        ((self.fac[n] * self.inv_fac[k]) % self.mo * self.inv_fac[n - k]) % self.mo
    }
}

pub struct BinomWithModInt<M: Modulo> {
    size: usize,
    fac: Vec<ModInt<M>>,
}

impl<M: Modulo> BinomWithModInt<M> {
    /// `0` 以上 `size` 未満の `n` について
    ///
    /// - `fac[n]`: `ModInt(n) * ModInt(n - 1) * ... * ModInt(2) * ModInt(1)`
    ///
    /// を構築します。
    ///
    /// 型パラメータ `M` は `mod_int::Modulo` を実装した型です。
    ///
    /// - `mod_int::Mod1000000007`
    /// - `mod_int::Mod998244353`
    ///
    /// などが該当します。
    ///
    /// # Examples
    /// ```
    /// use combination::{BinomWithModInt, BinomialCoefficient};
    /// use mod_int::Mod1000000007;
    /// let binom = BinomWithModInt::<Mod1000000007>::new(123);
    /// let choose_34_5 = binom.get(34, 5);
    /// ```
    pub fn new(size: usize) -> Self {
        let mut fac = vec![ModInt::new(0); size];
        fac[0] = ModInt::new(1);
        for i in 1..size {
            fac[i] = fac[i - 1] * ModInt::new(i as i64);
        }
        Self { size, fac }
    }
}

impl<M: Modulo> BinomialCoefficient for BinomWithModInt<M> {
    type Output = ModInt<M>;
    /// 二項係数を `ModInt` で wrap して返します。割り算をするので計算量は `ModInt` の法 `p` について対数時間です。
    /// # Panics
    /// 構築時の `size` 以上の `n` を与えると `panic` です。
    ///
    /// ```should_panic
    /// use combination::{BinomWithModInt, BinomialCoefficient};
    /// use mod_int::Mod1000000007;
    /// let binom = BinomWithModInt::<Mod1000000007>::new(20);
    /// println!(
    ///     "There are {} ways to choose 10 items from 20 items.",
    ///     binom.get(20, 10).val()
    /// ); // panic
    /// ```
    fn get(&self, n: usize, k: usize) -> Self::Output {
        assert!(n < self.size);

        if n < k {
            return ModInt::new(0);
        }
        self.fac[n] / self.fac[k] / self.fac[n - k]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Binom, BinomWithModInt, BinomialCoefficient};
    use mod_int::{define_mod_int_p, ModInt, Modulo};

    #[test]
    fn check_by_pascal_triangle() {
        const N: usize = 100;
        const K: usize = 100;
        const M: i64 = 107;
        let mut dp = vec![vec![0; K]; N];
        dp[0][0] = 1;
        for i in 1..N {
            dp[i][0] = 1;
            for j in 1..K {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % M;
            }
        }
        let binom_pascal_triangle = |n: usize, k: usize| dp[n][k];
        let binom = Binom::new(N, M);
        define_mod_int_p!(Mod107, ModInt107, M);
        let binom_mint = BinomWithModInt::<Mod107>::new(N);
        for i in 0..N {
            for j in 0..=i {
                let expect = binom_pascal_triangle(i, j);
                assert_eq!(binom.get(i, j), expect);
                assert_eq!(binom_mint.get(i, j).val(), expect);
            }
        }
    }
}
