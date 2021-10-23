use std::iter::FromIterator;
use std::ops::Index;

/// 座標圧縮です。
///
/// # Examples
///
/// ```
/// use std::iter::FromIterator;
/// use coordinate_compression::CoordinateCompression;
/// let values = vec![2, -1, -1, 5, -1, 2, -3];
/// // -3, -1, 2, 5
/// let cc = CoordinateCompression::from_iter(values.into_iter());
/// assert_eq!(cc.find_index(&-3), 0);
/// assert_eq!(cc.find_index(&-1), 1);
/// assert_eq!(cc.find_index(&2), 2);
/// assert_eq!(cc.find_index(&5), 3);
///
/// assert_eq!(cc[0], -3);
/// assert_eq!(cc[1], -1);
/// assert_eq!(cc[2], 2);
/// assert_eq!(cc[3], 5);
/// ```
///
/// # Panics
///
/// 構築時に与えられなかったキーを引くとパニックです。
///
/// ```should_panic
/// use coordinate_compression::CoordinateCompression;
/// let primes = vec![2, 3, 5, 7, 11];
/// let cc: CoordinateCompression<i32> = primes.into_iter().collect();
/// cc.find_index(&4);
/// ```
///
/// index が unique な要素数以上だとパニックです。
///
/// ```should_panic
/// use coordinate_compression::CoordinateCompression;
/// let primes = vec![1, 1, 2, 2, 3, 4, 9, 9];
/// let cc: CoordinateCompression<i32> = primes.into_iter().collect();
/// cc[5];
/// ```
///
#[derive(Debug)]
pub struct CoordinateCompression<T>(Vec<T>);

impl<T> FromIterator<T> for CoordinateCompression<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut values = Vec::new();
        for v in iter {
            values.push(v);
        }
        values.sort();
        values.dedup();
        Self(values)
    }
}

impl<T> CoordinateCompression<T>
where
    T: Ord,
{
    pub fn find_index(&self, value: &T) -> usize {
        self.0
            .binary_search(value)
            .unwrap_or_else(|_| panic!("not found"))
    }
}

impl<T> Index<usize> for CoordinateCompression<T>
where
    T: Ord,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> CoordinateCompression<T> {
    /// 保持している要素のうち unique な要素の個数を返します。
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::CoordinateCompression;

    #[test]
    fn find_index_test() {
        let cc: CoordinateCompression<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        // 2, 4, 5, 9
        assert_eq!(cc.find_index(&2), 0);
        assert_eq!(cc.find_index(&4), 1);
        assert_eq!(cc.find_index(&5), 2);
        assert_eq!(cc.find_index(&9), 3);
    }

    #[test]
    fn index_test() {
        let cc: CoordinateCompression<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        // 2, 4, 5, 9
        assert_eq!(cc[0], 2);
        assert_eq!(cc[1], 4);
        assert_eq!(cc[2], 5);
        assert_eq!(cc[3], 9);
    }

    #[test]
    #[should_panic]
    fn not_found_test() {
        let cc: CoordinateCompression<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        cc.find_index(&6);
    }
}
