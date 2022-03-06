use std::fmt::Debug;
use std::iter::FromIterator;

/// 座標圧縮です。
///
/// # Examples
///
/// ```
/// use std::iter::FromIterator;
/// use coordinate_compression::OrderMap;
/// let values = vec![2, -1, -1, 5, -1, 2, -3];
/// // -3, -1, 2, 5
/// let map = OrderMap::from_iter(values.into_iter());
/// assert_eq!(map.ord(&-3), 0);
/// assert_eq!(map.ord(&-1), 1);
/// assert_eq!(map.ord(&2), 2);
/// assert_eq!(map.ord(&5), 3);
///
/// assert_eq!(map.at(0), &(-3));
/// assert_eq!(map.at(1), &(-1));
/// assert_eq!(map.at(2), &2);
/// assert_eq!(map.at(3), &5);
/// ```
///
/// # Panics
///
/// 構築時に与えられなかったキーを引くとパニックです。
///
/// ```should_panic
/// use coordinate_compression::OrderMap;
/// let primes = vec![2, 3, 5, 7, 11];
/// let map: OrderMap<i32> = primes.into_iter().collect();
/// map.ord(&4);
/// ```
///
/// index が unique な要素数以上だとパニックです。
///
/// ```should_panic
/// use coordinate_compression::OrderMap;
/// let primes = vec![1, 1, 2, 2, 3, 4, 9, 9];
/// let map: OrderMap<i32> = primes.into_iter().collect();
/// map.at(5);
/// ```
///
#[derive(Debug)]
pub struct OrderMap<T>(Vec<T>);

impl<T> FromIterator<T> for OrderMap<T>
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

impl<T> OrderMap<T>
where
    T: Ord + Debug,
{
    pub fn ord(&self, value: &T) -> usize {
        self.0
            .binary_search(value)
            .unwrap_or_else(|_| panic!("not found {:?}", value))
    }

    pub fn at(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T> OrderMap<T> {
    /// 保持している要素のうち unique な要素の個数を返します。
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::OrderMap;
    use std::iter::FromIterator;

    #[test]
    fn ord_test() {
        let map: OrderMap<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        // 2, 4, 5, 9
        assert_eq!(map.ord(&2), 0);
        assert_eq!(map.ord(&4), 1);
        assert_eq!(map.ord(&5), 2);
        assert_eq!(map.ord(&9), 3);
    }

    #[test]
    fn index_test() {
        let map = OrderMap::from_iter([4, 4, 2, 5, 2, 9].iter().copied());
        // 2, 4, 5, 9
        assert_eq!(map.at(0), &2);
        assert_eq!(map.at(1), &4);
        assert_eq!(map.at(2), &5);
        assert_eq!(map.at(3), &9);
    }

    #[test]
    #[should_panic]
    fn not_found_test() {
        let map: OrderMap<i32> = vec![4, 4, 2, 5, 2, 9].into_iter().collect();
        map.ord(&6);
    }
}
