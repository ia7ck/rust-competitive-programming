use floor_sqrt::floor_sqrt;

/// `n` を素因数分解する。
///
/// # Examples
/// ```
/// use prime_factorization::prime_factorization;
///
/// assert_eq!(prime_factorization(2), vec![(2, 1)]);
/// // 90 = 2 * 3 * 3 * 5
/// assert_eq!(prime_factorization(90), vec![(2, 1), (3, 2), (5, 1)]);
/// ```
pub fn prime_factorization(n: u32) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    let mut n = n;
    for k in 2..=floor_sqrt(n as u64) as u32 {
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

#[cfg(test)]
mod tests {
    use crate::prime_factorization;

    #[test]
    fn small_test() {
        assert_eq!(prime_factorization(0), vec![]);
        assert_eq!(prime_factorization(1), vec![]);
        assert_eq!(prime_factorization(2), vec![(2, 1)]);
        assert_eq!(prime_factorization(3), vec![(3, 1)]);
        assert_eq!(prime_factorization(4), vec![(2, 2)]);
    }

    #[test]
    fn test() {
        for n in 1..1000 {
            let f = prime_factorization(n);
            let mut res = 1;
            for (p, e) in f {
                res *= p.pow(e);
            }
            assert_eq!(res, n);
        }
    }
}
