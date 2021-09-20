/// 0 以上 `n` 未満の全ての `i`, `j` について二項係数 `i` choose `j` (mod `m`) を求めます。
///
/// # Examples
///
/// ```
/// use pascal_triangle::pascal_triangle;
/// assert_eq!(
///     pascal_triangle(5, 100000),
///     vec![
///         vec![1, 0, 0, 0, 0],
///         vec![1, 1, 0, 0, 0],
///         vec![1, 2, 1, 0, 0],
///         vec![1, 3, 3, 1, 0],
///         vec![1, 4, 6, 4, 1],
///     ],
/// );
/// ```
pub fn pascal_triangle(n: usize, m: u64) -> Vec<Vec<u64>> {
    let mut binom = vec![vec![0; n]; n];
    binom[0][0] = 1;
    for i in 1..n {
        binom[i][0] = 1;
        for j in 1..n {
            binom[i][j] = (binom[i - 1][j - 1] + binom[i - 1][j]) % m;
        }
    }
    binom
}

#[cfg(test)]
mod tests {
    use crate::pascal_triangle;

    #[test]
    fn test() {
        assert_eq!(
            pascal_triangle(6, 100000),
            vec![
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0],
                vec![1, 2, 1, 0, 0, 0],
                vec![1, 3, 3, 1, 0, 0],
                vec![1, 4, 6, 4, 1, 0],
                vec![1, 5, 10, 10, 5, 1],
            ],
        );
    }
}
