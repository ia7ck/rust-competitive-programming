//! 接尾辞配列（Suffix Array）とLCP配列を計算するライブラリです。
//!
//! 接尾辞配列は文字列の全ての接尾辞を辞書順でソートした際のインデックス配列です。
//! 文字列検索、最長共通部分文字列の検出、文字列マッチングなど様々な文字列処理
//! アルゴリズムの基盤として使用されます。
//!
//! # 主な機能
//!
//! - **接尾辞配列構築**: O(n log n) で文字列の接尾辞配列を構築
//! - **LCP配列計算**: O(n) で隣接する接尾辞間の最長共通接頭辞長を計算
//!
//! # 使用例
//!
//! ```
//! use suffix_array::{suffix_array, lcp_array};
//!
//! let s: Vec<char> = "banana".chars().collect();
//! let sa = suffix_array(&s);
//! let lcp = lcp_array(&s, &sa);
//!
//! // 接尾辞配列: [5, 3, 1, 0, 4, 2]
//! // 対応する接尾辞: ["a", "ana", "anana", "banana", "na", "nana"]
//! 
//! // 文字列検索の例
//! let pattern: Vec<char> = "ana".chars().collect();
//! // sa を使って pattern の出現位置を効率的に検索可能
//! ```
//!
//! # 競技プログラミングでの応用
//!
//! - **文字列検索**: パターンマッチング
//! - **最長共通部分文字列**: 複数文字列間の共通部分検出
//! - **回文検索**: 回文の効率的な検出
//! - **辞書順k番目の部分文字列**: 部分文字列の辞書順列挙
//!
//! # 計算量
//!
//! - 接尾辞配列構築: O(n log n)
//! - LCP配列計算: O(n)
//! - 空間計算量: O(n)

fn sort_cyclic_shifts(s: &[char]) -> Vec<usize> {
    let n = s.len();
    const ALPHABET: usize = 256;
    let mut cnt = vec![0; n.max(ALPHABET)];
    for &ch in s {
        cnt[ch as usize] += 1;
    }
    for i in 1..ALPHABET {
        cnt[i] += cnt[i - 1];
    }
    let mut p = vec![!0; n];
    // p[i] := the index of the i-th substring (starting at i and with length 2^k) in the sorted order
    for (i, &ch) in s.iter().enumerate().rev() {
        cnt[ch as usize] -= 1;
        p[cnt[ch as usize]] = i;
    }
    let mut c = vec![!0; n];
    // c[i] := the equivalence class to which the substring belongs
    c[p[0]] = 0;
    let mut classes = 1;
    for w in p.windows(2) {
        let (prev, cur) = (w[0], w[1]);
        if s[prev] != s[cur] {
            classes += 1;
        }
        c[cur] = classes - 1;
    }
    for h in (0..).take_while(|&h| 1 << h < n) {
        let pn: Vec<usize> = p.iter().copied().map(|x| (n + x - (1 << h)) % n).collect();
        #[allow(clippy::needless_range_loop)]
        for i in 0..classes {
            cnt[i] = 0;
        }
        for &x in &pn {
            cnt[c[x]] += 1;
        }
        for i in 1..classes {
            cnt[i] += cnt[i - 1];
        }
        for &x in pn.iter().rev() {
            cnt[c[x]] -= 1;
            p[cnt[c[x]]] = x;
        }
        let mut cn = vec![!0; n];
        cn[p[0]] = 0;
        classes = 1;
        for w in p.windows(2) {
            let (prev, cur) = (
                (c[w[0]], c[(w[0] + (1 << h)) % n]),
                (c[w[1]], c[(w[1] + (1 << h)) % n]),
            );
            if prev != cur {
                classes += 1;
            }
            cn[w[1]] = classes - 1;
        }
        c = cn;
    }
    p
}

/// 文字列 `s` の suffix array を O(|s|log|s|) で求めます。
///
/// 返り値は `s.len()` を `n` としたとき、長さ `n` のベクタ `sa` であり次の条件を満たすものです。
///
/// - `s[sa[i]..]` が `s` の `n` 個ある suffix のうち辞書順で `i` 番目である
///
/// original: [CP-Algorithms](https://cp-algorithms.com/string/suffix-array.html)
///
/// # 引数
///
/// - `s`: 接尾辞配列を構築する対象の文字列（文字の配列として表現）
///
/// # 戻り値
///
/// 接尾辞配列。`sa[i]` は辞書順で i 番目の接尾辞の開始位置を表す
///
/// # 計算量
///
/// O(n log n) (n = `s.len()`)
///
/// # Examples
/// ```
/// use suffix_array::suffix_array;
/// let s: Vec<char> = "mississippi".chars().collect();
/// let sa = suffix_array(&s);
/// assert_eq!(sa, vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
/// // 対応する接尾辞（辞書順）:
/// // i
/// // ippi
/// // issippi
/// // ississippi
/// // mississippi
/// // pi
/// // ppi
/// // sippi
/// // sissippi
/// // ssippi
/// // ssissippi
/// ```
///
/// # 実用例: 文字列検索
/// ```
/// use suffix_array::suffix_array;
/// 
/// let text: Vec<char> = "abracadabra".chars().collect();
/// let sa = suffix_array(&text);
/// 
/// // パターン "abr" を検索する例
/// let pattern: Vec<char> = "abr".chars().collect();
/// 
/// // 二分探索で pattern を持つ接尾辞の範囲を見つけられる
/// // （実際の実装は省略）
/// ```
pub fn suffix_array(s: &[char]) -> Vec<usize> {
    let mut s = s.to_vec();
    s.push('$');
    let sorted_shifts = sort_cyclic_shifts(&s);
    sorted_shifts[1..].to_vec()
}

/// LCP 配列を O(|s|) で求めます。
///
/// LCP 配列は隣接する接尾辞間の最長共通接頭辞（Longest Common Prefix）の長さを
/// 格納する配列です。返り値は長さ `s.len() - 1` のベクタ `lcp` であり 
/// `lcp[i]` := `s[sa[i]..]` と `s[sa[i+1]..]` との最長共通接頭辞の長さ、です。
///
/// # 引数
///
/// - `s`: 対象の文字列（文字の配列として表現）
/// - `sa`: `s` の接尾辞配列（`suffix_array` 関数で得られるもの）
///
/// # 戻り値
///
/// LCP配列。`lcp[i]` は接尾辞配列の隣接する要素間の最長共通接頭辞長
///
/// # 計算量
///
/// O(n) (n = `s.len()`)
///
/// # Examples
/// ```
/// use suffix_array::{suffix_array, lcp_array};
/// let s: Vec<char> = "mississippi".chars().collect();
/// let sa = suffix_array(&s);
/// let lcp = lcp_array(&s, &sa);
/// assert_eq!(lcp, vec![1, 1, 4, 0, 0, 1, 0, 2, 1, 3]);
/// ```
///
/// # 実用例: 最長重複部分文字列
/// ```
/// use suffix_array::{suffix_array, lcp_array};
/// 
/// let s: Vec<char> = "banana".chars().collect();
/// let sa = suffix_array(&s);
/// let lcp = lcp_array(&s, &sa);
/// 
/// // LCP配列の最大値が最長重複部分文字列の長さ
/// let max_lcp = lcp.iter().max().unwrap_or(&0);
/// assert_eq!(*max_lcp, 3); // "ana" が最長重複部分文字列
/// ```
pub fn lcp_array(s: &[char], sa: &[usize]) -> Vec<usize> {
    let n = sa.len();
    if n == 1 {
        return vec![];
    }
    let mut rank = vec![!0; n];
    for i in 0..n {
        rank[sa[i]] = i;
    }
    let mut k = 0;
    let mut lcp = vec![0; n - 1];
    for i in 0..n {
        if rank[i] + 1 == n {
            k = 0;
            continue;
        }
        if k >= 1 {
            k -= 1;
        }
        let j = sa[rank[i] + 1];
        while i + k < n && j + k < n && s[i + k] == s[j + k] {
            k += 1;
        }
        lcp[rank[i]] = k;
    }
    lcp
}

#[cfg(test)]
mod tests {
    use crate::{lcp_array, suffix_array};

    #[test]
    fn test_small() {
        let tests = vec![
            ("a", vec![0], vec![]),
            ("aa", vec![1, 0], vec![1]),
            ("abc", vec![0, 1, 2], vec![0, 0]),
            ("aaba", vec![3, 0, 1, 2], vec![1, 1, 0]),
            ("abaab", vec![2, 3, 0, 4, 1], vec![1, 2, 0, 1]),
            ("dabbb", vec![1, 4, 3, 2, 0], vec![0, 1, 2, 0]),
        ];
        for (s, sa, lcp) in tests {
            let s: Vec<char> = s.chars().collect();
            assert_eq!(suffix_array(&s), sa);
            assert_eq!(lcp_array(&s, &suffix_array(&s)), lcp);
        }
    }
}
