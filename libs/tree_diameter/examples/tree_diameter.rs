use proconio::input;
use tree_diameter::tree_diameter;

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, u64); n - 1],
    };

    let (diameter, path) = tree_diameter(n, &edges);
    println!("{} {}", diameter, path.len());
    for i in 0..path.len() {
        print!("{}", path[i]);
        if i + 1 < path.len() {
            print!(" ");
        }
    }
    println!();
}
