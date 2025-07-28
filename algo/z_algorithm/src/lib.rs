//! Z Algorithm（Z アルゴリズム）による文字列処理ライブラリです。
//!
//! Z Algorithm は文字列の各位置から始まる部分文字列と、文字列全体との
//! 最長共通接頭辞の長さを効率的に計算するアルゴリズムです。
//! パターンマッチング、回文判定、周期性の検出などに応用できます。
//!
//! # アルゴリズムの概要
//!
//! 文字列 S に対して、配列 Z を構築します。Z[i] は S[i..] と S との
//! 最長共通接頭辞の長さを表します。つまり、位置 i から始まる部分文字列が
//! 文字列の先頭からどれだけ一致するかを示します。
//!
//! # 主な応用
//!
//! - **パターンマッチング**: パターン + セパレータ + テキストの形で結合し、
//!   パターンの長さと一致する Z 値を持つ位置を探すことで、パターンの出現位置を特定
//! - **回文判定**: 文字列を逆順にしたものと結合して Z Algorithm を適用
//! - **周期性検出**: 文字列の周期的パターンの検出
//! - **接頭辞の重複検出**: 文字列内での接頭辞の再出現位置を特定
//!
//! # 使用例
//!
//! ```
//! use z_algorithm::z_algorithm;
//!
//! let s = "abcabc".chars().collect::<Vec<char>>();
//! let z = z_algorithm(&s);
//! // z[0] = 6 (全体)
//! // z[3] = 3 ("abc" が再出現)
//! assert_eq!(z[0], 6);
//! assert_eq!(z[3], 3);
//! ```
//!
//! # 計算量
//!
//! - 時間計算量: O(n) (n: 文字列長)
//! - 空間計算量: O(n)

/// `z[i]`: `a[i..]` と `a` との最長共通接頭辞の長さ、を返します。
///
/// [実装の参考資料](https://snuke.hatenablog.com/entry/2014/12/03/214243)
///
/// # 引数
///
/// - `a`: 対象となる配列（通常は文字の配列）
///
/// # 戻り値
///
/// Z 配列。`z[i]` は位置 i から始まる部分配列と元の配列との最長共通接頭辞の長さ
///
/// # 計算量
///
/// O(n) (n = `a.len()`)
///
/// # Examples
/// ```
/// use z_algorithm::z_algorithm;
///
/// let a = "abcabc".chars().collect::<Vec<char>>();
/// let z = z_algorithm(&a);
/// assert_eq!(z[0], 6); // abcabc（全体）
/// assert_eq!(z[1], 0); // bcabc
/// assert_eq!(z[2], 0); // cabc
/// assert_eq!(z[3], 3); // abc（先頭と一致）
/// assert_eq!(z[4], 0); // bc
/// assert_eq!(z[5], 0); // c
/// ```
///
/// # 実用例: パターンマッチング
/// ```
/// use z_algorithm::z_algorithm;
///
/// // パターン "abc" をテキスト "xyzabcdefabc" から探す
/// let pattern = "abc";
/// let text = "xyzabcdefabc";
/// let separator = "$"; // パターンとテキストの区切り文字
/// 
/// let combined = format!("{}{}{}", pattern, separator, text);
/// let chars: Vec<char> = combined.chars().collect();
/// let z = z_algorithm(&chars);
/// 
/// let pattern_len = pattern.len();
/// let offset = pattern_len + separator.len(); // パターン + セパレータの長さ
/// 
/// // Z値がパターン長と一致する位置を探す
/// for i in offset..z.len() {
///     if z[i] == pattern_len {
///         let pos_in_text = i - offset;
///         println!("パターンが位置 {} で見つかりました", pos_in_text);
///     }
/// }
/// ```
///
/// # 実用例: 最長回文接頭辞
/// ```
/// use z_algorithm::z_algorithm;
///
/// // 文字列の先頭から始まる最長回文の長さを求める
/// fn longest_palindrome_prefix(s: &str) -> usize {
///     let chars: Vec<char> = s.chars().collect();
///     let reversed: Vec<char> = s.chars().rev().collect();
///     let separator = vec!['$'];
///     
///     let mut combined = reversed;
///     combined.extend_from_slice(&separator);
///     combined.extend_from_slice(&chars);
///     
///     let z = z_algorithm(&combined);
///     let n = chars.len();
///     let offset = n + 1;
///     
///     let mut max_len = 0;
///     for i in 0..n {
///         let z_val = z[offset + i];
///         if offset + i + z_val == combined.len() {
///             max_len = max_len.max(z_val);
///         }
///     }
///     max_len
/// }
///
/// assert_eq!(longest_palindrome_prefix("abacaba"), 7); // 全体が回文
/// assert_eq!(longest_palindrome_prefix("abcdef"), 1);  // 先頭の1文字のみ
/// ```
#[allow(clippy::many_single_char_names)]
pub fn z_algorithm<T>(a: &[T]) -> Vec<usize>
where
    T: PartialEq + std::fmt::Debug,
{
    let n = a.len();
    let mut z = vec![0; n];
    let mut i = 0;
    for j in 1..n {
        if j + z[j - i] < i + z[i] {
            debug_assert_eq!(a[j..(j + z[j - i])], a[..z[j - i]]);
            z[j] = z[j - i];
        } else {
            let start = j + (i + z[i]).saturating_sub(j);
            debug_assert_eq!(a[j..start], a[..(start - j)]);
            let end = (start..n).find(|&k| a[k - j] != a[k]).unwrap_or(n);
            debug_assert_eq!(a[j..end], a[..(end - j)]);
            z[j] = end - j;
            i = j;
        }
    }
    z[0] = n;
    z
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn test() {
        let chars = ['a', 'b', 'x', 'y'];
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(1, 100);
            let s = (0..n)
                .map(|_| *chars.choose(&mut rng).unwrap())
                .collect::<Vec<_>>();
            let z = z_algorithm(&s);
            for i in 0..n {
                assert_eq!(z[i], lcp(&s, &s[i..]));
            }
        }
    }

    fn lcp(a: &[char], b: &[char]) -> usize {
        let mut i = 0;
        while i < a.len() && i < b.len() {
            if a[i] != b[i] {
                break;
            }
            i += 1;
        }
        i
    }
}
