use ::std::cmp::Ordering;

/// 2次元座標の偏角順
///
/// # Examples
///
/// ```
/// use arg_cmp::ArgCmp;
///
/// let north = (0, 1);
/// let south = (0, -1);
/// let east = (1, 0);
/// let west = (-1, 0);
/// let mut points = vec![north, south, east, west];
///
/// points.sort_by_key(|&(x, y)| ArgCmp::new(x, y));
///
/// // 0°, 90°, 180°, 270°
/// assert_eq!(points, vec![(1, 0), (0, 1), (-1, 0), (0, -1)]);
/// ```
///
/// ## 偏角が等しい例
///
/// ```
/// use arg_cmp::ArgCmp;
///
/// assert!(ArgCmp::new(1, 0).cmp(&ArgCmp::new(1, 0)).is_eq());
/// assert!(ArgCmp::new(1, 0).cmp(&ArgCmp::new(2, 0)).is_eq());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArgCmp((i64, i64));

/// 象限
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Quadrant {
    /// 1. [0°, 90°)
    First,
    /// 2. [90°, 180°)
    Second,
    /// 3. [180°, 270°)
    Third,
    /// 4. [270°, 360°)
    Fourth,
}

impl ArgCmp {
    /// # Panics
    ///
    /// if (x, y) is origin
    pub fn new(x: i64, y: i64) -> Self {
        assert_ne!((x, y), (0, 0));
        Self((x, y))
    }

    pub fn x(&self) -> i64 {
        self.0.0
    }

    pub fn y(&self) -> i64 {
        self.0.1
    }

    fn is_lower_half(&self) -> bool {
        self.y() < 0 || (self.y() == 0 && self.x() < 0)
    }

    /// 象限を返す
    ///
    /// # Examples
    ///
    /// ```
    /// use arg_cmp::{ArgCmp, Quadrant};
    ///
    /// let north_east = ArgCmp::new(1, 1);
    /// let north_west = ArgCmp::new(-1, 1);
    /// let south_west = ArgCmp::new(-1, -1);
    /// let south_east = ArgCmp::new(1, -1);
    ///
    /// assert_eq!(north_east.quadrant(), Quadrant::First);
    /// assert_eq!(north_west.quadrant(), Quadrant::Second);
    /// assert_eq!(south_west.quadrant(), Quadrant::Third);
    /// assert_eq!(south_east.quadrant(), Quadrant::Fourth);
    /// ```
    pub fn quadrant(&self) -> Quadrant {
        if self.is_lower_half() {
            if self.x() < 0 {
                Quadrant::Third
            } else {
                Quadrant::Fourth
            }
        } else {
            if self.x() > 0 {
                Quadrant::First
            } else {
                Quadrant::Second
            }
        }
    }
}

impl Ord for ArgCmp {
    // https://atcoder.jp/contests/abc442/editorial/15136
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_lower_half()
            .cmp(&other.is_lower_half())
            .then_with(||
            // cross_product = self.x() * other.y() - self.y() * other.x() > 0
            (self.y() * other.x()).cmp(&(self.x() * other.y())))
    }
}

impl PartialOrd for ArgCmp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArgCmp, Quadrant};

    #[test]
    fn test_arg_cmp() {
        // 0°, 45°, 90°, ...
        let points = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        for i in 0..8 {
            for j in 0..8 {
                let (xi, yi) = points[i];
                let (xj, yj) = points[j];
                assert_eq!(ArgCmp::new(xi, yi).cmp(&ArgCmp::new(xj, yj)), i.cmp(&j));
            }
        }
    }

    #[test]
    fn test_quadrant() {
        // 0°, 45°, 90°, 135°, 180°, 225°, 270°, 315°
        let points = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let expected = vec![
            Quadrant::First,
            Quadrant::First,
            Quadrant::Second,
            Quadrant::Second,
            Quadrant::Third,
            Quadrant::Third,
            Quadrant::Fourth,
            Quadrant::Fourth,
        ];
        for (&(x, y), &q) in points.iter().zip(&expected) {
            assert_eq!(ArgCmp::new(x, y).quadrant(), q);
        }
    }
}
