/// 非負整数を素因数分解です。
pub trait PrimeFactorization: Sized {
    /// (素因数, べき) のベクタを返します。
    ///
    /// # Examples
    /// ```
    /// use prime_factorization::PrimeFactorization;
    ///
    /// assert_eq!(2_u32.prime_factorization(), vec![(2, 1)]);
    /// // 90 = 2 * 3 * 3 * 5
    /// assert_eq!(90_u32.prime_factorization(), vec![(2, 1), (3, 2), (5, 1)]);
    /// ```
    fn prime_factorization(self) -> Vec<(Self, Self)>;
}

macro_rules! impl_prime_factorization {
    ($($t:ty),+) => {
        $(
            impl PrimeFactorization for $t {
                fn prime_factorization(self) -> Vec<(Self, Self)> {
                    let mut res = Vec::new();
                    let mut n = self;
                    for k in ((2 as Self)..).take_while(|&k| k.saturating_mul(k) <= self) {
                        if n % k == 0 {
                            let mut e = 0;
                            while n % k == 0 {
                                e += 1;
                                n /= k;
                            }
                            res.push((k, e));
                        }
                    }
                    if n > 1 {
                        res.push((n, 1));
                    }
                    res
                }
            }
        )+
    };
}

impl_prime_factorization!(usize, u32, u64);

#[cfg(test)]
mod tests {
    use crate::PrimeFactorization;

    #[test]
    fn small_test() {
        assert_eq!(0_u32.prime_factorization(), vec![]);
        assert_eq!(1_u32.prime_factorization(), vec![]);
        assert_eq!(2_u32.prime_factorization(), vec![(2, 1)]);
        assert_eq!(3_u32.prime_factorization(), vec![(3, 1)]);
        assert_eq!(4_u32.prime_factorization(), vec![(2, 2)]);
    }

    #[test]
    fn test() {
        for n in 1..1000 {
            let f = (n as u32).prime_factorization();
            let mut res = 1;
            for (p, e) in f {
                res *= p.pow(e);
            }
            assert_eq!(res, n);
        }
    }
}
