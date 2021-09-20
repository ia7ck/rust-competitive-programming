use std::ops::Range;

/// __注意⚠__ この実装は遅いので time limit の厳しい問題には代わりに ACL のセグメントツリーを使うこと。
///
/// セグメントツリーです。
pub struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    /// 長さ `n` の列を初期値 `e` で初期化します。
    ///
    /// `multiply` は fold に使う二項演算です。
    pub fn new(n: usize, e: T, multiply: F) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![e.clone(); n * 2 - 1],
            e,
            multiply,
        }
    }

    /// 列の `i` 番目の要素を取得します。
    pub fn get(&self, i: usize) -> &T {
        &self.dat[i + self.n - 1]
    }

    /// 列の `i` 番目の要素を `x` で更新します。
    pub fn update(&mut self, i: usize, x: T) {
        let mut k = i + self.n - 1;
        self.dat[k] = x;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.multiply)(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
        }
    }

    /// `range` が `l..r` として、`multiply(l番目の要素, multiply(..., multiply(r-1番目の要素, r番目の要素)))` の値を返します。
    ///
    /// 実際のアルゴリズムは、結合法則を使って `1 + (2 + (3 + 4))` ではなく `(1 + 2) + (3 + 4)` のように計算しています。
    pub fn fold(&self, range: Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(&self, range: &Range<usize>, i: usize, i_range: Range<usize>) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e.clone();
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i].clone();
        }
        let m = (i_range.start + i_range.end) / 2;
        (self.multiply)(
            &self._fold(range, i * 2 + 1, i_range.start..m),
            &self._fold(range, i * 2 + 2, m..i_range.end),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::SegmentTree;

    #[test]
    fn single_element() {
        let mut seg = SegmentTree::new(1, 0, |a, b| a + b);
        assert_eq!(seg.get(0), &0);
        seg.update(0, 42);
        assert_eq!(seg.get(0), &42);
    }
}
