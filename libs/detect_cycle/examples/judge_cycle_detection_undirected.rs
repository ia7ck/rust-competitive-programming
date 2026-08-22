use proconio::{
    input,
    source::{Source, once::OnceSource},
};

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Input {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize)>,
}

enum Output {
    NotFound,
    Found {
        vertices: Vec<usize>,
        edge_ids: Vec<usize>,
    },
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
    Input { n, m, edges }
}

fn read_output(output: impl BufRead) -> Output {
    let mut source = OnceSource::new(output);
    input! {
        from &mut source,
        len: isize,
    }
    if len < 0 {
        assert_eq!(len, -1);
        assert!(source.is_empty());
        return Output::NotFound;
    }

    let len = len as usize;
    assert!(len >= 1);
    input! {
        from &mut source,
        vertices: [usize; len],
        edge_ids: [usize; len],
    }
    assert!(source.is_empty());
    Output::Found { vertices, edge_ids }
}

fn validate_cycle(input: &Input, vertices: &[usize], edge_ids: &[usize]) {
    let len = vertices.len();
    assert_eq!(edge_ids.len(), len);

    let mut seen_vertices = vec![false; input.n];
    let mut seen_edges = vec![false; input.m];
    for i in 0..len {
        let vertex = vertices[i];
        let next_vertex = vertices[(i + 1) % len];
        let edge_id = edge_ids[i];

        assert!(vertex < input.n);
        assert!(!seen_vertices[vertex]);
        seen_vertices[vertex] = true;

        assert!(edge_id < input.m);
        assert!(!seen_edges[edge_id]);
        seen_edges[edge_id] = true;

        let (u, v) = input.edges[edge_id];
        assert!((u == vertex && v == next_vertex) || (u == next_vertex && v == vertex));
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);
    let (input, my_output, expected_output) = (&args[1], &args[2], &args[3]);

    let input = read_input(BufReader::new(File::open(input)?));
    let my_output = read_output(BufReader::new(File::open(my_output)?));
    let expected_output = read_output(BufReader::new(File::open(expected_output)?));

    match (my_output, expected_output) {
        (Output::NotFound, Output::NotFound) => {}
        (Output::Found { vertices, edge_ids }, Output::Found { .. }) => {
            validate_cycle(&input, &vertices, &edge_ids)
        }
        _ => panic!("cycle existence does not match the expected output"),
    }

    Ok(())
}
