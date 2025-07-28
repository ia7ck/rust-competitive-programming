//! スライディングウィンドウを使った最小値・最大値計算のライブラリです。
//!
//! 固定サイズの窓をスライドさせながら、各窓での最小値または最大値を効率的に計算します。
//! デックを使用したアルゴリズムにより、全体で O(n) の時間計算量を実現します。
//!
//! # 計算量
//!
//! - 時間計算量: O(n)（全体）
//! - 空間計算量: O(k) ここで k は窓のサイズ
//!
//! # アルゴリズム
//!
//! モノトニックデック（単調デック）を使用します：
//! - 最小値の場合: デック内で値が単調増加になるよう維持
//! - 最大値の場合: デック内で値が単調減少になるよう維持
//!
//! # 用途
//!
//! - 固定サイズ窓での最小値・最大値クエリ
//! - 動的プログラミングの最適化
//! - 競技プログラミングでのスライディングウィンドウ問題
//! - 時系列データの解析
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use sliding_window::{sliding_window_minimum, sliding_window_maximum};
//!
//! let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
//! 
//! // 窓サイズ3での最小値
//! let minimums = sliding_window_minimum(&data, 3);
//! assert_eq!(minimums, vec![&1, &1, &1, &1, &2, &2]);
//! 
//! // 窓サイズ3での最大値  
//! let maximums = sliding_window_maximum(&data, 3);
//! assert_eq!(maximums, vec![&4, &4, &5, &9, &9, &6]);
//! ```
//!
//! ## 競技プログラミングでの応用例
//!
//! ```
//! use sliding_window::sliding_window_minimum;
//!
//! // 最小コストでk個連続する要素の和を最小化
//! fn min_subarray_sum(arr: &[i32], k: usize) -> i32 {
//!     if arr.len() < k { return 0; }
//!     
//!     // 窓サイズkでの合計を計算
//!     let mut window_sum = arr[0..k].iter().sum::<i32>();
//!     let mut min_sum = window_sum;
//!     
//!     for i in k..arr.len() {
//!         window_sum += arr[i] - arr[i - k];
//!         min_sum = min_sum.min(window_sum);
//!     }
//!     min_sum
//! }
//!
//! let arr = vec![2, 1, 4, 9, 2, 5, 1, 3];
//! assert_eq!(min_subarray_sum(&arr, 3), 7); // [2, 1, 4] または [2, 5, 1] など
//! ```

use std::collections::VecDeque;

/// 幅 `window_width` の区間すべてに対し最小値を求めます。
///
/// 配列 `a` に対してスライディングウィンドウを適用し、
/// 各位置での窓内最小値を効率的に計算します。
///
/// 配列 `a` に対し次で定める配列 `b` を求めます：
///
/// - `a` の長さ `a.len()` を `n` とする
/// - `b[0]`: `min(a[0], a[1], ..., a[window_width - 1])`
/// - `b[1]`: `min(a[1], a[2], ..., a[window_width])`
/// - ...
/// - `b[n - window_width]`: `min(a[n - window_width], ..., a[n - 2], a[n - 1])`
///
/// # アルゴリズム
///
/// モノトニックデック（単調増加デック）を使用して、各窓での最小値を O(1) で取得します。
/// [実装の参考資料](https://qiita.com/kuuso1/items/318d42cd089a49eeb332)
///
/// # Panics
///
/// `window_width` が 0 または `a.len()` より大きい場合にパニックします。
///
/// # Examples
///
/// ```
/// use sliding_window::sliding_window_minimum;
///
/// let a = vec![4, 7, 7, 8, 5, 7, 6, 9, 9, 2, 8, 3];
/// let minimums = sliding_window_minimum(&a, 6);
/// assert_eq!(
///     minimums,
///     vec![
///         &4, // 4 7 7 8 5 7
///         &5, //   7 7 8 5 7 6
///         &5, //     7 8 5 7 6 9
///         &5, //       8 5 7 6 9 9
///         &2, //         5 7 6 9 9 2
///         &2, //           7 6 9 9 2 8
///         &2, //             6 9 9 2 8 3
///     ]
/// );
/// ```
///
/// ## 競技プログラミングでの応用例
///
/// ```
/// use sliding_window::sliding_window_minimum;
///
/// // 配列の各k個連続部分の最小値と最大値の差の最大値を求める
/// fn max_min_difference(arr: &[i32], k: usize) -> i32 {
///     if arr.len() < k { return 0; }
///     
///     let minimums = sliding_window_minimum(arr, k);
///     let maximums = sliding_window::sliding_window_maximum(arr, k);
///     
///     minimums.iter().zip(maximums.iter())
///         .map(|(min, max)| **max - **min)
///         .max()
///         .unwrap_or(0)
/// }
///
/// let arr = vec![1, 3, 2, 7, 5, 1, 4];
/// let max_diff = max_min_difference(&arr, 3);
/// assert_eq!(max_diff, 6); // [2, 7, 5] の差 7-2=5 など
/// ```
pub fn sliding_window_minimum<T>(a: &[T], window_width: usize) -> Vec<&T>
where
    T: Ord,
{
    sliding_window(a, window_width, |last, new| last >= new)
}

/// [`sliding_window_minimum`] の最大値バージョンです。
///
/// 幅 `window_width` の区間すべてに対し最大値を求めます。
/// アルゴリズムは最小値版と同様ですが、モノトニックデックを単調減少で維持します。
///
/// # Examples
///
/// ```
/// use sliding_window::sliding_window_maximum;
///
/// let a = vec![4, 7, 7, 8, 5, 7, 6, 9, 9, 2, 8, 3];
/// let maximums = sliding_window_maximum(&a, 4);
/// assert_eq!(
///     maximums,
///     vec![
///         &8, // 4 7 7 8
///         &8, //   7 7 8 5
///         &8, //     7 8 5 7
///         &8, //       8 5 7 6
///         &9, //         5 7 6 9
///         &9, //           7 6 9 9
///         &9, //             6 9 9 2
///         &9, //               9 9 2 8
///         &8, //                 9 2 8 3
///     ]
/// );
/// ```
///
/// ## Range Minimum/Maximum Query (RMQ) での使用
///
/// ```
/// use sliding_window::{sliding_window_minimum, sliding_window_maximum};
///
/// // 固定サイズ窓での RMQ を前計算
/// fn precompute_rmq(arr: &[i32], window_size: usize) -> (Vec<i32>, Vec<i32>) {
///     let mins: Vec<i32> = sliding_window_minimum(arr, window_size)
///         .into_iter().map(|&x| x).collect();
///     let maxs: Vec<i32> = sliding_window_maximum(arr, window_size)
///         .into_iter().map(|&x| x).collect();
///     (mins, maxs)
/// }
///
/// let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// let (mins, maxs) = precompute_rmq(&data, 3);
/// 
/// // 各窓での最小値・最大値が前計算されている
/// assert_eq!(mins[0], 1); // [3,1,4] の最小値
/// assert_eq!(maxs[0], 4); // [3,1,4] の最大値
/// assert_eq!(mins[3], 1); // [1,5,9] の最小値
/// assert_eq!(maxs[3], 9); // [1,5,9] の最大値
/// ```
///
/// [`sliding_window_minimum`]: fn.sliding_window_minimum.html
pub fn sliding_window_maximum<T>(a: &[T], window_width: usize) -> Vec<&T>
where
    T: Ord,
{
    sliding_window(a, window_width, |last, new| last <= new)
}

fn sliding_window<T, F>(a: &[T], window_width: usize, pop_back_cond: F) -> Vec<&T>
where
    T: Ord,
    F: Fn(&T, &T) -> bool, // (last, new)
{
    assert!(0 < window_width && window_width <= a.len());
    let mut result = Vec::new();
    let mut positions: VecDeque<usize> = VecDeque::new();
    for (i, v) in a.iter().enumerate() {
        while let Some(last) = positions.back().copied() {
            if pop_back_cond(&a[last], v) {
                positions.pop_back();
            } else {
                break;
            }
        }
        positions.push_back(i);
        if i >= window_width - 1 {
            let p = positions.front().copied().unwrap();
            result.push(&a[p]);
            if p == i - (window_width - 1) {
                positions.pop_front();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{sliding_window_maximum, sliding_window_minimum};

    #[test]
    fn test_min() {
        let a = vec![2, 2, 3, 6, 0, 6, 7, 9, 7, 7, 4, 9];
        assert_eq!(
            sliding_window_minimum(&a, 4),
            vec![
                &2, // 2 2 3 6
                &0, //   2 3 6 0
                &0, //     3 6 0 6
                &0, //       6 0 6 7
                &0, //         0 6 7 9
                &6, //           6 7 9 7
                &7, //             7 9 7 7
                &4, //               9 7 7 4
                &4, //                 7 7 4 9
            ]
        );

        assert_eq!(sliding_window_minimum(&a, 1), a.iter().collect::<Vec<_>>());

        assert_eq!(
            sliding_window_minimum(&a, a.len()),
            vec![a.iter().min().unwrap()],
        );
    }

    #[test]
    fn test_max() {
        let a = vec![2, 2, 3, 6, 0, 6, 7, 9, 7, 7, 4, 9];
        assert_eq!(
            sliding_window_maximum(&a, 4),
            vec![
                &6, // 2 2 3 6
                &6, //   2 3 6 0
                &6, //     3 6 0 6
                &7, //       6 0 6 7
                &9, //         0 6 7 9
                &9, //           6 7 9 7
                &9, //             7 9 7 7
                &9, //               9 7 7 4
                &9, //                 7 7 4 9
            ]
        );

        assert_eq!(sliding_window_maximum(&a, 1), a.iter().collect::<Vec<_>>());

        assert_eq!(
            sliding_window_maximum(&a, a.len()),
            vec![a.iter().max().unwrap()],
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_0() {
        assert!(sliding_window_minimum::<u32>(&[], 0).is_empty());
    }

    #[test]
    #[should_panic]
    fn test_empty_1() {
        assert!(sliding_window_minimum::<u32>(&[], 1).is_empty());
    }
}
