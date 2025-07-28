//! 拡張ユークリッド互除法（Extended Euclidean Algorithm）のライブラリです。
//!
//! 2つの整数 a, b に対して、ベズー恒等式 ax + by = gcd(a, b) を満たす
//! 整数解 (x, y) と最大公約数 gcd(a, b) を同時に求めます。
//!
//! # 計算量
//!
//! - 時間計算量: O(log(min(|a|, |b|)))
//! - 空間計算量: O(log(min(|a|, |b|))) （再帰呼び出しによる）
//!
//! # 用途
//!
//! - 乗法逆元の計算（mod p での a^(-1) の計算）
//! - 一次不定方程式 ax + by = c の解の計算
//! - 中国剰余定理の実装
//! - 競技プログラミングでの整数問題の解決
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use ext_gcd::ext_gcd;
//!
//! // 48x + 30y = gcd(48, 30) = 6 の解
//! let (x, y, g) = ext_gcd(48, 30);
//! assert_eq!(g, 6);
//! assert_eq!(48 * x + 30 * y, g);
//! ```
//!
//! ## 乗法逆元の計算
//!
//! ```
//! use ext_gcd::ext_gcd;
//!
//! // mod 1000000007 での 123 の逆元を求める
//! let a = 123;
//! let p = 1000000007; // 素数
//! let (x, _y, g) = ext_gcd(a, p);
//! 
//! assert_eq!(g, 1); // gcd(a, p) = 1 なので逆元が存在
//! let inv = x.rem_euclid(p);
//! assert_eq!((a * inv) % p, 1); // a * inv ≡ 1 (mod p)
//! ```
//!
//! ## 一次不定方程式の解
//!
//! ```
//! use ext_gcd::ext_gcd;
//!
//! // 方程式 15x + 25y = 5 の解を求める
//! let (a, b, c) = (15, 25, 5);
//! let (x0, y0, g) = ext_gcd(a, b);
//! 
//! assert_eq!(g, 5); // gcd(15, 25) = 5
//! assert!(c % g == 0); // c が g の倍数なので解が存在
//! 
//! // 特解を計算
//! let k = c / g;
//! let (x, y) = (x0 * k, y0 * k);
//! assert_eq!(a * x + b * y, c);
//! ```

/// 拡張ユークリッド互除法を実行します。
/// 
/// ベズー恒等式 ax + by = gcd(a, b) を満たす整数解 (x, y) と
/// 最大公約数 gcd(a, b) を計算します。
///
/// # 引数
///
/// - `a`, `b`: 任意の整数（負数も可能）
///
/// # 戻り値
///
/// `(x, y, g)` のタプルを返します：
/// - `x`, `y`: ベズー恒等式 ax + by = g を満たす整数
/// - `g`: a と b の最大公約数（常に非負）
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
///
/// # 競技プログラミングでの応用例
///
/// ## ModInt での逆元計算
/// ```
/// use ext_gcd::ext_gcd;
///
/// fn mod_inverse(a: i64, m: i64) -> Option<i64> {
///     let (x, _y, g) = ext_gcd(a, m);
///     if g == 1 {
///         Some(x.rem_euclid(m))
///     } else {
///         None // 逆元が存在しない
///     }
/// }
///
/// let p = 1000000007;
/// let a = 123456;
/// if let Some(inv) = mod_inverse(a, p) {
///     assert_eq!((a * inv) % p, 1);
/// }
/// ```
///
/// ## 中国剰余定理での使用
/// ```
/// use ext_gcd::ext_gcd;
///
/// fn chinese_remainder_theorem(r1: i64, m1: i64, r2: i64, m2: i64) -> Option<(i64, i64)> {
///     let (x, _y, g) = ext_gcd(m1, m2);
///     if (r2 - r1) % g != 0 {
///         return None; // 解が存在しない
///     }
///     
///     let lcm = m1 / g * m2;
///     let result = (r1 + m1 * (r2 - r1) / g * x).rem_euclid(lcm);
///     Some((result, lcm))
/// }
///
/// // x ≡ 2 (mod 3) かつ x ≡ 3 (mod 5) の解
/// if let Some((x, _m)) = chinese_remainder_theorem(2, 3, 3, 5) {
///     assert_eq!(x % 3, 2);
///     assert_eq!(x % 5, 3);
/// }
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
