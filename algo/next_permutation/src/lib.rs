//! Next Permutation アルゴリズムのライブラリです。
//!
//! 辞書順で次の順列を効率的に生成するアルゴリズムを提供します。
//! C++ の `std::next_permutation` と同等の機能を Rust で実装しています。
//!
//! # 計算量
//!
//! - 時間計算量: O(n) （最悪ケース）
//! - 空間計算量: O(1) （in-place で実行）
//!
//! # アルゴリズム
//!
//! 1. 右から左に向かって、隣接する要素が増加している最初の位置を見つける
//! 2. その位置より右側で、基準要素より大きい最小の要素を見つける
//! 3. 2つの要素を交換
//! 4. 基準位置より右側の部分を逆順にする
//!
//! # 用途
//!
//! - 順列の全列挙
//! - 辞書順での順列生成
//! - 競技プログラミングでの全探索
//! - 組み合わせ最適化問題
//!
//! # Examples
//!
//! ## 基本的な使用例
//!
//! ```
//! use next_permutation::NextPermutation;
//!
//! let mut v = vec![1, 2, 3];
//! 
//! // 順列を順番に生成
//! assert_eq!(v, vec![1, 2, 3]);
//! assert!(v.next_permutation());
//! assert_eq!(v, vec![1, 3, 2]);
//! assert!(v.next_permutation());
//! assert_eq!(v, vec![2, 1, 3]);
//! assert!(v.next_permutation());
//! assert_eq!(v, vec![2, 3, 1]);
//! assert!(v.next_permutation());
//! assert_eq!(v, vec![3, 1, 2]);
//! assert!(v.next_permutation());
//! assert_eq!(v, vec![3, 2, 1]);
//! assert!(!v.next_permutation()); // これ以上の順列はない
//! ```
//!
//! ## 競技プログラミングでの応用例
//!
//! ```
//! use next_permutation::NextPermutation;
//!
//! // 全順列での最適解探索
//! fn solve_with_permutation(items: &[i32]) -> i32 {
//!     let mut perm = items.to_vec();
//!     perm.sort(); // 辞書順最小から開始
//!     
//!     let mut best_score = i32::MIN;
//!     loop {
//!         // 現在の順列での評価
//!         let score = evaluate_permutation(&perm);
//!         best_score = best_score.max(score);
//!         
//!         if !perm.next_permutation() {
//!             break;
//!         }
//!     }
//!     best_score
//! }
//!
//! fn evaluate_permutation(perm: &[i32]) -> i32 {
//!     // 例：隣接要素の差の絶対値の和
//!     perm.windows(2).map(|w| (w[1] - w[0]).abs()).sum()
//! }
//!
//! let items = vec![1, 3, 2];
//! let result = solve_with_permutation(&items);
//! assert!(result > 0);
//! ```

/// next permutation です。
/// Next Permutation アルゴリズムを提供するトレイトです。
///
/// スライス型に対してnext permutationアルゴリズムを適用し、
/// 辞書順で次の順列を生成する機能を提供します。
/// 
/// [実装の参考資料](https://ngtkana.hatenablog.com/entry/2021/11/08/000209)
pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> NextPermutation for [T] {
    /// 数列を辞書順でひとつ進めます。
    ///
    /// 現在の順列を辞書順で次の順列に変更します。
    /// 次の順列が存在しない場合（つまり、現在の順列が辞書順で最大の場合）は
    /// 配列を変更せずに `false` を返します。
    ///
    /// # 戻り値
    ///
    /// - `true`: 次の順列に正常に進んだ場合
    /// - `false`: 次の順列が存在しない場合（辞書順最大に到達）
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
    ///
    /// ## 全順列の生成例
    ///
    /// ```
    /// use next_permutation::NextPermutation;
    ///
    /// let mut permutations = Vec::new();
    /// let mut current = vec![1, 2, 3];
    /// 
    /// loop {
    ///     permutations.push(current.clone());
    ///     if !current.next_permutation() {
    ///         break;
    ///     }
    /// }
    ///
    /// assert_eq!(permutations, vec![
    ///     vec![1, 2, 3],
    ///     vec![1, 3, 2],
    ///     vec![2, 1, 3],
    ///     vec![2, 3, 1],
    ///     vec![3, 1, 2],
    ///     vec![3, 2, 1],
    /// ]);
    /// ```
    ///
    /// ## 重複要素がある場合
    ///
    /// ```
    /// use next_permutation::NextPermutation;
    ///
    /// let mut v = vec![1, 1, 2];
    /// let mut perms = vec![v.clone()];
    ///
    /// while v.next_permutation() {
    ///     perms.push(v.clone());
    /// }
    ///
    /// assert_eq!(perms, vec![
    ///     vec![1, 1, 2],
    ///     vec![1, 2, 1],
    ///     vec![2, 1, 1],
    /// ]);
    /// ```
    ///
    /// ## 競技プログラミングでの使用例
    ///
    /// ```
    /// use next_permutation::NextPermutation;
    ///
    /// // n人の並び方での最適解を求める
    /// fn solve_arrangement_problem(scores: &[i32]) -> i32 {
    ///     let mut arrangement: Vec<usize> = (0..scores.len()).collect();
    ///     let mut best_score = 0;
    ///
    ///     loop {
    ///         // 現在の並び方での得点計算
    ///         let current_score: i32 = arrangement.iter()
    ///             .enumerate()
    ///             .map(|(pos, &person)| scores[person] * (pos as i32 + 1))
    ///             .sum();
    ///
    ///         best_score = best_score.max(current_score);
    ///
    ///         if !arrangement.next_permutation() {
    ///             break;
    ///         }
    ///     }
    ///     best_score
    /// }
    ///
    /// let scores = vec![3, 1, 4]; // 各人の基礎点
    /// let result = solve_arrangement_problem(&scores);
    /// // 最適な並び方での得点
    /// assert!(result > 0);
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
