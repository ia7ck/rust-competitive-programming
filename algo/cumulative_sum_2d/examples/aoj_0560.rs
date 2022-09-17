// problem: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0560

use cumulative_sum_2d::CumulativeSum2D;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        field: [Chars; h],
        queries: [(usize, usize, usize, usize); k],
    };

    let mut j = vec![vec![0; w]; h];
    let mut o = vec![vec![0; w]; h];
    let mut i = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            match field[y][x] {
                'J' => {
                    j[y][x] += 1;
                }
                'O' => {
                    o[y][x] += 1;
                }
                'I' => {
                    i[y][x] += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    let cum_sum_j = CumulativeSum2D::new(&j);
    let cum_sum_o = CumulativeSum2D::new(&o);
    let cum_sum_i = CumulativeSum2D::new(&i);
    for (a, b, c, d) in queries {
        println!(
            "{} {} {}",
            cum_sum_j.sum(a - 1..c, b - 1..d),
            cum_sum_o.sum(a - 1..c, b - 1..d),
            cum_sum_i.sum(a - 1..c, b - 1..d),
        );
    }
}
