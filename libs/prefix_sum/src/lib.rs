//! 任意次元の prefix sum を提供します。
//!
//! ## Examples
//!
//! ### 二次元
//!
//! ```
//! use prefix_sum::PrefixSum2DBuilder;
//!
//! let mut builder = PrefixSum2DBuilder::new(2, 3);
//! builder.set(0, 1, 10_i64);
//! let prefix_sum = builder.build();
//!
//! assert_eq!(prefix_sum.sum(0..2, 1..3), 10);
//! ```
//!
//! ### 三次元
//!
//! ```
//! use prefix_sum::PrefixSumBuilder;
//!
//! let mut builder = PrefixSumBuilder::new(&[2, 3, 4]);
//! builder.set(&[0, 1, 2], 10_i64);
//! let prefix_sum = builder.build();
//!
//! assert_eq!(prefix_sum.sum(&[0..2, 1..3, 0..4]), 10);
//! ```

use std::ops::Range;

/// 任意次元 prefix sum の入力を構築します。
pub struct PrefixSumBuilder<T> {
    shape: Vec<usize>,
    strides: Vec<usize>,
    values: Vec<T>,
}

impl<T> PrefixSumBuilder<T>
where
    T: PrefixSumValue,
{
    /// 各軸の長さが `shape` の prefix sum builder を作成します。
    ///
    /// # Panics
    ///
    /// `shape` が空、いずれかの軸長が 0、または要素数が `usize` に収まらない場合に
    /// panic します。
    pub fn new(shape: &[usize]) -> Self {
        assert!(!shape.is_empty(), "shape must have at least one dimension");
        assert!(
            shape.iter().all(|&length| length > 0),
            "each axis length must be positive"
        );

        let (strides, len) = strides_and_len(shape);
        Self {
            shape: shape.to_vec(),
            strides,
            values: vec![T::zero(); len],
        }
    }

    /// `index` の要素を `value` で上書きします。
    ///
    /// # Panics
    ///
    /// `index` の次元数が shape と異なる、またはいずれかの座標が範囲外の場合に panic
    /// します。
    pub fn set(&mut self, index: &[usize], value: T) {
        let flat_index = flat_index(&self.shape, &self.strides, index);
        self.values[flat_index] = value;
    }

    /// 入力を prefix sum に変換します。
    ///
    /// 計算量は O(N * ∏ D_i) です。N は次元数、D_i は各軸の長さです。
    pub fn build(mut self) -> PrefixSum<T> {
        for axis in 0..self.shape.len() {
            let stride = self.strides[axis];
            let axis_length = self.shape[axis];
            let block_length = stride * axis_length;

            for block_start in (0..self.values.len()).step_by(block_length) {
                for coordinate in 1..axis_length {
                    let start = block_start + coordinate * stride;
                    for offset in 0..stride {
                        let previous = self.values[start - stride + offset].clone();
                        self.values[start + offset].add_assign(&previous);
                    }
                }
            }
        }

        PrefixSum {
            shape: self.shape,
            strides: self.strides,
            values: self.values,
        }
    }
}

/// 任意次元 prefix sum です。
pub struct PrefixSum<T> {
    shape: Vec<usize>,
    strides: Vec<usize>,
    values: Vec<T>,
}

impl<T> PrefixSum<T>
where
    T: PrefixSumValue,
{
    /// 各軸の半開区間 `ranges` の和を返します。
    ///
    /// 空区間を含む場合は零元を返します。
    ///
    /// 計算量は O(N * 2^N) です。N は次元数です。
    ///
    /// # Panics
    ///
    /// `ranges` の次元数が shape と異なる、いずれかの range が逆順、または範囲外の
    /// 場合に panic します。
    pub fn sum(&self, ranges: &[Range<usize>]) -> T {
        assert_eq!(
            ranges.len(),
            self.shape.len(),
            "range rank must match shape rank"
        );
        for (range, &axis_length) in ranges.iter().zip(&self.shape) {
            assert!(
                range.start <= range.end && range.end <= axis_length,
                "range must be within the shape"
            );
        }
        if ranges.iter().any(|range| range.start == range.end) {
            return T::zero();
        }

        assert!(
            self.shape.len() < usize::BITS as usize,
            "rank is too large to enumerate query corners"
        );
        let corner_count = 1usize << self.shape.len();
        let mut corners = Vec::with_capacity(corner_count);

        for mask in 0..corner_count {
            let mut flat_index = 0;
            let mut is_zero = false;
            #[expect(clippy::needless_range_loop)]
            for axis in 0..self.shape.len() {
                let endpoint = if mask & (1usize << axis) == 0 {
                    ranges[axis].start
                } else {
                    ranges[axis].end
                };
                if endpoint == 0 {
                    is_zero = true;
                    break;
                }
                flat_index += (endpoint - 1) * self.strides[axis];
            }
            corners.push(if is_zero {
                T::zero()
            } else {
                self.values[flat_index].clone()
            });
        }

        for axis in 0..self.shape.len() {
            let bit = 1usize << axis;
            for mask in 0..corner_count {
                if mask & bit != 0 {
                    let lower = corners[mask ^ bit].clone();
                    corners[mask].sub_assign(&lower);
                }
            }
        }

        corners.pop().unwrap()
    }
}

/// 一次元 prefix sum の入力を構築します。
pub struct PrefixSum1DBuilder<T> {
    inner: PrefixSumBuilder<T>,
}

impl<T> PrefixSum1DBuilder<T>
where
    T: PrefixSumValue,
{
    /// 指定した長さの一次元 prefix sum builder を作成します。
    pub fn new(length: usize) -> Self {
        Self {
            inner: PrefixSumBuilder::new(&[length]),
        }
    }

    /// 指定した位置の要素を値で上書きします。
    pub fn set(&mut self, index: usize, value: T) {
        self.inner.set(&[index], value);
    }

    /// 入力を prefix sum に変換します。
    ///
    /// 計算量は O(length) です。
    pub fn build(self) -> PrefixSum1D<T> {
        PrefixSum1D {
            inner: self.inner.build(),
        }
    }
}

/// 一次元 prefix sum です。
pub struct PrefixSum1D<T> {
    inner: PrefixSum<T>,
}

impl<T> PrefixSum1D<T>
where
    T: PrefixSumValue,
{
    /// 指定した半開区間の和を返します。
    ///
    /// 計算量は O(1) です。
    pub fn sum(&self, range: Range<usize>) -> T {
        self.inner.sum(&[range])
    }
}

/// 二次元 prefix sum の入力を構築します。
pub struct PrefixSum2DBuilder<T> {
    inner: PrefixSumBuilder<T>,
}

impl<T> PrefixSum2DBuilder<T>
where
    T: PrefixSumValue,
{
    /// 指定した高さと幅の二次元 prefix sum builder を作成します。
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            inner: PrefixSumBuilder::new(&[height, width]),
        }
    }

    /// 指定した行・列の要素を値で上書きします。
    pub fn set(&mut self, row: usize, column: usize, value: T) {
        self.inner.set(&[row, column], value);
    }

    /// 入力を prefix sum に変換します。
    ///
    /// 計算量は O(height * width) です。
    pub fn build(self) -> PrefixSum2D<T> {
        PrefixSum2D {
            inner: self.inner.build(),
        }
    }
}

/// 二次元 prefix sum です。
pub struct PrefixSum2D<T> {
    inner: PrefixSum<T>,
}

impl<T> PrefixSum2D<T>
where
    T: PrefixSumValue,
{
    /// 指定した行・列の半開区間の和を返します。
    ///
    /// 計算量は O(1) です。
    pub fn sum(&self, rows: Range<usize>, columns: Range<usize>) -> T {
        self.inner.sum(&[rows, columns])
    }
}

fn strides_and_len(shape: &[usize]) -> (Vec<usize>, usize) {
    let mut strides = vec![0; shape.len()];
    let mut len = 1usize;
    for axis in (0..shape.len()).rev() {
        strides[axis] = len;
        len = len
            .checked_mul(shape[axis])
            .expect("shape is too large to fit in usize");
    }
    (strides, len)
}

fn flat_index(shape: &[usize], strides: &[usize], index: &[usize]) -> usize {
    assert_eq!(index.len(), shape.len(), "index rank must match shape rank");
    index
        .iter()
        .zip(shape)
        .zip(strides)
        .map(|((&coordinate, &axis_length), &stride)| {
            assert!(coordinate < axis_length, "index must be within the shape");
            coordinate * stride
        })
        .sum()
}

/// Prefix sum の要素型です。
///
/// `zero` は加法単位元であり、`add_assign` と `sub_assign` はそれぞれ加算と減算を
/// 行う必要があります。
pub trait PrefixSumValue: Clone {
    fn zero() -> Self;
    fn add_assign(&mut self, rhs: &Self);
    fn sub_assign(&mut self, rhs: &Self);
}

macro_rules! impl_prefix_sum_value {
    ($($t:ty),+ $(,)?) => {
        $(
            impl PrefixSumValue for $t {
                fn zero() -> Self {
                    0
                }

                fn add_assign(&mut self, rhs: &Self) {
                    *self += *rhs;
                }

                fn sub_assign(&mut self, rhs: &Self) {
                    *self -= *rhs;
                }
            }
        )+
    };
}

impl_prefix_sum_value!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);

#[cfg(test)]
mod tests {
    use super::{PrefixSum1DBuilder, PrefixSum2DBuilder, PrefixSumBuilder, PrefixSumValue};

    #[test]
    fn all_ranges_in_irregular_three_dimensions() {
        let shape = [2, 3, 4];
        let values = vec![
            vec![
                vec![-12, -11, -10, -9],
                vec![-8, -7, -6, -5],
                vec![-4, -3, -2, -1],
            ],
            vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]],
        ];
        let mut builder = PrefixSumBuilder::new(&shape);
        for (a, plane) in values.iter().enumerate() {
            for (b, row) in plane.iter().enumerate() {
                for (c, &value) in row.iter().enumerate() {
                    builder.set(&[a, b, c], value);
                }
            }
        }
        let prefix_sum = builder.build();

        for a_start in 0..=shape[0] {
            for a_end in a_start..=shape[0] {
                for b_start in 0..=shape[1] {
                    for b_end in b_start..=shape[1] {
                        for c_start in 0..=shape[2] {
                            for c_end in c_start..=shape[2] {
                                let ranges = [a_start..a_end, b_start..b_end, c_start..c_end];
                                let mut expected = 0;
                                for a in a_start..a_end {
                                    for b in b_start..b_end {
                                        for c in c_start..c_end {
                                            expected += values[a][b][c];
                                        }
                                    }
                                }
                                assert_eq!(prefix_sum.sum(&ranges), expected, "ranges: {ranges:?}");
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn one_and_two_dimensional_wrappers() {
        let mut one_dimensional = PrefixSum1DBuilder::new(5);
        for (index, value) in [3, 1, 4, 1, 5].into_iter().enumerate() {
            one_dimensional.set(index, value);
        }
        let one_dimensional = one_dimensional.build();
        assert_eq!(one_dimensional.sum(1..4), 6);

        let values = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut two_dimensional = PrefixSum2DBuilder::new(2, 3);
        for (row, values) in values.iter().enumerate() {
            for (column, &value) in values.iter().enumerate() {
                two_dimensional.set(row, column, value);
            }
        }
        let two_dimensional = two_dimensional.build();
        assert_eq!(two_dimensional.sum(0..2, 1..3), 16);
    }

    #[test]
    fn empty_range_returns_zero() {
        let mut builder = PrefixSumBuilder::new(&[3]);
        builder.set(&[0], 10);
        let prefix_sum = builder.build();

        assert_eq!(prefix_sum.sum(&[0..0]), 0);
        assert_eq!(prefix_sum.sum(&[2..2]), 0);
    }

    #[test]
    fn usize_with_nonzero_starts() {
        let values = vec![
            vec![1_usize, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let mut builder = PrefixSumBuilder::new(&[3, 4]);
        for (row, values) in values.iter().enumerate() {
            for (column, &value) in values.iter().enumerate() {
                builder.set(&[row, column], value);
            }
        }
        let prefix_sum = builder.build();

        assert_eq!(prefix_sum.sum(&[1..3, 1..4]), 54);
    }

    #[test]
    fn set_overwrites_and_unset_values_are_zero() {
        let mut builder = PrefixSumBuilder::new(&[2]);
        builder.set(&[0], 10);
        builder.set(&[0], 3);
        let prefix_sum = builder.build();

        assert_eq!(prefix_sum.sum(&[0..2]), 3);
        assert_eq!(prefix_sum.sum(&[1..2]), 0);
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Sum(i64);

    impl PrefixSumValue for Sum {
        fn zero() -> Self {
            Self(0)
        }

        fn add_assign(&mut self, rhs: &Self) {
            self.0 += rhs.0;
        }

        fn sub_assign(&mut self, rhs: &Self) {
            self.0 -= rhs.0;
        }
    }

    #[test]
    fn supports_non_copy_values() {
        let mut builder = PrefixSumBuilder::new(&[3]);
        builder.set(&[0], Sum(3));
        builder.set(&[1], Sum(1));
        builder.set(&[2], Sum(4));
        let prefix_sum = builder.build();

        assert_eq!(prefix_sum.sum(&[1..3]), Sum(5));
    }

    #[test]
    #[should_panic]
    fn empty_shape_panics() {
        PrefixSumBuilder::<i32>::new(&[]);
    }

    #[test]
    #[should_panic]
    fn zero_length_axis_panics() {
        PrefixSumBuilder::<i32>::new(&[0]);
    }

    #[test]
    #[should_panic]
    fn invalid_index_rank_panics() {
        let mut builder = PrefixSumBuilder::<i32>::new(&[2]);
        builder.set(&[0, 0], 1);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_index_panics() {
        let mut builder = PrefixSumBuilder::<i32>::new(&[2]);
        builder.set(&[2], 1);
    }

    #[test]
    #[should_panic]
    fn invalid_range_rank_panics() {
        let prefix_sum = PrefixSumBuilder::<i32>::new(&[2]).build();
        prefix_sum.sum(&[0..2, 0..1]);
    }

    #[test]
    #[should_panic]
    fn reversed_range_panics() {
        let prefix_sum = PrefixSumBuilder::<i32>::new(&[2]).build();
        prefix_sum.sum(&[1..0]);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_range_panics() {
        let prefix_sum = PrefixSumBuilder::<i32>::new(&[2]).build();
        prefix_sum.sum(&[0..3]);
    }
}
