// problem: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0580

use coordinate_compression::OrderMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xydxyd: [(u64, u64, u64, u64, u64, u64); n],
    }

    let cx: OrderMap<u64> = xydxyd
        .iter()
        .copied()
        .map(|(x1, _, _, x2, _, _)| vec![x1, x2])
        .flatten()
        .collect();
    let cy: OrderMap<u64> = xydxyd
        .iter()
        .copied()
        .map(|(_, y1, _, _, y2, _)| vec![y1, y2])
        .flatten()
        .collect();
    let cz: OrderMap<u64> = xydxyd
        .iter()
        .copied()
        .map(|(_, _, z1, _, _, z2)| vec![z1, z2])
        .flatten()
        .collect();

    let mut freq = vec![vec![vec![0; n * 2]; n * 2]; n * 2];
    for (x1, y1, z1, x2, y2, z2) in &xydxyd {
        for x in cx.ord(x1)..cx.ord(x2) {
            for y in cy.ord(y1)..cy.ord(y2) {
                for z in cz.ord(z1)..cz.ord(z2) {
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
                    ans += (cx.at(x + 1) - cx.at(x))
                        * (cy.at(y + 1) - cy.at(y))
                        * (cz.at(z + 1) - cz.at(z));
                }
            }
        }
    }
    println!("{}", ans);
}
