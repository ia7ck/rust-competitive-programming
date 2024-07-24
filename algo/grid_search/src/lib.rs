use std::ops::Range;

/// This `struct` is created by the [`around`] methods.
/// See its documentation for more.
///
/// [`around`]: fn.around.html
pub struct Around<'a> {
    y: usize,
    x: usize,
    y_range: Range<usize>,
    x_range: Range<usize>,
    directions: &'a [(isize, isize)],
}

/// `(y, x)` を基点とした周辺座標を yield するイテレータを作ります。
///
/// # Examples
/// 隣接 4 方向を走査する例です。
///
/// ```
/// use grid_search::around;
/// const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
/// let mut a = around(0, 1).y_range(0..3).x_range(0..4).directions(&NSEW);
/// assert_eq!(a.next(), Some((1, 1)));
/// assert_eq!(a.next(), Some((0, 2)));
/// assert_eq!(a.next(), Some((0, 0)));
/// assert_eq!(a.next(), None);
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
pub fn around<'a>(y: usize, x: usize) -> Around<'a> {
    Around {
        y,
        x,
        y_range: 0..std::usize::MAX,
        x_range: 0..std::usize::MAX,
        directions: &[],
    }
}

impl<'a> Around<'a> {
    /// 上下方向の範囲をセットします。デフォルトは `0..usize::MAX` です。
    pub fn y_range(self, r: Range<usize>) -> Self {
        Self { y_range: r, ..self }
    }
    /// 左右方向の範囲をセットします。デフォルトは `0..usize::MAX` です。
    pub fn x_range(self, r: Range<usize>) -> Self {
        Self { x_range: r, ..self }
    }
    /// 基点からの相対座標たちをセットします。デフォルトは空のスライスです。
    pub fn directions(self, dirs: &'a [(isize, isize)]) -> Self {
        Self {
            directions: dirs,
            ..self
        }
    }
}

impl<'a> Iterator for Around<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((&(dy, dx), rest)) = self.directions.split_first() {
            self.directions = rest;
            match (self.y.checked_add_signed(dy), self.x.checked_add_signed(dx)) {
                (Some(ny), Some(nx))
                    if self.y_range.contains(&self.y)
                        && self.x_range.contains(&self.x)
                        && self.y_range.contains(&ny)
                        && self.x_range.contains(&nx) =>
                {
                    return Some((ny, nx));
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    #[test]
    fn simple() {
        let mut a = around(5, 5).y_range(0..10).x_range(0..10).directions(&NSEW);
        assert_eq!(a.next(), Some((4, 5)));
        assert_eq!(a.next(), Some((6, 5)));
        assert_eq!(a.next(), Some((5, 6)));
        assert_eq!(a.next(), Some((5, 4)));
        assert_eq!(a.next(), None);
    }

    #[test]
    fn out_of_bounds_dest() {
        let mut a = around(1, 1).y_range(1..10).x_range(1..10).directions(&NSEW);
        assert_eq!(a.next(), Some((2, 1)));
        assert_eq!(a.next(), Some((1, 2)));
        assert_eq!(a.next(), None); // != (0, 1), (1, 0)
    }

    #[test]
    fn out_of_bounds_source() {
        let mut a = around(9, 10)
            .y_range(0..10)
            .x_range(0..10)
            .directions(&NSEW);
        assert_eq!(a.next(), None); // != (9, 9)
    }

    #[test]
    fn no_directions() {
        let mut a = around(5, 5).y_range(0..10).x_range(0..10);
        assert_eq!(a.next(), None);
    }
}
