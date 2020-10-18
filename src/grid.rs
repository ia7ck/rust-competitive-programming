/// グリッドグラフで現在位置の周辺を走査したいときに使えます。
pub struct Adjacent<I> {
    position: (usize, usize),
    h: usize,
    w: usize,
    direction: I,
}

impl<I> Adjacent<I>
where
    I: Iterator<Item = (isize, isize)>,
{
    /// 隣接 4 方向を走査する例です。
    /// # Examples
    /// ```
    /// use crate::rust_competitive_programming::grid::Adjacent;
    /// const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    /// let adjs = Adjacent::new((0, 1), 3, 4, NSEW.iter().copied()).collect::<Vec<_>>();
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
    pub fn new(position: (usize, usize), h: usize, w: usize, direction: I) -> Self {
        Self {
            position,
            h,
            w,
            direction,
        }
    }
}

impl<I> Iterator for Adjacent<I>
where
    I: Iterator<Item = (isize, isize)>,
{
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        while let Some((di, dj)) = self.direction.next() {
            let (i, j) = self.position;
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if 0 <= ni && ni < self.h as isize && 0 <= nj && nj < self.w as isize {
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
        let adjs = Adjacent::new((i, j), H, W, NSEW.iter().copied()).collect::<Vec<_>>();
        assert_eq!(vec![(1, 1), (0, 2), (0, 0)], adjs);
    }
}
