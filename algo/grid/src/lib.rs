/// グリッドグラフで現在位置の周辺を走査したいときに使えます。
pub struct Adjacent<'a> {
    base_point: (usize, usize),
    height: usize,
    width: usize,
    direction: &'a [(isize, isize)],
    idx: usize,
}

impl<'a> Adjacent<'a> {
    ///
    /// - `base_point`: 基点となる座標
    /// - `height`: グリッドの高さ
    /// - `width`: グリッドの幅
    /// - `direction`: 基点からの相対座標たち
    ///
    /// 隣接 4 方向を走査する例です。
    ///
    /// # Examples
    /// ```
    /// use grid::Adjacent;
    /// const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    /// let adjs = Adjacent::new((0, 1), 3, 4, &NSEW).collect::<Vec<_>>();
    /// assert_eq!(vec![(1, 1), (0, 2), (0, 0)], adjs);
    /// // .x..
    /// // ....
    /// // ....
    /// //
    /// //  |
    /// //  v
    /// //
    /// // x.x.
    /// // .x..
    /// // ....
    /// ```
    pub fn new(
        base_point: (usize, usize),
        height: usize,
        width: usize,
        direction: &'a [(isize, isize)],
    ) -> Self {
        Self {
            base_point,
            height,
            width,
            direction,
            idx: 0,
        }
    }
}

impl<'a> Iterator for Adjacent<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let (i, j) = self.base_point;
        while let Some((di, dj)) = self.direction.get(self.idx) {
            self.idx += 1;
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if 0 <= ni && ni < self.height as isize && 0 <= nj && nj < self.width as isize {
                return Some((ni as usize, nj as usize));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adj4() {
        const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        const H: usize = 3;
        const W: usize = 4;
        let i = 0;
        let j = 1;
        let adjs = Adjacent::new((i, j), H, W, &NSEW).collect::<Vec<_>>();
        assert_eq!(vec![(1, 1), (0, 2), (0, 0)], adjs);
    }
}
