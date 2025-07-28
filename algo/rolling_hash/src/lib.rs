//! Rolling Hash（ローリングハッシュ）による高速文字列処理ライブラリです。
//!
//! Rolling Hash は文字列の部分文字列のハッシュ値を O(1) で計算できる技法です。
//! 文字列の比較、パターンマッチング、部分文字列の検索などを高速化できます。
//! 
//! # アルゴリズムの概要
//!
//! 文字列 S に対して、前計算として各位置までの累積ハッシュ値を計算しておくことで、
//! 任意の部分文字列 S[l..r] のハッシュ値を O(1) で求められます。
//!
//! # ハッシュ衝突について
//!
//! ハッシュ値が同じでも実際の文字列が異なる場合（ハッシュ衝突）があります。
//! このライブラリでは 2^61-1 を法とする大きな法を使用して衝突確率を下げていますが、
//! 完全には回避できません。競技プログラミングでは通常問題ありませんが、
//! 重要な用途では複数のハッシュ関数を併用することを推奨します。
//!
//! # 主な機能
//!
//! - **前計算**: O(n) で文字列全体のハッシュ値を計算
//! - **部分文字列ハッシュ**: O(1) で任意の部分文字列のハッシュ値を取得
//! - **部分文字列判定**: ある文字列が別の文字列の部分文字列かを高速判定
//!
//! # 使用例
//!
//! ```
//! use rolling_hash::RollingHash;
//!
//! let text = RollingHash::from_iter("abcdefg".bytes());
//! let pattern = RollingHash::from_iter("cde".bytes());
//!
//! // 部分文字列のハッシュ値を比較
//! assert_eq!(pattern.hash(0..3), text.hash(2..5)); // "cde" == "cde"
//!
//! // 部分文字列判定
//! assert!(pattern.is_substring(&text));
//! ```
//!
//! # 競技プログラミングでの応用
//!
//! - **文字列検索**: KMP法などの代替として高速な文字列検索
//! - **回文判定**: 文字列とその逆向きのハッシュ値を比較
//! - **最長共通部分文字列**: 複数文字列間での共通部分の効率的な検出
//! - **文字列の一意性判定**: 重複する部分文字列の検出
//!
//! # 計算量
//!
//! - 前計算: O(n) (n: 文字列長)
//! - 部分文字列ハッシュ取得: O(1)
//! - 部分文字列判定: O(m) (m: 検索対象文字列長)
//! - 空間計算量: O(n)

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
///
/// # Examples
///
/// 基本的な使用方法:
/// ```
/// use rolling_hash::RollingHash;
///
/// let rh = RollingHash::from_iter("hello".bytes());
/// let hash_full = rh.hash(0..5);    // "hello" 全体
/// let hash_part = rh.hash(1..4);    // "ell" 部分
/// ```
#[derive(Debug, Clone)]
pub struct RollingHash {
    xs: Vec<u64>,
    hashes: Vec<u64>,
    pows: Vec<u64>,
}

impl<T> FromIterator<T> for RollingHash
where
    T: Into<u64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let xs = iter.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        Self::new(&xs)
    }
}

impl RollingHash {
    /// 数値配列から Rolling Hash を構築します。
    ///
    /// # 引数
    ///
    /// - `xs`: ハッシュ化する数値の配列。通常は文字のバイト値
    ///
    /// # 戻り値
    ///
    /// 構築された `RollingHash` インスタンス
    ///
    /// # 計算量
    ///
    /// O(n) (n = `xs.len()`)
    ///
    /// # Examples
    ///
    /// ```
    /// use rolling_hash::RollingHash;
    ///
    /// // 文字列から構築
    /// let rh1 = RollingHash::new(&[65, 66, 67]); // "ABC"
    ///
    /// // または FromIterator を使用
    /// let rh2 = RollingHash::from_iter("ABC".bytes());
    /// ```
    pub fn new(xs: &[u64]) -> Self {
        let n = xs.len();
        let xs = xs.to_vec();
        let mut hashes = vec![0; n + 1];
        let mut pows = vec![1; n + 1];
        for (i, &x) in xs.iter().enumerate() {
            // hashes[i + 1] = hashes[i] * BASE + x
            hashes[i + 1] = calc_mod(mul(hashes[i], BASE) + x);
            // pows[i + 1] = pows[i] * BASE
            pows[i + 1] = calc_mod(mul(pows[i], BASE));
        }
        Self { xs, hashes, pows }
    }

    /// 文字列の長さを返します。
    pub fn len(&self) -> usize {
        self.xs.len()
    }

    /// 文字列が空かどうかを返します。
    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    /// 指定位置の文字（数値）を返します。
    ///
    /// # 引数
    ///
    /// - `i`: 取得する位置のインデックス
    ///
    /// # パニック条件
    ///
    /// `i >= self.len()` の場合にパニックします。
    pub fn at(&self, i: usize) -> u64 {
        assert!(i < self.len());
        self.xs[i]
    }

    /// 部分文字列のハッシュ値を返します。
    ///
    /// 指定された範囲の部分文字列のハッシュ値を O(1) で計算します。
    ///
    /// # 引数
    ///
    /// - `range`: 部分文字列の範囲（`start..end` 形式）
    ///
    /// # 戻り値
    ///
    /// 部分文字列のハッシュ値
    ///
    /// # パニック条件
    ///
    /// - `range.start > range.end` の場合
    /// - `range.end > self.len()` の場合
    ///
    /// # Examples
    ///
    /// ```
    /// use rolling_hash::RollingHash;
    ///
    /// let rh = RollingHash::from_iter("hello".bytes());
    /// let full_hash = rh.hash(0..5);    // "hello"
    /// let part_hash = rh.hash(1..4);    // "ell"
    /// let char_hash = rh.hash(0..1);    // "h"
    /// ```
    pub fn hash(&self, range: ops::Range<usize>) -> u64 {
        let l = range.start;
        let r = range.end;
        assert!(l <= r);
        assert!(r <= self.hashes.len());
        // hashes[r] - hashes[l] * pows[r - l]
        // = (xs[0] * BASE ^ (r - 1) + xs[1] * BASE ^ (r - 2) + ... + xs[r - 1])
        //   - (xs[0] * BASE ^ (l - 1) + xs[1] * BASE ^ (l - 2) + ... + xs[l - 1]) * BASE ^ (r - l)
        // = xs[l] * BASE ^ (r - l - 1) + xs[l + 1] * BASE ^ (r - l - 2) + ... + xs[r - 1]
        calc_mod(self.hashes[r] + POSITIVIZER - mul(self.hashes[l], self.pows[r - l]))
    }

    /// self が other の部分文字列かどうかを返します。
    ///
    /// ハッシュ値の比較により、self の文字列が other の文字列の
    /// 部分文字列として含まれているかを判定します。
    ///
    /// # 引数
    ///
    /// - `other`: 検索対象となる文字列の `RollingHash`
    ///
    /// # 戻り値
    ///
    /// self が other の部分文字列の場合 `true`、そうでなければ `false`
    ///
    /// # 計算量
    ///
    /// O(other.len())
    ///
    /// # 注意
    ///
    /// ハッシュ値の一致により判定するため、極稀にハッシュ衝突による
    /// 偽陽性（false positive）が発生する可能性があります。
    ///
    /// # Examples
    /// ```
    /// use rolling_hash::RollingHash;
    /// let rh1 = RollingHash::from_iter("abcd".bytes());
    /// let rh2 = RollingHash::from_iter("xxabcdyy".bytes());
    /// assert!(rh1.is_substring(&rh2));
    ///
    /// // より実用的な例：パターン検索
    /// let pattern = RollingHash::from_iter("world".bytes());
    /// let text = RollingHash::from_iter("hello world!".bytes());
    /// assert!(pattern.is_substring(&text));
    ///
    /// // 存在しないパターン
    /// let missing = RollingHash::from_iter("xyz".bytes());
    /// assert!(!missing.is_substring(&text));
    /// ```
    // 出現位置をすべて返すようにしたほうがいいかも
    pub fn is_substring(&self, other: &Self) -> bool {
        for j in 0..other.len() {
            if j + self.len() > other.len() {
                break;
            }
            if self.hash(0..self.len()) == other.hash(j..(j + self.len())) {
                return true;
            }
        }
        false
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

    #[test]
    fn test_is_substring() {
        let rh1 = RollingHash::from_iter("xyz".bytes());
        let rh2 = RollingHash::from_iter("abcxyz".bytes());
        assert!(rh1.is_substring(&rh2));
    }
}
