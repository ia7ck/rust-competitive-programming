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

use std::convert::TryInto;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use ext_gcd::ext_gcd;

pub trait Modulo: Copy + Clone + Debug {
    const P: i64;
}

#[derive(Copy, Clone, Debug)]
pub struct ModInt<M>(i64, PhantomData<M>);

impl<M: Modulo> ModInt<M> {
    /// 整数を `0 <= x < p` に正規化してインスタンスを作ります。
    pub fn new(x: i64) -> Self {
        if 0 <= x && x < M::P {
            Self::new_raw(x)
        } else {
            Self::new_raw(x.rem_euclid(M::P))
        }
    }

    fn new_raw(x: i64) -> Self {
        debug_assert!(0 <= x && x < M::P);
        Self(x, PhantomData)
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
    /// assert_eq!(ModInt1000000007::p(), 1000000007);
    /// assert_eq!(ModInt998244353::p(), 998244353);
    /// ```
    pub fn p() -> i64 {
        M::P
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
        T: TryInto<u64>,
        <T as TryInto<u64>>::Error: Debug,
    {
        let mut res = 1;
        let mut base = self.0;
        let mut exp = exp.try_into().unwrap();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
                res %= M::P;
            }
            base *= base;
            base %= M::P;
            exp >>= 1;
        }
        Self::new_raw(res)
    }

    /// `x * y % p = 1` となる `y` を返します。
    ///
    /// # Examples
    /// ```
    /// use mod_int::ModInt1000000007;
    /// let (x, p) = (2, ModInt1000000007::p());
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
    /// use mod_int::{Modulo, ModInt, define_mod_int_p};
    /// define_mod_int_p!(Mod10, ModInt10, 10);
    /// // 6 * n : 0, 6, 2, 8, 4, 0, 6, 2, 8, 4
    /// ModInt10::new(6).inv(); // panic
    /// ```
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Don't divide by zero!");
        let (x, _, g) = ext_gcd(self.0, M::P);
        assert_eq!(g, 1, "{} is not prime!", M::P);
        Self::new(x)
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into().0;
        debug_assert!(0 <= self.0 && self.0 <= (M::P - 1) * 2);
        if self.0 >= M::P {
            self.0 -= M::P;
        }
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: T) -> Self::Output {
        let mut result = self;
        result += rhs.into();
        result
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs.into().0;
        debug_assert!(-(M::P - 1) <= self.0 && self.0 < M::P);
        if self.0 < 0 {
            self.0 += M::P;
        }
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: T) -> Self::Output {
        let mut result = self;
        result -= rhs.into();
        result
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs.into().0;
        if self.0 >= M::P {
            self.0 %= M::P;
        }
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        result *= rhs.into();
        result
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> DivAssign<T> for ModInt<M> {
    fn div_assign(&mut self, rhs: T) {
        *self *= rhs.into().inv();
    }
}

impl<M: Modulo, T: Into<ModInt<M>>> Div<T> for ModInt<M> {
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
            impl<M: Modulo> From<$t> for ModInt<M> {
                fn from(x: $t) -> Self {
                    Self::new(x as i64)
                }
            }
        )+
    };
}

impl_from_int!(i32, i64, u32);

macro_rules! impl_from_large_int {
    ($($t:ty),+) => {
        $(
            impl<M: Modulo> From<$t> for ModInt<M> {
                fn from(x: $t) -> Self {
                    Self::new((x % (M::P as $t)) as i64)
                }
            }
        )+
    };
}

impl_from_large_int!(u64, usize);

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
/// assert_eq!(Mint::p(), 19);
/// assert_eq!((Mint::new(18) + Mint::new(2)).val(), 1);
/// ```
#[macro_export]
macro_rules! define_mod_int_p {
    ($mod: ident, $mod_int: ident, $p: expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $mod;
        impl Modulo for $mod {
            const P: i64 = $p;
        }
        pub type $mod_int = ModInt<$mod>;
    };
}
define_mod_int_p!(Mod1000000007, ModInt1000000007, 1_000_000_000 + 7);
define_mod_int_p!(Mod998244353, ModInt998244353, 998_244_353);

#[cfg(test)]
mod tests {
    use super::{define_mod_int_p, ModInt, Modulo};

    #[test]
    fn ops_test() {
        define_mod_int_p!(Mod19, ModInt19, 19);
        type Mint = ModInt19;
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
