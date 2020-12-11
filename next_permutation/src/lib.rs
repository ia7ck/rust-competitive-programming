pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> NextPermutation for [T] {
    /// 数列を辞書順でひとつ進めます。進めなかったら false を返します。
    ///
    /// # Examples
    /// ```
    /// use next_permutation::NextPermutation;
    /// let mut a = vec![1, 2, 3];
    /// a.next_permutation();
    /// assert_eq!(a, vec![1, 3, 2]);
    /// a.next_permutation();
    /// assert_eq!(a, vec![2, 1, 3]);
    /// let mut a = vec![3, 2, 1];
    /// assert!(!a.next_permutation());
    /// ```
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[i - 1] >= self[j] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let mut a: Vec<i32> = vec![];
        assert!(!a.next_permutation());
    }

    #[test]
    fn one_test() {
        let mut a = vec![1];
        assert!(!a.next_permutation());
    }

    #[test]
    fn uniq_test() {
        let mut a = vec![1, 2, 3];
        let want = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        for i in 0..want.len() {
            assert_eq!(a, want[i]);
            if i < want.len() - 1 {
                assert_eq!(a.next_permutation(), true);
            } else {
                assert_eq!(a.next_permutation(), false);
            }
        }
    }
    #[test]
    fn general_test() {
        let mut a = vec![1, 2, 2, 3];
        let want = vec![
            vec![1, 2, 2, 3],
            vec![1, 2, 3, 2],
            vec![1, 3, 2, 2],
            vec![2, 1, 2, 3],
            vec![2, 1, 3, 2],
            vec![2, 2, 1, 3],
            vec![2, 2, 3, 1],
            vec![2, 3, 1, 2],
            vec![2, 3, 2, 1],
            vec![3, 1, 2, 2],
            vec![3, 2, 1, 2],
            vec![3, 2, 2, 1],
        ];
        for i in 0..want.len() {
            assert_eq!(a, want[i]);
            if i < want.len() - 1 {
                assert_eq!(a.next_permutation(), true);
            } else {
                assert_eq!(a.next_permutation(), false);
            }
        }
    }
}
