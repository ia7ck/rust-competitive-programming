/// log2 の切り上げです。
pub trait CeilLog2 {
    /// 2^x >= self となる最小の x を返します。
    fn ceil_log2(self) -> Self;
}

macro_rules! impl_ceil_log2 {
    ($($t:ty),+) => {
        $(
            impl CeilLog2 for $t {
                fn ceil_log2(self) -> Self {
                    assert!(self >= 1);
                    let mut x = 0;
                    let mut pow_2_x = 1;
                    while pow_2_x < self {
                        x += 1;
                        pow_2_x = pow_2_x.saturating_mul(2);
                    }
                    x
                }
            }
        )+
    };
}

impl_ceil_log2!(usize, u32, u64);

#[cfg(test)]
mod tests {
    use crate::CeilLog2;

    #[test]
    fn test_ceil_log2() {
        let tests = vec![
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 4),
        ];
        for (x, expected) in tests {
            assert_eq!((x as usize).ceil_log2(), (expected as usize));
            assert_eq!((x as u32).ceil_log2(), (expected as u32));
            assert_eq!((x as u64).ceil_log2(), (expected as u64));
        }
    }
}
