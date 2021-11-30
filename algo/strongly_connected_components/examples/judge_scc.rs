use proconio::{
    input,
    source::{once::OnceSource, Source},
};

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Input {
    n: usize,
    edges: Vec<(usize, usize)>,
}

struct Output {
    k: usize,
    components: Vec<Vec<usize>>,
}

fn read_input(input: impl BufRead) -> Input {
    let mut source = OnceSource::new(input);
    input! {
        from &mut source,
        n: usize,
        _m: usize,
        edges: [(usize, usize); _m],
    };
    assert!(source.is_empty());
    Input { n, edges }
}

fn read_output(output: impl BufRead) -> Output {
    let mut source = OnceSource::new(output);
    input! {
        from &mut source,
        k: usize,
    };
    let mut components = Vec::new();
    for _ in 0..k {
        input! {
            from &mut source,
            l: usize,
            com: [usize; l],
        };
        components.push(com);
    }

    assert!(source.is_empty());
    Output { k, components }
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

    assert_eq!(my_output.k, expected_output.k);

    let mut my_scc = my_output.components;
    let mut component_id = vec![0; input.n];
    for i in 0..my_scc.len() {
        for &v in &my_scc[i] {
            component_id[v] = i;
        }
    }
    for (u, v) in input.edges {
        assert!(component_id[u] <= component_id[v]);
    }

    let mut expected_scc = expected_output.components;
    for com in &mut my_scc {
        com.sort();
    }
    for com in &mut expected_scc {
        com.sort();
    }
    my_scc.sort();
    expected_scc.sort();
    assert_eq!(my_scc, expected_scc);

    Ok(())
}
