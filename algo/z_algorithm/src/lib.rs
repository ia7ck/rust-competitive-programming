///
/// `z[i]`: `a[i..]` と `a` との最長共通接頭辞の長さ、を返します。
///
/// # Examples
/// ```
/// use z_algorithm::z_algorithm;
///
/// let a = "abcabc".chars().collect::<Vec<char>>();
/// let z = z_algorithm(&a);
/// assert_eq!(z[0], 6); // abcabc
/// assert_eq!(z[1], 0); // bcabc
/// assert_eq!(z[2], 0); // cabc
/// assert_eq!(z[3], 3); // abc
/// assert_eq!(z[4], 0); // bc
/// assert_eq!(z[5], 0); // c
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
