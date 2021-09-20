// oj: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0580

use coordinate_compression::CoordinateCompression;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xydxyd: [(u64, u64, u64, u64, u64, u64); n],
    }

    let cx: CoordinateCompression<u64> = xydxyd
        .iter()
        .copied()
        .map(|(x1, _, _, x2, _, _)| vec![x1, x2])
        .flatten()
        .collect();
    let cy: CoordinateCompression<u64> = xydxyd
        .iter()
        .copied()
        .map(|(_, y1, _, _, y2, _)| vec![y1, y2])
        .flatten()
        .collect();
    let cz: CoordinateCompression<u64> = xydxyd
        .iter()
        .copied()
        .map(|(_, _, z1, _, _, z2)| vec![z1, z2])
        .flatten()
        .collect();

    let mut freq = vec![vec![vec![0; n * 2]; n * 2]; n * 2];
    for (x1, y1, z1, x2, y2, z2) in &xydxyd {
        for x in cx.find_index(x1)..cx.find_index(x2) {
            for y in cy.find_index(y1)..cy.find_index(y2) {
                for z in cz.find_index(z1)..cz.find_index(z2) {
                    freq[x][y][z] += 1;
                }
            }
        }
    }
    let mut ans = 0;
    for x in 0..(n * 2 - 1) {
        for y in 0..(n * 2 - 1) {
            for z in 0..(n * 2 - 1) {
                if freq[x][y][z] >= k {
                    ans += (cx[x + 1] - cx[x]) * (cy[y + 1] - cy[y]) * (cz[z + 1] - cz[z]);
                }
            }
        }
    }
    println!("{}", ans);
}
