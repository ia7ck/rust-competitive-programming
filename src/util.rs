#[allow(unused_macros)]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        $a = std::cmp::min($a, $b)
    };
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        $a = std::cmp::max($a, $b)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn chmin_test() {
        let mut a = 123;
        chmin!(a, 4);
        assert_eq!(a, 4);
        let mut b = 1;
        chmin!(b, 234);
        assert_eq!(b, 1);
    }

    #[test]
    fn chmax_test() {
        let mut a = 123;
        chmax!(a, 4);
        assert_eq!(a, 123);
        let mut b = 1;
        chmax!(b, 234);
        assert_eq!(b, 234);
    }
}
