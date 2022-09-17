/// 初項 `a`, 項数 `n`, 公差 `d` の等差数列の和を求めます。
///
/// # Panics
/// if `n` is negative or zero.
///
/// # Examples
/// ```
/// use arithmetic_series::arithmetic_series;
///
/// // 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10
/// assert_eq!(arithmetic_series(1, 10, 1), Some(55));
/// // 1 + 3 + 5 + 7 + 9
/// assert_eq!(arithmetic_series(1, 5, 2), Some(25));
/// // 5 + 2 + (-1) + (-4) + (-7) + (-10)
/// assert_eq!(arithmetic_series(5, 6, -3), Some(-15));
/// ```
pub fn arithmetic_series<T: Int>(a: T, n: T, d: T) -> Option<T> {
    assert!(n.is_positive());
    let last = d.checked_mul(n.decrement())?.checked_add(a)?;
    a.checked_add(last)?.checked_mul(n)?.checked_div(T::two())
}

pub trait Int: Copy + Ord {
    fn is_positive(self) -> bool;
    fn decrement(self) -> Self;
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn two() -> Self;
}

macro_rules! impl_int {
    ($($t:ty),+) => {
        $(
            impl Int for $t {
                fn is_positive(self) -> bool {
                    self >= 1
                }
                fn decrement(self) -> Self {
                    self - 1
                }
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    self.checked_add(rhs)
                }
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    self.checked_mul(rhs)
                }
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    self.checked_div(rhs)
                }
                fn two() -> Self {
                    2
                }
            }
        )+
    };
}

impl_int!(i32, i64, i128, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use crate::arithmetic_series;

    #[test]
    fn test_sum_of_1_2_3_to_10() {
        assert_eq!(arithmetic_series(1, 10, 1), Some(55));
    }

    #[test]
    fn test_single() {
        assert_eq!(arithmetic_series(42, 1, 3), Some(42));
    }

    #[test]
    fn test_decrease_sequence() {
        assert_eq!(
            arithmetic_series(8, 6, -3),
            Some(8 + 5 + 2 + (-1) + (-4) + (-7))
        );
    }

    #[test]
    fn test_too_large() {
        assert_eq!(arithmetic_series(1, std::i64::MAX, 1), None);
    }

    #[test]
    #[should_panic]
    fn test_empty() {
        arithmetic_series(42, 0, 3);
    }

    #[test]
    #[should_panic]
    fn test_negative_length() {
        arithmetic_series(42, -4, 3);
    }
}
