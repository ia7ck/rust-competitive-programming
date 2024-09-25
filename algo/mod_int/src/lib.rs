//! `ModInt` は整数の四則演算を mod `p` で行う構造体です。
//!
//! ```
//! use mod_int::ModInt1000000007;
//! let p = 1000000007_i64;
//! let (a, b, c) = (1000000001, 1000000005, 100000006);
//! let x = (123 * a % p * b % p - c).rem_euclid(p);
//! let y = ModInt1000000007::new(123) * a * b - c;
//! assert_eq!(x, y.val());
//! ```

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use ext_gcd::ext_gcd;

#[derive(Debug, Clone, Copy)]
pub struct ModInt<const M: i64>(i64);

impl<const M: i64> ModInt<M> {
    /// 整数を `0 <= x < modulo` に正規化してインスタンスを作ります。
    pub fn new(x: i64) -> Self {
        if 0 <= x && x < M {
            Self::new_raw(x)
        } else {
            Self::new_raw(x.rem_euclid(M))
        }
    }

    fn new_raw(x: i64) -> Self {
        debug_assert!(0 <= x && x < M);
        Self(x)
    }

    /// `ModInt` に格納されている値を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// assert_eq!(ModInt1000000007::new(123).val(), 123);
    /// ```
    pub fn val(self) -> i64 {
        self.0
    }

    /// 法を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::{ModInt1000000007, ModInt998244353};
    /// assert_eq!(ModInt1000000007::modulo(), 1000000007);
    /// assert_eq!(ModInt998244353::modulo(), 998244353);
    /// ```
    pub fn modulo() -> i64 {
        M
    }

    /// 二分累乗法で `x^exp % p` を計算します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// use std::iter::repeat;
    /// let (x, exp, p) = (123, 100_u32, 1000000007);
    /// let y = repeat(x).take(exp as usize).fold(1, |acc, x| acc * x % p);
    /// assert_eq!(y, ModInt1000000007::new(x).pow(exp).val());
    /// ```
    pub fn pow(self, exp: u32) -> Self {
        let mut res = 1;
        let mut base = self.0;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
                res %= M;
            }
            base *= base;
            base %= M;
            exp >>= 1;
        }
        Self::new_raw(res)
    }

    /// `x * y % p = 1` となる `y` を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// let (x, p) = (2, ModInt1000000007::modulo());
    /// let y = ModInt1000000007::new(x).inv().val();
    /// assert_eq!(x * y % p, 1);
    /// ```
    ///
    /// ```should_panic
    /// use mod_int::ModInt1000000007;
    /// ModInt1000000007::new(0).inv(); // panic
    /// ```
    ///
    /// ```should_panic
    /// use mod_int::ModInt;
    /// // 6 * n % 10 : 0, 6, 2, 8, 4, 0, 6, 2, 8, 4
    /// ModInt::<10>::new(6).inv(); // panic
    /// ```
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Don't divide by zero!");
        let (x, _, g) = ext_gcd(self.0, M);
        assert_eq!(g, 1, "{} is not prime!", M);
        Self::new(x)
    }
}

impl<const M: i64, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into().0;
        debug_assert!(0 <= self.0 && self.0 <= (M - 1) * 2);
        if self.0 >= M {
            self.0 -= M;
        }
    }
}

impl<const M: i64, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: T) -> Self::Output {
        let mut result = self;
        result += rhs.into();
        result
    }
}

impl<const M: i64, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs.into().0;
        debug_assert!(-(M - 1) <= self.0 && self.0 < M);
        if self.0 < 0 {
            self.0 += M;
        }
    }
}

impl<const M: i64, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: T) -> Self::Output {
        let mut result = self;
        result -= rhs.into();
        result
    }
}

impl<const M: i64, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs.into().0;
        if self.0 >= M {
            self.0 %= M;
        }
    }
}

impl<const M: i64, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        result *= rhs.into();
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<const M: i64, T: Into<ModInt<M>>> DivAssign<T> for ModInt<M> {
    fn div_assign(&mut self, rhs: T) {
        *self *= rhs.into().inv();
    }
}

impl<const M: i64, T: Into<ModInt<M>>> Div<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self;
        result /= rhs.into();
        result
    }
}

macro_rules! impl_from_int {
    ($($t:ty),+) => {
        $(
            impl<const M: i64> From<$t> for ModInt<M> {
                fn from(x: $t) -> Self {
                    Self::new(i64::from(x))
                }
            }
        )+
    };
}

impl_from_int!(i8, i16, i32, i64, u8, u16, u32);

macro_rules! impl_from_large_int {
    ($($t:ty),+) => {
        $(
            impl<const M: i64> From<$t> for ModInt<M> {
                fn from(x: $t) -> Self {
                    Self::new((x % (M as $t)) as i64)
                }
            }
        )+
    };
}

impl_from_large_int!(u64, usize, isize);

pub type ModInt1000000007 = ModInt<1_000_000_007>;
pub type ModInt998244353 = ModInt<998_244_353>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops_test() {
        type Mint = ModInt<19>;
        for a in 0..50 {
            for b in 0..50 {
                // add
                assert_eq!((Mint::new(a) + Mint::new(b)).val(), (a + b) % 19);
                // add assign
                let mut sum = Mint::new(a);
                sum += b;
                assert_eq!(sum.val(), (a + b) % 19);

                // sub
                assert_eq!((Mint::new(a) - Mint::new(b)).val(), (a - b).rem_euclid(19));
                // sub assign
                let mut diff = Mint::new(a);
                diff -= b;
                assert_eq!(diff.val(), (a - b).rem_euclid(19));

                // mul
                assert_eq!((Mint::new(a) * Mint::new(b)).val(), a * b % 19);
                // mul assign
                let mut prod = Mint::new(a);
                prod *= b;
                assert_eq!(prod.val(), a * b % 19);

                if b % 19 != 0 {
                    let expect = (0..19).find(|&x| a % 19 == b * x % 19).unwrap();
                    // div
                    assert_eq!((Mint::new(a) / Mint::new(b)).val(), expect);
                    // div assign
                    let mut frac = Mint::new(a);
                    frac /= b;
                    assert_eq!(frac.val(), expect);
                }
            }
        }
    }
}
