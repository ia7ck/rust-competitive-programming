#[allow(clippy::many_single_char_names)]
pub fn z_algorithm<T: PartialEq>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    let mut z = vec![0; n];
    z[0] = n;
    let mut i = 1;
    let mut w = 0;
    while i < n {
        while i + w < n && a[w] == a[i + w] {
            w += 1;
        }
        z[i] = w;
        if w == 0 {
            i += 1;
            continue;
        }
        let mut j = 1;
        while j + z[j] < w {
            z[i + j] = z[j];
            j += 1;
        }
        i += j;
        w -= j;
    }
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
