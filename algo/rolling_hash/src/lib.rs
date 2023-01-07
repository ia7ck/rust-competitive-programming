use std::{iter::FromIterator, ops};

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

impl<T> FromIterator<T> for RollingHash
where
    T: Into<u64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut h = vec![0];
        let mut p = vec![1];
        for (i, x) in iter.into_iter().enumerate() {
            h.push(calc_mod(mul(h[i], BASE) + x.into()));
            p.push(calc_mod(mul(p[i], BASE)));
        }
        Self { h, p }
    }
}

impl RollingHash {
    /// `range` が指す範囲の部分文字列のハッシュ値を返します。
    ///
    /// # Examples
    /// ```
    /// use std::iter::FromIterator;
    /// use rolling_hash::RollingHash;
    /// let rh = RollingHash::from_iter("abcxyzbcxy".bytes());
    /// assert_eq!(rh.get(1..4), rh.get(6..9)); // "bcx"
    /// ```
    pub fn get(&self, range: ops::Range<usize>) -> u64 {
        let l = range.start;
        let r = range.end;
        calc_mod(self.h[r] + POSITIVIZER - mul(self.h[l], self.p[r - l]))
    }
    /// 2 つの文字列を連結した文字列のハッシュ値を返します。
    ///
    /// # Examples
    /// ```
    /// use std::iter::FromIterator;
    /// use rolling_hash::RollingHash;
    /// let rh = RollingHash::from_iter("abcdexyz".bytes());
    /// let left = rh.get(0..3);  // "abc"
    /// let right = rh.get(5..8); // "xyz"
    /// assert_eq!(rh.connect(0..3, 5..8), RollingHash::from_iter("abcxyz".bytes()).get(0..6));
    /// ```
    pub fn connect(&self, l_range: ops::Range<usize>, r_range: ops::Range<usize>) -> u64 {
        assert!(l_range.end <= r_range.start);
        calc_mod(mul(self.get(l_range), self.p[r_range.len()]) + self.get(r_range))
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
        let bytes: Vec<u8> = "abxy".bytes().collect();
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(1, 20);
            let s: Vec<u8> = (0..n).map(|_| *bytes.choose(&mut rng).unwrap()).collect();
            let rh = RollingHash::from_iter(s.clone());
            for i in 0..n {
                for j in i..n {
                    for ii in j..n {
                        for jj in ii..n {
                            let t: Vec<u8> = s[i..j].iter().chain(&s[ii..jj]).cloned().collect();
                            assert_eq!(
                                rh.connect(i..j, ii..jj),
                                RollingHash::from_iter(t.clone()).get(0..t.len())
                            );
                        }
                    }
                }
            }
        }
    }
}
