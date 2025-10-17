use proconio::{
    input,
    source::{Source, once::OnceSource},
};

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Input {
    _n: usize,
    m: usize,
    edges: Vec<(usize, usize)>,
}

struct Output {
    n: isize,
    edge_id: Vec<usize>,
}

fn read_input(input: impl BufRead) -> Input {
    let mut source = OnceSource::new(input);
    input! {
        from &mut source,
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    assert!(source.is_empty());
    Input { _n: n, m, edges }
}

fn read_output(output: impl BufRead) -> Output {
    let mut source = OnceSource::new(output);
    input! {
        from &mut source,
        n: isize,
    }
    if n < 0 {
        return Output {
            n,
            edge_id: Vec::new(),
        };
    }
    let n = n as usize;
    input! {
        from &mut source,
        edge_id: [usize; n],
    }
    assert!(source.is_empty());
    Output {
        n: n as isize,
        edge_id,
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

    assert_eq!(my_output.n, expected_output.n);
    if my_output.n < 0 {
        assert_eq!(my_output.n, -1);
        return Ok(());
    }
    let n = my_output.n as usize;
    let edge_id = my_output.edge_id;
    assert_eq!(edge_id.len(), n);
    assert!(edge_id.len() >= 2);
    let mut seen = vec![false; input.m];
    for i in 0..edge_id.len() {
        assert!(!seen[edge_id[i]]);
        seen[edge_id[i]] = true;
        let s = edge_id[i];
        let t = edge_id[(i + 1) % edge_id.len()];
        assert_eq!(input.edges[s].1, input.edges[t].0);
    }

    Ok(())
}
