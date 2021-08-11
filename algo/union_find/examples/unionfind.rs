// oj: https://judge.yosupo.jp/problem/unionfind
use join::Join;
use proconio::{fastout, input};
use union_find::UnionFind;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut uf = UnionFind::new(n);
    input! {
        q: usize,
    }
    let mut ans = Vec::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            0 => {
                input! {
                    u: usize,
                    v: usize,
                }
                uf.unite(u, v);
            }
            1 => {
                input! {
                    u: usize,
                    v: usize,
                }
                ans.push(uf.same(u, v) as usize);
            }
            _ => unreachable!(),
        }
    }

    // Library Checker には output が 0 行のケースが含まれていて
    // そのファイルに末尾改行が無いので ans が empty のときは何も出力しないようにする
    if ans.is_empty() {
        return;
    }

    println!("{}", ans.iter().join("\n"));
}
