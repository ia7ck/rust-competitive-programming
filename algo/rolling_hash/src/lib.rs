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
    hashes: Vec<u64>,
    pows: Vec<u64>,
}

impl<T> FromIterator<T> for RollingHash
where
    T: Into<u64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut hashes = vec![0];
        let mut pows = vec![1];
        for (i, x) in iter.into_iter().enumerate() {
            // hashes[i] * BASE + x
            hashes.push(calc_mod(mul(hashes[i], BASE) + x.into()));
            // pows[i] * BASE
            pows.push(calc_mod(mul(pows[i], BASE)));
        }
        Self { hashes, pows }
    }
}

impl RollingHash {
    /// 部分文字列のハッシュ値を返します。
    ///
    /// # Examples
    /// ```
    /// use std::iter::FromIterator;
    /// use rolling_hash::RollingHash;
    /// let rh = RollingHash::from_iter("abcxyzbcxy".bytes());
    /// assert_eq!(rh.hash(1..4), rh.hash(6..9)); // "bcx"
    /// ```
    pub fn hash(&self, range: ops::Range<usize>) -> u64 {
        let l = range.start;
        let r = range.end;
        assert!(l <= r);
        assert!(r <= self.hashes.len());
        // hashes[r] - hashes[l] * pows[r - l]
        calc_mod(self.hashes[r] + POSITIVIZER - mul(self.hashes[l], self.pows[r - l]))
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

    #[test]
    fn test_hash() {
        let rh1 = RollingHash::from_iter("abcd".bytes());
        let rh2 = RollingHash::from_iter("xxbcyy".bytes());
        assert_eq!(
            rh1.hash(1..3), // a"bc"d
            rh2.hash(2..4), // xx"bc"yy
        );
    }
}
