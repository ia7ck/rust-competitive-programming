/// This `struct` is created by the [`around`] methods.
/// See its documentation for more.
///
/// [`around`]: fn.around.html
pub struct Around<'a> {
    y: usize,
    x: usize,
    y_range: std::ops::Range<usize>,
    x_range: std::ops::Range<usize>,
    directions: &'a [(isize, isize)],
    dir_idx: usize,
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
        dir_idx: 0,
    }
}

impl<'a> Around<'a> {
    /// 上下方向の範囲をセットします。デフォルトは `0..usize::MAX` です。
    pub fn y_range(self, y_rng: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            y_range: half_open_range(y_rng),
            ..self
        }
    }
    /// 左右方向の範囲をセットします。デフォルトは `0..usize::MAX` です。
    pub fn x_range(self, x_rng: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            x_range: half_open_range(x_rng),
            ..self
        }
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
        fn dest(u: usize, i: isize) -> Option<usize> {
            if i.is_positive() {
                u.checked_add(i as usize)
            } else {
                u.checked_sub((-i) as usize)
            }
        }
        while let Some(&(dy, dx)) = self.directions.get(self.dir_idx) {
            self.dir_idx += 1;
            if let Some(ny) = dest(self.y, dy) {
                if let Some(nx) = dest(self.x, dx) {
                    if self.y_range.contains(&self.y)
                        && self.x_range.contains(&self.x)
                        && self.y_range.contains(&ny)
                        && self.x_range.contains(&nx)
                    {
                        return Some((ny, nx));
                    }
                }
            }
        }
        None
    }
}

fn half_open_range(rng: impl std::ops::RangeBounds<usize>) -> std::ops::Range<usize> {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    let start = match rng.start_bound() {
        Included(&s) => s,
        Excluded(&s) => s + 1,
        Unbounded => 0,
    };
    let end = match rng.end_bound() {
        Included(&e) => e + 1,
        Excluded(&e) => e,
        Unbounded => std::usize::MAX,
    };
    start..end
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
    fn out_of_bounds() {
        let mut a = around(0, 0).y_range(0..10).x_range(0..10).directions(&NSEW);
        assert_eq!(a.next(), Some((1, 0)));
        assert_eq!(a.next(), Some((0, 1)));
        let mut a = around(9, 10)
            .y_range(0..10)
            .x_range(0..10)
            .directions(&NSEW);
        assert_eq!(a.next(), None);
    }

    #[test]
    fn no_directions() {
        let mut a = around(5, 5).y_range(0..10).x_range(0..10);
        assert_eq!(a.next(), None);
    }
}
