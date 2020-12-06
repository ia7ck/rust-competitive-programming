const MASK30: u64 = (1 << 30) - 1;
const MASK31: u64 = (1 << 31) - 1;
const MOD: u64 = (1 << 61) - 1;
const MASK61: u64 = (1 << 61) - 1;
const POSITIVIZER: u64 = MOD * 4;
const BASE: u64 = 1_000_000_000 + 9;

/// Rolling Hash です。O(文字列長) の前計算をしたうえで、部分文字列のハッシュ値を O(1) で計算します。
///
/// [実装の参考資料](https://qiita.com/keymoon/items/11fac5627672a6d6a9f6)
pub struct RollingHash {
    h: Vec<u64>,
    p: Vec<u64>,
}
impl RollingHash {
    pub fn new(s: &[u64]) -> Self {
        let n = s.len();
        let mut h = vec![0; n + 1];
        let mut p = vec![0; n + 1];
        p[0] = 1;
        for i in 0..n {
            h[i + 1] = calc_mod(mul(h[i], BASE) + s[i]);
            p[i + 1] = calc_mod(mul(p[i], BASE));
        }
        Self { h, p }
    }
    /// `range` が指す範囲の部分文字列のハッシュ値を返します。
    ///
    /// # Examples
    /// ```
    /// use rolling_hash::RollingHash;
    /// let s = "abcxyzbcxy".chars().map(|c| c as u64).collect::<Vec<_>>();
    /// let rh = RollingHash::new(&s);
    /// assert_eq!(rh.get(1..4), rh.get(6..9)); // "bcx"
    /// ```
    pub fn get(&self, range: std::ops::Range<usize>) -> u64 {
        let l = range.start;
        let r = range.end;
        calc_mod(self.h[r] + POSITIVIZER - mul(self.h[l], self.p[r - l]))
    }
    /// 2 つの文字列を連結したときのハッシュ値を返します。
    ///
    /// `left`, `right` はそれぞれ連結前の文字列のハッシュ値です。`right_len` は末尾にくっつける側の文字列の長さです。
    /// # Examples
    /// ```
    /// use rolling_hash::RollingHash;
    /// let s = "abcdexyz".chars().map(|c| c as u64).collect::<Vec<_>>();
    /// let rh = RollingHash::new(&s);
    /// let left = rh.get(0..5);  // "abcde"
    /// let right = rh.get(5..8); // "xyz"
    /// assert_eq!(rh.connect(left, right, 3), rh.get(0..8));
    /// ```
    pub fn connect(&self, left: u64, right: u64, right_len: usize) -> u64 {
        calc_mod(mul(left, self.p[right_len]) + right)
    }
}
fn mul(a: u64, b: u64) -> u64 {
    let au = a >> 31;
    let ad = a & MASK31;
    let bu = b >> 31;
    let bd = b & MASK31;
    let mid = ad * bu + au * bd;
    let midu = mid >> 30;
    let midd = mid & MASK30;
    au * bu * 2 + midu + (midd << 31) + ad * bd
}
fn calc_mod(x: u64) -> u64 {
    let xu = x >> 61;
    let xd = x & MASK61;
    let mut res = xu + xd;
    if res >= MOD {
        res -= MOD;
    }
    res
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
            let n = rng.gen_range(1, 50);
            let s: String = (0..n).map(|_| *chars.choose(&mut rng).unwrap()).collect();
            let rh = RollingHash::new(&s.chars().map(|c| c as u64).collect::<Vec<_>>());
            for i in 0..n {
                for j in i..n {
                    let t: String = format!("{}{}", &s[0..i], &s[j..n]);
                    let t = t.chars().map(|c| c as u64).collect::<Vec<_>>();
                    assert_eq!(
                        rh.connect(rh.get(0..i), rh.get(j..n), n - j),
                        RollingHash::new(&t).get(0..t.len())
                    );
                }
            }
        }
    }
}
