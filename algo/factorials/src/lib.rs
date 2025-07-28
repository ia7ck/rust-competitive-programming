//! 階乗とその乗法逆元、二項係数を効率的に計算するライブラリです。
//!
//! 素数を法とする環での階乗の前計算と、それを利用した二項係数の高速計算を提供します。
//! 多くの組み合わせ論的計算において、二項係数を O(1) で計算できるため非常に有用です。
//!
//! # 計算量
//!
//! - 前計算: O(n)
//! - 各クエリ（factorial, binomial など）: O(1)
//!
//! # 用途
//!
//! - 二項係数 C(n, k) の高速計算
//! - 組み合わせ論的問題の解決
//! - 動的プログラミングでの状態数計算
//! - 確率計算（mod での除算を含む）
//! - 競技プログラミングでの数学問題
//!
//! # 制約
//!
//! - 法 `modulo` は素数である必要があります
//! - `modulo >= size` である必要があります（階乗の計算範囲内）
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use factorials::Factorial;
//!
//! let modulo = 1_000_000_007;
//! let f = Factorial::new(100, modulo);
//!
//! // 階乗の計算
//! assert_eq!(f.factorial(5), 120); // 5! = 120
//!
//! // 二項係数の計算
//! assert_eq!(f.binomial(5, 2), 10); // C(5,2) = 10
//! assert_eq!(f.binomial(10, 3), 120); // C(10,3) = 120
//! ```
//!
//! ## 組み合わせ論的問題での使用例
//!
//! ```
//! use factorials::Factorial;
//!
//! let modulo = 1_000_000_007;
//! let f = Factorial::new(1000, modulo);
//!
//! // n 個の異なる玉から k 個選ぶ組み合わせ
//! let n = 50;
//! let k = 20;
//! let combinations = f.binomial(n, k);
//!
//! // 重複組み合わせ H(n, k) = C(n+k-1, k)
//! let repeated_combinations = f.binomial(n + k - 1, k);
//!
//! // カタラン数 C_n = C(2n, n) / (n+1)
//! let catalan_5 = f.binomial(10, 5) * f.inversion(6) % modulo;
//! assert_eq!(catalan_5, 42); // 5番目のカタラン数
//! ```

/// 階乗とその乗法逆元、そして二項係数を扱います。
pub struct Factorial {
    factorial: Vec<u64>,
    inversion_of_factorial: Vec<u64>,
    modulo: u64,
}

impl Factorial {
    /// 指定されたサイズと法で階乗計算構造体を初期化します。
    ///
    /// `0` 以上 `size` 未満の `n` について、`n` の階乗 (mod `modulo`) と、
    /// その乗法逆元を O(`size`) 時間で前計算します。
    /// [参考](https://drken1215.hatenablog.com/entry/2018/06/08/210000)
    ///
    /// # 前提条件
    ///
    /// 逆元を正しく計算するためには以下の条件が必要です：
    /// - `modulo` が素数
    /// - `modulo >= size`
    ///
    /// # Examples
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let modulo = 1_000_000_007;
    /// let f = Factorial::new(100, modulo);
    /// for i in 1..100 {
    ///     assert_eq!(f.factorial(i) * f.inversion(i) % modulo, 1);
    /// }
    /// ```
    ///
    /// ## 競技プログラミングでの典型的な使用パターン
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// // 最大 10^6 までの二項係数を扱う場合
    /// let modulo = 1_000_000_007;
    /// let max_n = 1_000_000;
    /// let f = Factorial::new(max_n + 1, modulo);
    ///
    /// // 多数のクエリを O(1) で処理
    /// let mut total = 0;
    /// for n in 1..=100 {
    ///     for k in 0..=n {
    ///         total = (total + f.binomial(n, k)) % modulo;
    ///     }
    /// }
    /// // total は 2^1 + 2^2 + ... + 2^100 (mod p)
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

    /// 素数性をチェックしてから階乗計算構造体を初期化します。
    ///
    /// `new` メソッドと同じ機能ですが、`modulo` が素数でない場合にパニックします。
    /// 素数判定に O(√`modulo`) 時間かかります。
    ///
    /// # Panics
    ///
    /// `modulo` が素数でない場合、または `modulo < size` の場合にパニックします。
    ///
    /// ```should_panic
    /// use factorials::Factorial;
    ///
    /// let modulo = 42; // 素数ではない
    /// Factorial::new_checking_modulo_prime(10, 42);
    /// ```
    pub fn new_checking_modulo_prime(size: usize, modulo: u64) -> Self {
        assert!((2..modulo)
            .take_while(|&x| x * x <= modulo)
            .all(|x| modulo % x != 0));
        Self::new(size, modulo)
    }

    /// `n` の階乗を返します。
    ///
    /// # Panics
    ///
    /// `n >= size` の場合にパニックします。
    ///
    /// # Examples
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let f = Factorial::new(10, 1000000007);
    /// assert_eq!(f.factorial(0), 1); // 0! = 1
    /// assert_eq!(f.factorial(5), 120); // 5! = 120
    /// ```
    pub fn factorial(&self, n: usize) -> u64 {
        assert!(n < self.factorial.len());
        self.factorial[n]
    }

    /// `n` の階乗の乗法逆元を返します。
    ///
    /// # Panics
    ///
    /// `n >= size` の場合にパニックします。
    ///
    /// # Examples
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let modulo = 1000000007;
    /// let f = Factorial::new(10, modulo);
    /// let n = 5;
    /// assert_eq!((f.factorial(n) * f.inversion(n)) % modulo, 1);
    /// ```
    pub fn inversion(&self, n: usize) -> u64 {
        assert!(n < self.inversion_of_factorial.len());
        self.inversion_of_factorial[n]
    }

    /// 二項係数 C(n, k) を返します。
    ///
    /// 二項係数は nCk = n! / (k! * (n-k)!) で定義され、
    /// n 個の異なる要素から k 個を選ぶ組み合わせの数を表します。
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
    /// ## 競技プログラミングでの応用例
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let modulo = 1000000007;
    /// let f = Factorial::new(200, modulo);
    ///
    /// // パスカルの三角形の性質: C(n, k) = C(n-1, k-1) + C(n-1, k)
    /// let n = 10;
    /// let k = 4;
    /// let left = if k > 0 { f.binomial(n - 1, k - 1) } else { 0 };
    /// let right = f.binomial(n - 1, k);
    /// assert_eq!(f.binomial(n, k), (left + right) % modulo);
    ///
    /// // 格子路問題: (0,0) から (m,n) への最短経路数
    /// let (m, n) = (5, 3);
    /// let paths = f.binomial(m + n, m); // または f.binomial(m + n, n)
    /// 
    /// // 重複組み合わせ: n種類から重複を許して k個選ぶ
    /// let h_n_k = f.binomial(n + k - 1, k);
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
    /// 数学的に C(n, k) = 0 when n < k という性質を利用した安全なバージョンです。
    /// ループ処理などで境界チェックを省略したい場合に便利です。
    ///
    /// # Examples
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let f = Factorial::new_checking_modulo_prime(5, 107);
    /// assert_eq!(f.binomial_or_zero(3, 4), 0); // n < k の場合
    /// assert_eq!(f.binomial_or_zero(4, 2), 6); // 通常の場合
    /// ```
    ///
    /// ## 競技プログラミングでの使用例
    ///
    /// ```
    /// use factorials::Factorial;
    ///
    /// let modulo = 1000000007;
    /// let f = Factorial::new(100, modulo);
    ///
    /// // 動的プログラミングでの安全な使用
    /// let n = 10;
    /// let mut dp = vec![0u64; n + 1];
    /// dp[0] = 1;
    ///
    /// for i in 1..=n {
    ///     for j in 0..=i {
    ///         // j > i の場合も安全に処理
    ///         dp[i] = (dp[i] + f.binomial_or_zero(i - 1, j)) % modulo;
    ///     }
    /// }
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
