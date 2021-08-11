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
/// # Examples
/// ```
/// use suffix_array::suffix_array;
/// let s: Vec<char> = "mississippi".chars().collect();
/// let sa = suffix_array(&s);
/// assert_eq!(sa, vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
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
pub fn suffix_array(s: &[char]) -> Vec<usize> {
    let mut s = s.to_vec();
    s.push('$');
    let sorted_shifts = sort_cyclic_shifts(&s);
    sorted_shifts[1..].to_vec()
}

/// LCP 配列を O(|s|) で求めます。
///
/// 返り値は長さ `s.len() - 1` のベクタ `lcp` であり `lcp[i]` := `s[sa[i]..]` と `s[sa[i+1]..]` との最長共通接頭辞の長さ、です。
///
/// # Examples
/// ```
/// use suffix_array::{suffix_array, lcp_array};
/// let s: Vec<char> = "mississippi".chars().collect();
/// let sa = suffix_array(&s);
/// let lcp = lcp_array(&s, &sa);
/// assert_eq!(lcp, vec![1, 1, 4, 0, 0, 1, 0, 2, 1, 3]);
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
