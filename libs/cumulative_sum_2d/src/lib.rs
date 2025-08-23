use std::ops::{Add, Range, Sub};

/// 二次元累積和です。
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
pub struct CumulativeSum2D<T> {
    h: usize,
    w: usize,
    cum_sum: Vec<Vec<T>>,
}

impl<T> CumulativeSum2D<T>
where
    T: Clone + Copy + Default + Add<Output = T> + Sub<Output = T>,
{
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
