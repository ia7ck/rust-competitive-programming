use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use auxiliary_tree::auxiliary_tree;
use graph::tree_drop_parent;
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{input, marker::Usize1};

const INF: usize = usize::MAX / 2;

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut nodes_by_color = vec![vec![]; n];
    for i in 0..n {
        nodes_by_color[c[i]].push(i);
    }

    let (g, _parent) = tree_drop_parent(n, 0, &edges);
    let mut ord = Vec::new();
    dfs(0, &g, &mut ord);
    let mut inv_ord = vec![0; n];
    for i in 0..n {
        inv_ord[ord[i]] = i;
    }

    let lca = LowestCommonAncestor::new(n, 0, &edges);

    let mut ans = vec![INF; n];
    let mut dist = vec![INF; n];
    let mut start = vec![INF; n];
    for (color, nodes) in nodes_by_color.iter().enumerate() {
        if nodes.is_empty() {
            continue;
        }
        let (_, h) = auxiliary_tree(&nodes, &inv_ord, &lca);
        let mut parent = HashMap::new();
        for (&i, children) in &h {
            for &j in children {
                parent.insert(j, i);
            }
        }

        // ここからむずくない？
        // dijkstra
        for &i in h.keys() {
            dist[i] = INF;
            start[i] = INF;
        }
        let mut heap = BinaryHeap::new();
        for &i in nodes {
            dist[i] = 0;
            start[i] = i;
            heap.push((Reverse(0), i));
        }
        while let Some((Reverse(d), i)) = heap.pop() {
            if dist[i] < d {
                continue;
            }
            let mut push = |j: usize| {
                let new_d = d + lca.get_dist(i, j);
                if new_d < dist[j] {
                    dist[j] = new_d;
                    start[j] = start[i];
                    heap.push((Reverse(new_d), j));
                }
            };
            if let Some(&j) = parent.get(&i) {
                push(j);
            }
            for &j in &h[&i] {
                push(j);
            }
        }

        for (&i, children) in &h {
            for &j in children {
                let (i, j) = (start[i], start[j]);
                assert_eq!(c[i], color);
                assert_eq!(c[j], color);
                if i == j {
                    continue;
                }
                let d = lca.get_dist(i, j);
                ans[i] = ans[i].min(dist[j] + d);
                ans[j] = ans[j].min(dist[i] + d);
            }
        }
    }

    for ans in ans {
        assert!(ans < n);
        println!("{}", ans);
    }
}

fn dfs(i: usize, g: &Vec<Vec<usize>>, ord: &mut Vec<usize>) {
    ord.push(i);
    for &j in &g[i] {
        dfs(j, g, ord);
    }
}
