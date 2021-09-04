/// g = gcd(a, b), ax + by = g を満たす (x, y, g) を返します。
///
/// # Examples
/// ```
/// use ext_gcd::ext_gcd;
///
/// let (x, y, g) = ext_gcd(48, 30);
/// assert_eq!(g, 6);
/// assert_eq!(48 * x + 30 * y, g); // e.g. x = 2, y = -3
///
/// assert_eq!(ext_gcd(42, 0), (1, 0, 42));
/// assert_eq!(ext_gcd(0, 0), (0, 0, 0));
/// ```
#[allow(clippy::many_single_char_names)]
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        // ax + 0y = a
        if a == 0 {
            (0, 0, 0)
        } else {
            (1, 0, a)
        }
    } else {
        let (q, r) = (a / b, a % b);
        // a = bq + r, ax + by = g
        // -> b * (qx + y) + rx = g
        let (s, t, g) = ext_gcd(b, r);
        // s = qx + y
        // t = x
        (t, s - q * t, g)
    }
}

#[cfg(test)]
mod tests {
    use crate::ext_gcd;

    #[test]
    fn test() {
        for a in -20..=20 {
            for b in -20..=20 {
                let expected_g = gcd(a, b);
                let (x, y, g) = ext_gcd(a, b);
                assert_eq!(expected_g, g.abs());
                assert_eq!(a * x + b * y, g);
            }
        }
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if a == 0 && b == 0 {
            return 0;
        }
        (1..=(a.abs().max(b.abs())))
            .filter(|d| a % d == 0 && b % d == 0)
            .max()
            .unwrap()
    }
}
