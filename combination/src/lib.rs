use mod_int::{ModInt, Modulo};

pub fn make_binom_func_raw(len: usize, mo: i64) -> impl Fn(usize, usize) -> i64 {
    let mut fac = vec![0; len];
    let mut inv = vec![0; len];
    let mut inv_fac = vec![0; len];
    fac[0] = 1;
    fac[1] = 1;
    inv[1] = 1;
    inv_fac[0] = 1;
    inv_fac[1] = 1;
    for i in 2..len {
        fac[i] = fac[i - 1] * (i as i64) % mo;
        inv[i] = (-inv[(mo as usize) % i] * (mo / (i as i64))).rem_euclid(mo);
        inv_fac[i] = inv_fac[i - 1] * inv[i] % mo;
    }
    move |n: usize, k: usize| -> i64 {
        if n < k {
            return 0;
        }
        ((fac[n] * inv_fac[k]) % mo * inv_fac[n - k]) % mo
    }
}

pub fn make_binom_func_mint<M>(len: usize) -> impl Fn(usize, usize) -> ModInt<M>
where
    M: Modulo,
{
    let mut fac = vec![ModInt::new(0); len];
    fac[0] = ModInt::new(1);
    for i in 1..len {
        fac[i] = fac[i - 1] * ModInt::new(i as i64);
    }
    move |n: usize, k: usize| {
        if n < k {
            return ModInt::new(0);
        }
        fac[n] / fac[k] / fac[n - k]
    }
}

#[cfg(test)]
mod tests {
    use crate::{make_binom_func_mint, make_binom_func_raw};
    use mod_int::{define_mod_int_p, ModInt, Modulo};

    #[test]
    fn check_by_pascal_triangle() {
        const N: usize = 100;
        const K: usize = 100;
        const M: i64 = 107;
        let mut dp = vec![vec![0; K]; N];
        dp[0][0] = 1;
        for i in 1..N {
            dp[i][0] = 1;
            for j in 1..K {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % M;
            }
        }
        let binom_pascal_triangle = |n: usize, k: usize| dp[n][k];
        let binom = make_binom_func_raw(N, M);
        define_mod_int_p!(Mod107, ModInt107, M);
        let binom_mint = make_binom_func_mint::<Mod107>(N);
        for i in 0..N {
            for j in 0..=i {
                let expect = binom_pascal_triangle(i, j);
                assert_eq!(binom(i, j), expect);
                assert_eq!(binom_mint(i, j).val(), expect);
            }
        }
    }
}
