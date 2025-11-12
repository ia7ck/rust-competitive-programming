use std::{
    fmt::{self, Debug},
    ops::Index,
};

/// 座標圧縮です。
///
/// # Examples
///
/// ```
/// use zarts::SortedSeq;
/// let values = vec![2, -1, -1, 5, -1, 2, -3];
/// // -3, -1, 2, 5
/// let seq = SortedSeq::new(values);
/// assert_eq!(seq.ord(&-3), 0);
/// assert_eq!(seq.ord(&-1), 1);
/// assert_eq!(seq.ord(&2), 2);
/// assert_eq!(seq.ord(&5), 3);
///
/// assert_eq!(seq.at(0), &(-3));
/// assert_eq!(seq.at(1), &(-1));
/// assert_eq!(seq.at(2), &2);
/// assert_eq!(seq.at(3), &5);
/// ```
///
/// # Panics
///
/// 構築時に与えられなかったキーを引くとパニックです。
///
/// ```should_panic
/// use zarts::SortedSeq;
/// let primes = vec![2, 3, 5, 7, 11];
/// let seq = SortedSeq::new(primes);
/// seq.ord(&4);
/// ```
///
/// index が集合のサイズ以上だとパニックです。
///
/// ```should_panic
/// use zarts::SortedSeq;
/// let values = vec![1, 1, 2, 2, 3, 4, 9, 9];
/// let seq = SortedSeq::new(values);
/// seq.at(5);
/// ```
///
pub struct SortedSeq<T>(Vec<T>);

impl<T> FromIterator<T> for SortedSeq<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<T> SortedSeq<T>
where
    T: Ord,
{
    pub fn new(mut values: Vec<T>) -> Self {
        values.sort_unstable();
        values.dedup();
        Self(values)
    }

    /// 集合内で小さいほうから何番目か (0-indexed) を返します
    pub fn ord(&self, value: &T) -> usize {
        self.0
            .binary_search(value)
            .unwrap_or_else(|_| panic!("not found"))
    }

    /// index 番目の値を返します
    pub fn at(&self, index: usize) -> &T {
        &self[index]
    }

    /// 集合のサイズを返します
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Index<usize> for SortedSeq<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> Debug for SortedSeq<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::SortedSeq;

    #[test]
    fn ord_test() {
        let seq = SortedSeq::new(vec![4, 4, 2, 5, 2, 9]);
        // 2, 4, 5, 9
        assert_eq!(seq.ord(&2), 0);
        assert_eq!(seq.ord(&4), 1);
        assert_eq!(seq.ord(&5), 2);
        assert_eq!(seq.ord(&9), 3);
    }

    #[test]
    fn index_test() {
        let seq = SortedSeq::new(vec![4, 4, 2, 5, 2, 9]);
        // 2, 4, 5, 9
        assert_eq!(seq.at(0), &2);
        assert_eq!(seq.at(1), &4);
        assert_eq!(seq.at(2), &5);
        assert_eq!(seq.at(3), &9);
    }

    #[test]
    #[should_panic]
    fn not_found_test() {
        let seq: SortedSeq<i32> = SortedSeq::new(vec![4, 4, 2, 5, 2, 9]);
        seq.ord(&6);
    }
}
