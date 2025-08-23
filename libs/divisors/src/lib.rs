/// 非負整数の約数全体です。
pub trait Divisors: Sized {
    /// 非負整数の約数を昇順で返します。`0` に対しては空のベクタ `vec![]` を返します。
    ///
    /// # Examples
    /// ```
    /// use divisors::Divisors;
    ///
    /// assert_eq!(24_u32.divisors(), vec![1, 2, 3, 4, 6, 8, 12, 24]);
    fn divisors(self) -> Vec<Self>;
}

macro_rules! impl_divisors {
    ($($t:ty),+) => {
        $(
            impl Divisors for $t {
                fn divisors(self) -> Vec<Self> {
                    let mut res = vec![];
                    let mut large = vec![];
                    for k in ((1 as Self)..).take_while(|&k| k.saturating_mul(k) <= self) {
                        if self % k == 0 {
                            res.push(k);
                            if self / k != k {
                                large.push(self / k);
                            }
                        }
                    }
                    large.reverse();
                    res.append(&mut large);
                    res
                }
            }
        )+
    };
}

impl_divisors!(usize, u32, u64);

#[cfg(test)]
mod tests {
    use crate::Divisors;

    #[test]
    fn divisors_test() {
        assert_eq!(0_u32.divisors(), vec![]);
        assert_eq!(1_u32.divisors(), vec![1]);
        assert_eq!(2_u32.divisors(), vec![1, 2]);
        assert_eq!(24_u32.divisors(), vec![1, 2, 3, 4, 6, 8, 12, 24]);
        assert_eq!(25_u32.divisors(), vec![1, 5, 25]);
        assert_eq!(29_u32.divisors(), vec![1, 29]);
    }
}
