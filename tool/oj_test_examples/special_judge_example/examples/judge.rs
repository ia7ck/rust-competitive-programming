use proconio::{
    input,
    source::{Source, once::OnceSource},
};

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Input {
    a: u32,
    b: u32,
}

struct Output {
    sum: u32,
}

fn read_input(input: impl BufRead) -> Input {
    let mut source = OnceSource::new(input);
    input! {
        from &mut source,
        a: u32,
        b: u32,
    }
    assert!(source.is_empty());
    Input { a, b }
}

fn read_output(output: impl BufRead) -> Output {
    let mut source = OnceSource::new(output);
    input! {
        from &mut source,
        sum: u32,
    }
    assert!(source.is_empty());
    Output { sum }
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

    assert_eq!(my_output.sum, expected_output.sum);
    assert_eq!(my_output.sum, input.a + input.b);

    Ok(())
}
