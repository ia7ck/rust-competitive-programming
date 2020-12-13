use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

pub trait Modulo: Copy + Clone + Debug {
    fn p() -> i64;
}

#[derive(Copy, Clone, Debug)]
pub struct ModInt<M>(i64, PhantomData<M>);

impl<M: Modulo> ModInt<M> {
    pub fn new(x: i64) -> Self {
        Self(x.rem_euclid(M::p()), PhantomData)
    }
    pub fn val(self) -> i64 {
        self.0
    }
    pub fn mo(self) -> i64 {
        M::p()
    }
    pub fn pow(self, exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = self;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                res = res * base;
            }
            base = base * base;
            exp >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Don't divide by zero!");
        self.pow(M::p() as u64 - 2)
    }
    pub fn new_frac(numer: i64, denom: i64) -> Self {
        Self::new(numer) / Self::new(denom)
    }
}

impl<M: Modulo> Add for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: ModInt<M>) -> Self::Output {
        Self((self.0 + rhs.0) % M::p(), PhantomData)
    }
}

impl<M: Modulo> Sub for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: ModInt<M>) -> Self::Output {
        Self((self.0 - rhs.0).rem_euclid(M::p()), PhantomData)
    }
}

impl<M: Modulo> Mul for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: ModInt<M>) -> Self::Output {
        Self((self.0 * rhs.0) % M::p(), PhantomData)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> Div for ModInt<M> {
    type Output = ModInt<M>;
    fn div(self, rhs: ModInt<M>) -> Self::Output {
        self * rhs.inv()
    }
}

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
