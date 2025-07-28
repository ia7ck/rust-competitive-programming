//! 二次元累積和（2D Cumulative Sum）のライブラリです。
//!
//! 二次元配列に対して効率的な範囲和クエリを提供します。
//! 前計算により、任意の矩形領域の合計を O(1) で計算できます。
//!
//! # 計算量
//!
//! - 前計算: O(H × W)
//! - 各クエリ: O(1)
//!
//! ここで H は行数、W は列数です。
//!
//! # 用途
//!
//! - 2D配列での矩形範囲の合計計算
//! - 画像処理でのフィルタ計算
//! - 競技プログラミングでの2次元クエリ問題
//! - 動的プログラミングの最適化
//!
//! # アルゴリズム
//!
//! 二次元累積和の構築と包除原理を使った範囲和計算：
//! ```text
//! sum(r1..r2, c1..c2) = S[r2-1][c2-1] - S[r1-1][c2-1] - S[r2-1][c1-1] + S[r1-1][c1-1]
//! ```
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use cumulative_sum_2d::CumulativeSum2D;
//!
//! let grid = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//! let cum_sum = CumulativeSum2D::new(&grid);
//!
//! // 全体の合計
//! assert_eq!(cum_sum.sum(0..3, 0..3), 45);
//!
//! // 部分矩形の合計
//! assert_eq!(cum_sum.sum(1..3, 1..3), 28); // 5+6+8+9
//! assert_eq!(cum_sum.sum(0..2, 0..2), 12); // 1+2+4+5
//! ```
//!
//! ## 競技プログラミングでの応用例
//!
//! ```
//! use cumulative_sum_2d::CumulativeSum2D;
//!
//! // 最大部分矩形問題での使用例
//! fn max_submatrix_sum(grid: &[Vec<i32>]) -> i32 {
//!     let h = grid.len();
//!     let w = grid[0].len();
//!     let cum_sum = CumulativeSum2D::new(grid);
//!     
//!     let mut max_sum = i32::MIN;
//!     for r1 in 0..h {
//!         for r2 in r1+1..=h {
//!             for c1 in 0..w {
//!                 for c2 in c1+1..=w {
//!                     let sum = cum_sum.sum(r1..r2, c1..c2);
//!                     max_sum = max_sum.max(sum);
//!                 }
//!             }
//!         }
//!     }
//!     max_sum
//! }
//!
//! let grid = vec![
//!     vec![1, -2, 3],
//!     vec![-4, 5, -6],
//!     vec![7, -8, 9],
//! ];
//! let max_sum = max_submatrix_sum(&grid);
//! assert_eq!(max_sum, 9); // 右下の単一要素
//! ```

use std::ops::{Add, Range, Sub};

/// 二次元累積和です。
///
/// 二次元配列に対して効率的な矩形範囲和クエリを提供するデータ構造です。
/// 前計算により、任意の矩形領域の合計を O(1) で計算できます。
///
/// # Examples
/// ```
/// use cumulative_sum_2d::CumulativeSum2D;
///
/// let cum_sum = CumulativeSum2D::new(&vec![
///     vec![1, 1, 1, 1, 1],
///     vec![1, 1, 1, 1, 1],
///     vec![1, 2, 2, 1, 1],
///     vec![1, 1, 1, 1, 1],
/// ]);
///
/// // whole
/// assert_eq!(cum_sum.sum(0..4, 0..5), 22);
///
/// // 1 1 . . .
/// // 1 1 . . .
/// // . . . . .
/// // . . . . .
/// assert_eq!(cum_sum.sum(0..2, 0..2), 4);
///
/// // . . . . .
/// // . 1 1 1 1
/// // . 2 2 1 1
/// // . . . . .
/// assert_eq!(cum_sum.sum(1..3, 1..4), 8);
/// ```
///
/// ## 競技プログラミングでの典型的な使用パターン
///
/// ```
/// use cumulative_sum_2d::CumulativeSum2D;
///
/// // imos法との組み合わせ
/// fn solve_2d_imos(h: usize, w: usize, updates: &[(usize, usize, usize, usize, i32)]) -> Vec<Vec<i32>> {
///     let mut grid = vec![vec![0; w]; h];
///     
///     // imos法で更新を適用
///     for &(r1, c1, r2, c2, val) in updates {
///         grid[r1][c1] += val;
///         if r2 < h { grid[r2][c1] -= val; }
///         if c2 < w { grid[r1][c2] -= val; }
///         if r2 < h && c2 < w { grid[r2][c2] += val; }
///     }
///     
///     // 累積和で実際の値を復元
///     let cum_sum = CumulativeSum2D::new(&grid);
///     let mut result = vec![vec![0; w]; h];
///     for i in 0..h {
///         for j in 0..w {
///             result[i][j] = cum_sum.sum(0..i+1, 0..j+1);
///         }
///     }
///     result
/// }
///
/// // テスト: (0,0)-(2,2) に +1 を適用
/// let updates = vec![(0, 0, 2, 2, 1)];
/// let result = solve_2d_imos(3, 3, &updates);
/// assert_eq!(result[1][1], 1); // 範囲内
/// assert_eq!(result[2][2], 0); // 範囲外
/// ```
pub struct CumulativeSum2D<T> {
    h: usize,
    w: usize,
    cum_sum: Vec<Vec<T>>,
}

impl<T> CumulativeSum2D<T>
where
    T: Clone + Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    /// 二次元配列から累積和を構築します。
    ///
    /// # Panics
    ///
    /// - `grid` が空の場合
    /// - `grid` の各行の長さが異なる場合
    ///
    /// # Examples
    ///
    /// ```
    /// use cumulative_sum_2d::CumulativeSum2D;
    ///
    /// let grid = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    /// ];
    /// let cum_sum = CumulativeSum2D::new(&grid);
    /// 
    /// // 構築後、各種クエリが可能
    /// assert_eq!(cum_sum.sum(0..2, 0..3), 21); // 全体の合計
    /// assert_eq!(cum_sum.sum(0..1, 0..2), 3);  // 上段左2要素
    /// ```
    pub fn new(grid: &[Vec<T>]) -> Self {
        let h = grid.len();
        assert!(h >= 1);
        let w = grid[0].len();
        for row in grid {
            assert_eq!(row.len(), w);
        }
        let mut cum_sum = grid.to_vec();
        #[allow(clippy::needless_range_loop)]
        for i in 0..h {
            for j in 1..w {
                cum_sum[i][j] = cum_sum[i][j] + cum_sum[i][j - 1];
            }
        }
        for j in 0..w {
            for i in 1..h {
                cum_sum[i][j] = cum_sum[i - 1][j] + cum_sum[i][j];
            }
        }
        Self { h, w, cum_sum }
    }

    /// 指定された矩形範囲の合計を返します。
    ///
    /// 包除原理を使用して、矩形 `(y_range.start, x_range.start)` から 
    /// `(y_range.end-1, x_range.end-1)` までの要素の合計を O(1) で計算します。
    ///
    /// # 引数
    ///
    /// - `y_range`: 行の範囲（半開区間）
    /// - `x_range`: 列の範囲（半開区間）
    ///
    /// # 戻り値
    ///
    /// 指定された矩形範囲内の要素の合計。範囲が空の場合は `T::default()`。
    ///
    /// # Panics
    ///
    /// - `y_range.end > h` または `x_range.end > w` の場合
    ///
    /// # Examples
    ///
    /// ```
    /// use cumulative_sum_2d::CumulativeSum2D;
    ///
    /// let grid = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    /// ];
    /// let cum_sum = CumulativeSum2D::new(&grid);
    ///
    /// // 中央の2x2領域
    /// assert_eq!(cum_sum.sum(0..2, 1..3), 16); // 2+3+5+6
    ///
    /// // 単一要素
    /// assert_eq!(cum_sum.sum(1..2, 1..2), 5);
    ///
    /// // 空の範囲
    /// assert_eq!(cum_sum.sum(1..1, 1..2), 0);
    /// assert_eq!(cum_sum.sum(1..2, 1..1), 0);
    /// ```
    ///
    /// ## パフォーマンス比較
    ///
    /// ```
    /// use cumulative_sum_2d::CumulativeSum2D;
    ///
    /// let large_grid = vec![vec![1; 1000]; 1000];
    /// let cum_sum = CumulativeSum2D::new(&large_grid);
    ///
    /// // O(1) での範囲和計算
    /// let sum1 = cum_sum.sum(100..200, 300..400); // 即座に計算
    /// let sum2 = cum_sum.sum(0..500, 0..600);     // こちらも即座に計算
    ///
    /// assert_eq!(sum1, 10000); // 100x100の領域
    /// assert_eq!(sum2, 300000); // 500x600の領域
    /// ```
    pub fn sum(&self, y_range: Range<usize>, x_range: Range<usize>) -> T {
        let (y_start, y_end) = (y_range.start, y_range.end);
        let (x_start, x_end) = (x_range.start, x_range.end);
        if y_start >= y_end || x_start >= x_end {
            return T::default();
        }
        assert!(y_end <= self.h);
        assert!(x_end <= self.w);
        let sum = self.cum_sum[y_end - 1][x_end - 1];
        if y_start >= 1 && x_start >= 1 {
            return sum + self.cum_sum[y_start - 1][x_start - 1]
                - self.cum_sum[y_start - 1][x_end - 1]
                - self.cum_sum[y_end - 1][x_start - 1];
        }
        if y_start >= 1 {
            assert_eq!(x_start, 0);
            return sum - self.cum_sum[y_start - 1][x_end - 1];
        }
        if x_start >= 1 {
            assert_eq!(y_start, 0);
            return sum - self.cum_sum[y_end - 1][x_start - 1];
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::CumulativeSum2D;

    #[test]
    fn test() {
        let grid: Vec<Vec<u32>> = vec![
            vec![3, 1, 4, 1, 5],
            vec![9, 2, 6, 5, 3],
            vec![5, 8, 9, 7, 9],
            vec![3, 2, 3, 8, 4],
        ];
        let cum_sum = CumulativeSum2D::new(&grid);
        for y_start in 0..=4 {
            for y_end in 0..=4 {
                for x_start in 0..=5 {
                    for x_end in 0..=5 {
                        let mut expected = 0;
                        for y in y_start..y_end {
                            for x in x_start..x_end {
                                expected += grid[y][x];
                            }
                        }
                        let actual = cum_sum.sum(y_start..y_end, x_start..x_end);
                        assert_eq!(expected, actual);
                    }
                }
            }
        }
    }
}
