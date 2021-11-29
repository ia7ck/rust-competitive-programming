use std::cmp::Ordering::{Greater, Less};
use std::collections::VecDeque;

/// 幅 `window_width` の区間すべてに対し最小値を求めます。
///
/// 配列 `a` に対し次で定める配列 `b` を求めます。
///
/// - `a` の長さ `a.len()` を `n` とする
/// - `b[0]`: `min(a[0], a[1], ..., a[window_width - 1])`
/// - `b[1]`: `min(a[1], a[2], ..., a[window_width])`
/// - ...
/// - `b[n - window_width]`: `min(a[n - window_width], ..., a[n - 2], a[n - 1])`
///
/// [実装の参考資料](https://qiita.com/kuuso1/items/318d42cd089a49eeb332)
///
/// # Panics
///
/// if `window_width` is zero or is greater than `a.len()`.
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
///         4, // 4 7 7 8 5 7
///         5, //   7 7 8 5 7 6
///         5, //     7 8 5 7 6 9
///         5, //       8 5 7 6 9 9
///         2, //         5 7 6 9 9 2
///         2, //           7 6 9 9 2 8
///         2, //             6 9 9 2 8 3
///     ]
/// );
/// ```
pub fn sliding_window_minimum<T>(a: &[T], window_width: usize) -> Vec<T>
where
    T: Ord + Clone,
{
    sliding_window(a, window_width, true)
}

/// [sliding_window](fn.sliding_window.html) の最大値バージョンです。
pub fn sliding_window_maximum<T>(a: &[T], window_width: usize) -> Vec<T>
where
    T: Ord + Clone,
{
    sliding_window(a, window_width, false)
}

fn sliding_window<T>(a: &[T], window_width: usize, choose_minimum: bool) -> Vec<T>
where
    T: Ord + Clone,
{
    assert!(0 < window_width && window_width <= a.len());
    let mut result = Vec::new();
    let mut arg_min_max_candidates: VecDeque<usize> = VecDeque::new();
    for (i, v) in a.iter().enumerate() {
        while !arg_min_max_candidates.is_empty() {
            let back = arg_min_max_candidates.back().unwrap();
            if choose_minimum && a[*back].cmp(v) == Less {
                break;
            }
            if !choose_minimum && a[*back].cmp(v) == Greater {
                break;
            }
            arg_min_max_candidates.pop_back();
        }
        arg_min_max_candidates.push_back(i);
        if i >= window_width - 1 {
            let arg_min_max = arg_min_max_candidates.front().unwrap();
            result.push(Clone::clone(&a[*arg_min_max]));
            if *arg_min_max == i - (window_width - 1) {
                arg_min_max_candidates.pop_front();
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
                2, // 2 2 3 6
                0, //   2 3 6 0
                0, //     3 6 0 6
                0, //       6 0 6 7
                0, //         0 6 7 9
                6, //           6 7 9 7
                7, //             7 9 7 7
                4, //               9 7 7 4
                4  //                  7 7 4 9
            ]
        );

        assert_eq!(sliding_window_minimum(&a, 1), a);

        assert_eq!(
            sliding_window_minimum(&a, a.len()),
            vec![a.iter().copied().min().unwrap()],
        );
    }

    #[test]
    fn test_max() {
        let a = vec![2, 2, 3, 6, 0, 6, 7, 9, 7, 7, 4, 9];
        assert_eq!(
            sliding_window_maximum(&a, 4),
            vec![
                6, // 2 2 3 6
                6, //   2 3 6 0
                6, //     3 6 0 6
                7, //       6 0 6 7
                9, //         0 6 7 9
                9, //           6 7 9 7
                9, //             7 9 7 7
                9, //               9 7 7 4
                9  //                  7 7 4 9
            ]
        );

        assert_eq!(sliding_window_maximum(&a, 1), a);

        assert_eq!(
            sliding_window_maximum(&a, a.len()),
            vec![a.iter().copied().max().unwrap()],
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_0() {
        assert_eq!(sliding_window_minimum::<u32>(&[], 0), vec![]);
    }

    #[test]
    #[should_panic]
    fn test_empty_1() {
        assert_eq!(sliding_window_minimum::<u32>(&[], 1), vec![]);
    }
}
