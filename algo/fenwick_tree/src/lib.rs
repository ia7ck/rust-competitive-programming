//! Fenwick Tree（Binary Indexed Tree, BIT）は一点更新と区間和クエリを効率的に行うデータ構造です。
//!
//! セグメントツリーより実装が簡単で、特に区間和や区間XORなど群の演算に対して
//! 高速に動作します。
//!
//! ## 特徴
//!
//! - **時間計算量**: 
//!   - 一点更新: O(log n)
//!   - 区間和クエリ: O(log n)
//!   - 構築: O(n log n) (各要素を個別に追加する場合)
//! - **空間計算量**: O(n)
//! - **実装の簡潔さ**: セグメントツリーより短く書ける
//! - **制約**: 逆元が存在する演算（群）にのみ対応
//!
//! ## 主な用途
//!
//! - 区間和クエリ（Range Sum Query）
//! - 区間XORクエリ
//! - 配列の要素の増減操作
//! - 転倒数の計算
//! - 座標圧縮と組み合わせた集計処理
//!
//! ## 基本的な使用例
//!
//! ```
//! use fenwick_tree::FenwickTree;
//!
//! let mut ft = FenwickTree::new(5, 0);
//! ft.add(0, 1);   // インデックス0に1を加算
//! ft.add(2, 10);  // インデックス2に10を加算
//! ft.add(4, 100); // インデックス4に100を加算
//! // 配列: [1, 0, 10, 0, 100]
//!
//! assert_eq!(ft.sum(0..1), 1);   // [0,1)の和: 1
//! assert_eq!(ft.sum(0..3), 11);  // [0,3)の和: 1 + 0 + 10 = 11
//! assert_eq!(ft.sum(2..5), 110); // [2,5)の和: 10 + 0 + 100 = 110
//! assert_eq!(ft.sum(..), 111);   // 全体の和: 111
//!
//! // 要素の増減
//! ft.add(1, 5);   // インデックス1に5を加算
//! assert_eq!(ft.sum(0..3), 16); // 1 + 5 + 10 = 16
//!
//! ft.add(2, -3);  // インデックス2から3を減算
//! assert_eq!(ft.sum(2..3), 7);  // 10 - 3 = 7
//! ```
//!
//! ## 参考資料
//!
//! - [Binary Indexed Tree のしくみ](http://hos.ac/slides/20140319_bit.pdf)

use std::ops::{Bound, RangeBounds};

/// Fenwick Tree（Binary Indexed Tree）の実装です。
///
/// 一点更新と区間和クエリを効率的に行うことができます。
/// 群の演算（逆元が存在する演算）に対応しています。
///
/// # Examples
/// ```
/// use fenwick_tree::FenwickTree;
/// let mut ft = FenwickTree::new(5, 0);
/// ft.add(0, 1);
/// ft.add(2, 10);
/// ft.add(4, 100);
/// // [1, 0, 10, 0, 100]
/// assert_eq!(ft.sum(0..1), 1);
/// assert_eq!(ft.sum(0..2), 1);
/// assert_eq!(ft.sum(0..3), 11);
/// assert_eq!(ft.sum(2..4), 10);
/// assert_eq!(ft.sum(2..5), 110);
/// assert_eq!(ft.sum(0..5), 111);
/// ```
#[derive(Clone, Debug)]
pub struct FenwickTree<T> {
    n: usize,
    e: T,
    dat: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy,
    T: std::ops::AddAssign,
    T: std::ops::SubAssign,
{
    /// 長さ `n` のFenwick Treeを単位元 `e` で初期化します。
    ///
    /// すべての要素が `e` で初期化されます。
    ///
    /// 時間計算量: O(n)
    /// 空間計算量: O(n)
    ///
    /// # Examples
    /// ```
    /// use fenwick_tree::FenwickTree;
    /// 
    /// // 整数の和用
    /// let ft_sum = FenwickTree::new(10, 0);
    /// 
    /// // 浮動小数点の和用
    /// let ft_float = FenwickTree::new(10, 0.0);
    /// ```
    pub fn new(n: usize, e: T) -> Self {
        Self {
            n,
            e,
            dat: vec![e; n + 1],
        }
    }

    /// インデックス `k` の要素に `x` を加算します。
    ///
    /// 0-indexedで指定したインデックスの要素に値を加算します。
    ///
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use fenwick_tree::FenwickTree;
    /// let mut ft = FenwickTree::new(5, 0);
    /// ft.add(2, 10);  // インデックス2に10を加算
    /// ft.add(2, 5);   // インデックス2にさらに5を加算
    /// assert_eq!(ft.sum(2..3), 15); // インデックス2の値は15
    /// ```
    pub fn add(&mut self, k: usize, x: T) {
        assert!(k < self.n);
        let mut k = k + 1;
        while k <= self.n {
            self.dat[k] += x;
            k += 1 << k.trailing_zeros();
        }
    }
    /// 1-indexedでの累積和を計算します（内部用）。
    ///
    /// インデックス1からrまでの要素の和を返します。
    /// BITの内部実装で使用される関数です。
    fn _sum(&self, r: usize) -> T {
        assert!(r <= self.n);
        let mut result = self.e;
        let mut k = r;
        while k >= 1 {
            result += self.dat[k];
            k -= 1 << k.trailing_zeros();
        }
        result
    }
    /// 指定した範囲の要素の和を計算します。
    ///
    /// 0-indexedで範囲を指定し、その範囲の要素の和を返します。
    /// 範囲指定にはRustの標準的な範囲記法が使用できます。
    ///
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use fenwick_tree::FenwickTree;
    /// let mut ft = FenwickTree::new(5, 0);
    /// ft.add(1, 2);
    /// ft.add(2, 3);
    /// ft.add(3, 5);
    /// 
    /// assert_eq!(ft.sum(1..4), 10);  // インデックス[1,4)の和: 2 + 3 + 5 = 10
    /// assert_eq!(ft.sum(1..=3), 10); // インデックス[1,3]の和: 2 + 3 + 5 = 10
    /// assert_eq!(ft.sum(2..4), 8);   // インデックス[2,4)の和: 3 + 5 = 8
    /// assert_eq!(ft.sum(..), 10);    // 全体の和: 10
    /// assert_eq!(ft.sum(0..0), 0);   // 空の範囲: 0（単位元）
    /// ```
    pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.n,
        };
        assert!(end <= self.n);
        let mut result = self._sum(end);
        result -= self._sum(start);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;
    use rand::prelude::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        for n in 1..=20 {
            let mut a = vec![0; n];
            let mut ft = FenwickTree::new(n, 0);
            for _ in 0..100 {
                let i = rng.gen_range(0, n);
                let x = rng.gen_range(-100, 100);
                a[i] += x;
                ft.add(i, x);
                for (l, r) in (0..n).zip(1..=n) {
                    if l <= r {
                        assert_eq!(a[l..r].iter().sum::<i32>(), ft.sum(l..r))
                    }
                }
            }
        }
    }

    #[test]
    fn test_single() {
        let mut f = FenwickTree::new(1, 0);
        f.add(0, 123);
        assert_eq!(f.sum(0..1), 123);
    }
}
