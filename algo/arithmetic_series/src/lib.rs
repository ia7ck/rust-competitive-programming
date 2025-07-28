//! 等差数列の和を効率的に計算するライブラリです。
//!
//! 等差数列 a, a+d, a+2d, ..., a+(n-1)d の和を公式を使って O(1) で計算します。
//! オーバーフローチェック機能付きで、安全に大きな数値を扱えます。
//!
//! # 数学的背景
//!
//! 等差数列の和の公式：
//! ```text
//! S = n/2 * (2a + (n-1)d) = n/2 * (初項 + 末項)
//! ```
//!
//! # 計算量
//!
//! - 時間計算量: O(1)
//! - 空間計算量: O(1)
//!
//! # 用途
//!
//! - 等差数列の和の高速計算
//! - 数学的問題の解決
//! - 競技プログラミングでの数列問題
//! - 繰り返し処理の最適化
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use arithmetic_series::arithmetic_series;
//!
//! // 1 + 2 + 3 + ... + 10
//! assert_eq!(arithmetic_series(1, 10, 1), Some(55));
//!
//! // 2 + 4 + 6 + 8 + 10 (偶数の和)
//! assert_eq!(arithmetic_series(2, 5, 2), Some(30));
//!
//! // 5 + 2 + (-1) + (-4) (減少数列)
//! assert_eq!(arithmetic_series(5, 4, -3), Some(2));
//! ```
//!
//! ## 競技プログラミングでの応用例
//!
//! ```
//! use arithmetic_series::arithmetic_series;
//!
//! // n(n+1)/2 の公式と同等
//! fn triangular_number(n: i64) -> Option<i64> {
//!     arithmetic_series(1, n, 1)
//! }
//!
//! // 平方数の和 1² + 2² + ... + n² = n(n+1)(2n+1)/6
//! fn sum_of_squares_formula(n: i64) -> Option<i64> {
//!     n.checked_mul(n + 1)?
//!         .checked_mul(2 * n + 1)?
//!         .checked_div(6)
//! }
//!
//! assert_eq!(triangular_number(100), Some(5050));
//! assert_eq!(sum_of_squares_formula(3), Some(14)); // 1 + 4 + 9
//! ```

/// 初項 `a`, 項数 `n`, 公差 `d` の等差数列の和を求めます。
/// 等差数列の和を計算します。
///
/// 初項 `a`, 項数 `n`, 公差 `d` の等差数列の和を公式 `n/2 * (2a + (n-1)d)` を使って計算します。
/// 計算途中でオーバーフローが発生した場合は `None` を返します。
///
/// # 引数
///
/// - `a`: 等差数列の初項
/// - `n`: 項数（0以上である必要があります）
/// - `d`: 公差
///
/// # 戻り値
///
/// - `Some(sum)`: 計算が成功した場合の和
/// - `None`: オーバーフローが発生した場合
///
/// # Panics
/// 
/// `n` が負の場合にパニックします。
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
/// 
/// // 空の数列
/// assert_eq!(arithmetic_series(42, 0, 3), Some(0));
/// 
/// // オーバーフローの例
/// assert_eq!(arithmetic_series(1, std::i64::MAX, 1), None);
/// ```
///
/// ## 数学的応用例
///
/// ```
/// use arithmetic_series::arithmetic_series;
///
/// // ガウスの公式: 1 + 2 + ... + n = n(n+1)/2
/// fn gauss_sum(n: i64) -> Option<i64> {
///     arithmetic_series(1, n, 1)
/// }
///
/// // 奇数の和: 1 + 3 + 5 + ... + (2n-1) = n²
/// fn odd_sum(n: i64) -> Option<i64> {
///     arithmetic_series(1, n, 2)
/// }
///
/// // 偶数の和: 2 + 4 + 6 + ... + 2n = n(n+1)
/// fn even_sum(n: i64) -> Option<i64> {
///     arithmetic_series(2, n, 2)
/// }
///
/// assert_eq!(gauss_sum(100), Some(5050));
/// assert_eq!(odd_sum(5), Some(25));    // 1+3+5+7+9 = 25 = 5²
/// assert_eq!(even_sum(4), Some(20));   // 2+4+6+8 = 20 = 4×5
/// ```
///
/// ## 競技プログラミングでの実用例
///
/// ```
/// use arithmetic_series::arithmetic_series;
///
/// // 等差数列の部分和（l番目からr番目まで）
/// fn arithmetic_range_sum(a: i64, d: i64, l: i64, r: i64) -> Option<i64> {
///     if l > r { return Some(0); }
///     
///     let full_sum = arithmetic_series(a, r, d)?;
///     if l <= 1 {
///         Some(full_sum)
///     } else {
///         let prefix_sum = arithmetic_series(a, l - 1, d)?;
///         full_sum.checked_sub(prefix_sum)
///     }
/// }
///
/// // テスト: 数列 [3, 5, 7, 9, 11] の 2番目から4番目まで
/// // = 5 + 7 + 9 = 21
/// assert_eq!(arithmetic_range_sum(3, 2, 2, 4), Some(21));
/// ```
pub fn arithmetic_series<T: Int>(a: T, n: T, d: T) -> Option<T> {
    if n == T::zero() {
        return Some(T::zero());
    }

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
    fn zero() -> Self;
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
                fn zero() -> Self {
                    0
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
    fn test_empty() {
        assert_eq!(arithmetic_series(42, 0, 3), Some(0));
    }

    #[test]
    fn test_too_large() {
        assert_eq!(arithmetic_series(1, std::i64::MAX, 1), None);
    }

    #[test]
    #[should_panic]
    fn test_negative_length() {
        arithmetic_series(42, -4, 3);
    }
}
