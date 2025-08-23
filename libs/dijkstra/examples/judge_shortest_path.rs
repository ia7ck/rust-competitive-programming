use proconio::{
    input,
    source::{once::OnceSource, Source},
};

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::{
    collections::{HashMap, HashSet},
    env,
};

struct Input {
    _n: usize,
    _m: usize,
    s: usize,
    t: usize,
    edges: Vec<(usize, usize, u64)>,
}

enum Output {
    NotFound(i8),
    Found {
        x: u64,
        y: usize,
        edges: Vec<(usize, usize)>,
    },
}

fn read_input(input: impl BufRead) -> Input {
    let mut source = OnceSource::new(input);
    input! {
        from &mut source,
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize, u64); m],
    }
    assert!(source.is_empty());
    Input {
        _n: n,
        _m: m,
        s,
        t,
        edges,
    }
}

fn read_output(output: impl BufRead) -> Output {
    let mut source = OnceSource::new(output);
    input! {
        from &mut source,
        x: i64,
    }
    if x < 0 {
        assert!(source.is_empty());
        return Output::NotFound(-1);
    }
    input! {
        from &mut source,
        y: usize,
        edges: [(usize, usize); y],
    }
    assert!(source.is_empty());
    Output::Found {
        x: x as u64,
        y,
        edges,
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);
    let (input, my_output, expected_output) = (&args[1], &args[2], &args[3]);

    let input = File::open(input)?;
    let input = read_input(BufReader::new(input));

    let my_output = File::open(my_output)?;
    let my_output = read_output(BufReader::new(my_output));

    let expected_output = File::open(expected_output)?;
    let expected_output = read_output(BufReader::new(expected_output));

    match (my_output, expected_output) {
        (Output::NotFound(i), Output::NotFound(j)) => {
            assert_eq!(i, j); // -1
        }
        (Output::Found { x, y, edges }, Output::Found { x: xx, .. }) => {
            assert_eq!(x, xx);
            assert_eq!(y, edges.len());
            let (s, t) = (edges[0].0, edges[y - 1].1);
            assert_eq!(s, input.s);
            assert_eq!(t, input.t);
            let mut map = HashMap::new();
            for &(a, b, c) in &input.edges {
                map.insert((a, b), c);
            }
            let mut seen = HashSet::new();
            seen.insert(edges[0].0);
            let mut d = 0;
            for (a, b) in edges {
                assert!(!seen.contains(&b));
                seen.insert(b);
                d += map[&(a, b)];
            }
            assert_eq!(x, d);
        }
        _ => {
            assert!(false);
        }
    }

    Ok(())
}
