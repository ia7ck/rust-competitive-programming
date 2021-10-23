/// 「`k` を割る最小の素数」をエラトステネスのふるいの要領で `2` 以上 `n` 未満の全ての `k` について計算します。[参考](https://osak.jp/diary/diary_201310.html#20131017)
///
/// # Examples
/// ```
/// use least_prime_factors::least_prime_factors;
/// let facs = least_prime_factors(10);
/// assert_eq!(facs[2], 2);
/// assert_eq!(facs[3], 3);
/// assert_eq!(facs[4], 2);
/// assert_eq!(facs[5], 5);
/// assert_eq!(facs[6], 2);
/// assert_eq!(facs[7], 7);
/// assert_eq!(facs[8], 2);
/// assert_eq!(facs[9], 3);
/// ```
pub fn least_prime_factors(n: usize) -> Vec<usize> {
    let mut result = vec![0; n];
    #[allow(clippy::needless_range_loop)]
    for i in 2..n {
        result[i] = i;
    }
    for i in 2..n {
        if result[i] == i {
            for j in ((i + i)..n).step_by(i) {
                if result[j] == j {
                    result[j] = i;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::least_prime_factors;

    #[test]
    fn min_factors_test() {
        let n = 1000;
        let min_factors = least_prime_factors(n);
        for i in 2..n {
            let j = (2..=i).find(|&j| i % j == 0).unwrap();
            assert_eq!(j, min_factors[i]);
        }
    }
}
