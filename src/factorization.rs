pub fn min_factors(n: usize) -> Vec<usize> {
    let mut result = (0..n).map(|i| i).collect::<Vec<_>>();
    for i in 2..n {
        if result[i] == i {
            let mut j = i + i;
            while j < n {
                if result[j] == j {
                    result[j] = i;
                }
                j += i;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::min_factors;

    #[test]
    fn min_factors_test() {
        let n = 1000;
        let min_factors = min_factors(n);
        for i in 2..n {
            for j in 2..=i {
                if i % j == 0 {
                    assert_eq!(j, min_factors[i]);
                    break;
                }
            }
        }
    }
}
