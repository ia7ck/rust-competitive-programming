#[macro_use]
mod util {
    #[allow(unused_macros)]
    macro_rules! chmin {
        ($a:expr, $b:expr) => {
            std::cmp::min($a, $b)
        };
    }

    #[allow(unused_macros)]
    macro_rules! chmax {
        ($a:expr, $b:expr) => {
            std::cmp::max($a, $b)
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn chmin_test() {
        assert_eq!(chmin!(123, 4), 4);
        assert_eq!(chmin!(1, 234), 1);
    }

    #[test]
    fn chmax_test() {
        assert_eq!(chmax!(123, 4), 123);
        assert_eq!(chmax!(1, 234), 234);
    }
}
