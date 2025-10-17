/// `floor(sqrt(n))` を返す。
///
/// # Examples
/// ```
/// use floor_sqrt::floor_sqrt;
///
/// assert_eq!(floor_sqrt(0), 0);
/// assert_eq!(floor_sqrt(1), 1);
/// assert_eq!(floor_sqrt(2), 1);
/// assert_eq!(floor_sqrt(3), 1);
/// assert_eq!(floor_sqrt(4), 2);
/// assert_eq!(floor_sqrt(5), 2);
/// ```
pub fn floor_sqrt(n: u64) -> u64 {
    let mut ok = 0;
    let mut ng = u64::from(u32::MAX);
    while ng - ok > 1 {
        let m = (ng + ok) / 2;
        if m * m <= n {
            ok = m;
        } else {
            ng = m;
        }
    }
    assert!(ok * ok <= n);
    assert!((ok + 1) * (ok + 1) > n);
    ok
}

#[cfg(test)]
mod tests {
    use crate::floor_sqrt;

    #[test]
    fn test() {
        assert_eq!(floor_sqrt(0), 0);
        assert_eq!(floor_sqrt(1), 1);
        assert_eq!(floor_sqrt(2), 1);
        assert_eq!(floor_sqrt(3), 1);
        assert_eq!(floor_sqrt(4), 2);
        assert_eq!(floor_sqrt(5), 2);
    }
}
