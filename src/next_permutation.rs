mod next_permutation {
    pub trait NextPermutation {
        fn next_permutation(&mut self) -> bool;
    }

    impl<T: Ord> NextPermutation for [T] {
        fn next_permutation(&mut self) -> bool {
            if self.len() <= 0 {
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
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::next_permutation::NextPermutation;

    #[test]
    fn next_permutation_test() {
        let mut a = vec![1, 2, 3, 4];
        let want = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ];
        assert_eq!(want.len(), 24);
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
