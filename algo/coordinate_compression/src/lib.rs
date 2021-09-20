use std::iter::FromIterator;

/// 座標圧縮です。
///
/// # Examples
///
/// ```
/// use coordinate_compression::CoordinateCompression;
/// let values = vec![2, -1, -1, 5, -1, 2, -3];
/// // -3, -1, 2, 5
/// let cc: CoordinateCompression<i32> = values.into_iter().collect();
/// assert_eq!(cc.find_index(&-3), 0);
/// assert_eq!(cc.find_index(&-1), 1);
/// assert_eq!(cc.find_index(&2), 2);
/// assert_eq!(cc.find_index(&5), 3);
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

#[cfg(test)]
mod tests {
    use crate::CoordinateCompression;

    #[test]
    fn test() {
        let cc: CoordinateCompression<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        // 2, 4, 5, 9
        assert_eq!(cc.find_index(&2), 0);
        assert_eq!(cc.find_index(&4), 1);
        assert_eq!(cc.find_index(&5), 2);
        assert_eq!(cc.find_index(&9), 3);
    }

    #[test]
    #[should_panic]
    fn not_found_test() {
        let cc: CoordinateCompression<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        cc.find_index(&6);
    }
}
