/// 非負整数 `n` の全ての約数を返します。
///
/// - 約数は昇順に並んでいることが保証されます。
/// - `0` を渡すと空のベクタ `vec![]` を返します。
///
/// # Examples
/// ```
/// use divisors::divisors;
///
/// assert_eq!(divisors(24), vec![1, 2, 3, 4, 6, 8, 12, 24]);
/// ```
pub fn divisors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut large = vec![];
    for k in (1..).take_while(|k| k * k <= n) {
        if n % k == 0 {
            res.push(k);
            if n / k != k {
                large.push(n / k);
            }
        }
    }
    large.reverse();
    res.append(&mut large);
    res
}

#[cfg(test)]
mod tests {
    use crate::divisors;

    #[test]
    fn divisors_test() {
        assert_eq!(divisors(0), vec![]);
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(2), vec![1, 2]);
        assert_eq!(divisors(24), vec![1, 2, 3, 4, 6, 8, 12, 24]);
        assert_eq!(divisors(25), vec![1, 5, 25]);
        assert_eq!(divisors(29), vec![1, 29]);
    }
}
