//! `ModInt` は整数の四則演算を mod `p` で行う構造体です。
//!
//! ```
//! use mod_int::ModInt1000000007;
//! type Mint = ModInt1000000007;
//! let p = 1000000007_i64;
//! let (a, b, c) = (1000000001, 1000000005, 100000006);
//! let x = (123 * a % p * b % p - c).rem_euclid(p);
//! let y = Mint::new(123) * Mint::new(a) * Mint::new(b) - Mint::new(c);
//! assert_eq!(x, y.val());
//! ```
//!
//! ほとんどのケースでは
//!
//! - `ModInt1000000007`
//! - `ModInt998244353`
//!
//! のどちらかを使えば十分だと思います。
//!
//! それ以外の法で `ModInt` を使いたいときは `define_mod_int_p` マクロを呼んでください。
//!
//! ```
//! use mod_int::{define_mod_int_p, Modulo, ModInt};
//! define_mod_int_p!(Mod1000000009, ModInt1000000009, 1000000009);
//! assert_eq!((ModInt1000000009::new(1000000008) + ModInt1000000009::new(2)).val(), 1);
//! ```
//!

pub trait Modulo: Copy + Clone + std::fmt::Debug {
    fn p() -> i64;
}

#[derive(Copy, Clone, std::fmt::Debug)]
pub struct ModInt<M>(i64, std::marker::PhantomData<M>);

impl<M: Modulo> ModInt<M> {
    /// 整数を `0 <= x < p` に正規化してインスタンスを作ります。
    ///
    /// # Panics
    /// if the conversion to `i64` was failed.
    ///
    /// ```should_panic
    /// use mod_int::ModInt1000000007;
    /// ModInt1000000007::new(std::u64::MAX); // panic
    /// ```
    pub fn new<T>(x: T) -> Self
    where
        T: std::convert::TryInto<i64>,
        <T as std::convert::TryInto<i64>>::Error: std::fmt::Debug,
    {
        let x = x.try_into().unwrap();
        if 0 <= x && x < M::p() {
            Self::new_raw(x)
        } else {
            Self::new_raw(x.rem_euclid(M::p()))
        }
    }

    fn new_raw(x: i64) -> Self {
        debug_assert!(0 <= x && x < M::p());
        Self(x, std::marker::PhantomData)
    }

    /// `ModInt` に格納されている値 `x` を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// assert_eq!(ModInt1000000007::new(123).val(), 123);
    /// ```
    pub fn val(self) -> i64 {
        self.0
    }

    /// 法 `p` を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::{ModInt1000000007, ModInt998244353};
    /// assert_eq!(ModInt1000000007::mo(), 1000000007);
    /// assert_eq!(ModInt998244353::mo(), 998244353);
    /// ```
    pub fn mo() -> i64 {
        M::p()
    }

    /// 二分累乗法で `x^exp % p` を計算します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// use std::iter::repeat;
    /// let (x, exp, p) = (123, 100, 1000000007);
    /// let y = repeat(x).take(exp as usize).fold(1, |acc, x| acc * x % p);
    /// assert_eq!(y, ModInt1000000007::new(x).pow(exp).val());
    /// ```
    pub fn pow<T>(self, exp: T) -> Self
    where
        T: std::convert::TryInto<u64>,
        <T as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
    {
        let mut res = 1;
        let mut base = self.0;
        let mut exp = exp.try_into().unwrap();
        let mo = Self::mo();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
                res %= mo;
            }
            base *= base;
            base %= mo;
            exp >>= 1;
        }
        Self::new_raw(res)
    }

    /// `x * y % p = 1` となる `y` を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// let (x, p) = (2, 1000000007);
    /// let y =ModInt1000000007::new(x).inv().val();
    /// assert_eq!(x * y % p, 1);
    /// ```
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Don't divide by zero!");
        self.pow(Self::mo() - 2)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> std::ops::Add for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: ModInt<M>) -> Self::Output {
        let x = self.0 + rhs.0;
        debug_assert!(0 <= x && x <= (Self::mo() - 1) * 2);
        if x < Self::mo() {
            Self::new_raw(x)
        } else {
            Self::new_raw(x - Self::mo())
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> std::ops::Sub for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: ModInt<M>) -> Self::Output {
        let x = self.0 - rhs.0;
        debug_assert!(-(Self::mo() - 1) <= x && x < Self::mo());
        if x >= 0 {
            Self::new_raw(x)
        } else {
            Self::new_raw(x + Self::mo())
        }
    }
}

impl<M: Modulo> std::ops::Mul for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: ModInt<M>) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> std::ops::Div for ModInt<M> {
    type Output = ModInt<M>;
    fn div(self, rhs: ModInt<M>) -> Self::Output {
        self * rhs.inv()
    }
}

/// 好きな法の `ModInt` を定義します。
///
/// - `$mod`: `Modulo` トレイトを実装する構造体の名前になります。ユーザー側で使うことはないと思うので適当でよいです。
/// - `$mod_int`: `ModInt` 構造体の名前になります。
/// - `$p`: `ModInt` の各種演算に使われる法を指定します。割り算をする予定があるならばこの値は素数にしてください。
///
/// # Examples
/// ```
/// use mod_int::{Modulo, ModInt, define_mod_int_p};
/// define_mod_int_p!(Mod19, ModInt19, 19);
/// type Mint = ModInt19;
/// assert_eq!(Mint::mo(), 19);
/// assert_eq!((Mint::new(18) + Mint::new(2)).val(), 1);
/// ```
#[macro_export]
macro_rules! define_mod_int_p {
    ($mod: ident, $mod_int: ident, $p: expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $mod;
        impl Modulo for $mod {
            fn p() -> i64 {
                $p
            }
        }
        pub type $mod_int = ModInt<$mod>;
    };
}
define_mod_int_p!(Mod1000000007, ModInt1000000007, 1_000_000_000 + 7);
define_mod_int_p!(Mod998244353, ModInt998244353, 998_244_353);

#[cfg(test)]
mod tests {
    use super::{ModInt, Modulo};
    #[test]
    fn ops_test() {
        define_mod_int_p!(Mod19, ModInt19, 19);
        type Mint = ModInt19;
        for a in 0..50 {
            for b in 0..50 {
                let sum = Mint::new(a) + Mint::new(b);
                assert_eq!(sum.val(), (a + b) % 19);
                let diff = Mint::new(a) - Mint::new(b);
                assert_eq!(diff.val(), (a - b).rem_euclid(19));
                let prod = Mint::new(a) * Mint::new(b);
                assert_eq!(prod.val(), a * b % 19);
                if b % 19 != 0 {
                    let frac = Mint::new(a) / Mint::new(b);
                    let expect = (0..19).find(|&x| a % 19 == b * x % 19).unwrap();
                    assert_eq!(frac.val(), expect);
                }
            }
        }
    }
}
