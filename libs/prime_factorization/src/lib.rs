use least_prime_factors::least_prime_factors;

/// 素因数分解
pub trait PrimeFactorization<T> {
    /// x の (素因数, べき) のベクタを返します
    fn factors(&self, x: T) -> Vec<(T, u32)>;
}

/// 試し割りによる素因数分解
#[derive(Debug, Clone)]
pub struct TrialDivision;

impl TrialDivision {
    pub fn new() -> Self {
        Self {}
    }
}

macro_rules! impl_prime_factorization {
    ($($t:ty),+) => {
        $(
            impl PrimeFactorization<$t> for TrialDivision {
                /// O(sqrt(x)) time
                fn factors(&self, x: $t) -> Vec<($t, u32)> {
                    let mut p_exp = Vec::new();
                    let mut y = x;
                    for p in 2.. {
                        // p * p > x
                        if p > x / p {
                            break;
                        }
                        let mut exp = 0;
                        while y % p == 0 {
                            exp += 1;
                            y /= p;
                        }
                        if exp > 0 {
                            p_exp.push((p, exp));
                        }
                    }
                    if y > 1 {
                        p_exp.push((y, 1));
                    }
                    p_exp
                }
            }
        )+
    };
}

impl_prime_factorization!(usize, u32, u64);

/// least prime factors による素因数分解
#[derive(Debug, Clone)]
pub struct ByLeastPrimeFactors {
    lpf: Vec<usize>,
}

impl ByLeastPrimeFactors {
    /// 素因数分解の前計算として [least prime factors](least_prime_factors::least_prime_factors) を求めます。
    pub fn new(n: usize) -> Self {
        let lpf = least_prime_factors(n);
        Self { lpf }
    }
}

impl PrimeFactorization<usize> for ByLeastPrimeFactors {
    /// O(log(x)) time
    fn factors(&self, x: usize) -> Vec<(usize, u32)> {
        assert!(x < self.lpf.len());
        let mut p_exp = Vec::new();
        let mut x = x;
        while x > 1 {
            let p = self.lpf[x];
            let mut exp = 0;
            while x % p == 0 {
                exp += 1;
                x /= p;
            }
            p_exp.push((p, exp));
        }
        p_exp
    }
}

#[cfg(test)]
mod tests {
    use crate::{ByLeastPrimeFactors, PrimeFactorization, TrialDivision};

    #[test]
    fn small_trial_division() {
        let trial_div = TrialDivision::new();
        assert_eq!(trial_div.factors(0_u32), vec![]);
        assert_eq!(trial_div.factors(1_u32), vec![]);
        assert_eq!(trial_div.factors(2_u32), vec![(2, 1)]);
        assert_eq!(trial_div.factors(3_u32), vec![(3, 1)]);
        assert_eq!(trial_div.factors(4_u32), vec![(2, 2)]);
    }

    #[test]
    fn small_least_prime_factors() {
        let lpf = ByLeastPrimeFactors::new(10);
        assert_eq!(lpf.factors(0_usize), vec![]);
        assert_eq!(lpf.factors(1_usize), vec![]);
        assert_eq!(lpf.factors(2_usize), vec![(2, 1)]);
        assert_eq!(lpf.factors(3_usize), vec![(3, 1)]);
        assert_eq!(lpf.factors(4_usize), vec![(2, 2)]);
    }

    #[test]
    fn test_trial_division() {
        let trial_div = TrialDivision::new();
        for n in 1_u32..=1000 {
            let mut res = 1;
            for (p, e) in trial_div.factors(n) {
                res *= p.pow(e);
            }
            assert_eq!(res, n);
        }
    }

    #[test]
    fn test_least_prime_factors() {
        let lpf = ByLeastPrimeFactors::new(1000);
        for n in 1_usize..=1000 {
            let mut res = 1;
            for (p, e) in lpf.factors(n) {
                res *= p.pow(e);
            }
            assert_eq!(res, n);
        }
    }
}
