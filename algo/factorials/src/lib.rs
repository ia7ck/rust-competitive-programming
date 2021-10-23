/// `1` 以上 `size` 未満の `n` について、`n` の階乗 (mod `modulo`) と、その乗法逆元を O(`size`) で計算します。[参考](https://drken1215.hatenablog.com/entry/2018/06/08/210000)
///
/// 逆元を正しく計算するためには
///
/// - `modulo` が素数
/// - `modulo >= size`
///
/// である必要があります。
///
/// # Examples
/// ```
/// use factorials::factorials;
/// let p = 1_000_000_000 + 7;
/// let (fac, fac_inv) = factorials(100, p);
/// for i in 1..100 {
///     assert_eq!(fac[i] * fac_inv[i] % p, 1);
/// }
/// ```
pub fn factorials(size: usize, modulo: u64) -> (Vec<u64>, Vec<u64>) {
    let mut fac = vec![0; size];
    let mut inv = vec![0; size];
    let mut inv_fac = vec![0; size];
    fac[0] = 1;
    fac[1] = 1;
    inv[1] = 1;
    inv_fac[0] = 1;
    inv_fac[1] = 1;
    for i in 2..size {
        let i_u64 = i as u64;
        fac[i] = fac[i - 1] * i_u64 % modulo;
        inv[i] = ((modulo - inv[(modulo as usize) % i]) * (modulo / i_u64)).rem_euclid(modulo);
        inv_fac[i] = inv_fac[i - 1] * inv[i] % modulo;
    }
    (fac, inv_fac)
}

#[cfg(test)]
mod tests {
    use super::factorials;
    #[test]
    fn test_mod_is_103() {
        let p = 103;
        let (fac, fac_inv) = factorials(100, p);
        for i in 1..100 {
            assert_eq!(fac[i] * fac_inv[i] % p, 1);
        }
    }
}
