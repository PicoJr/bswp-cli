use crate::utils::swap_from_str;
use bswp::pattern::{Pattern, Predicate};
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufReader};

mod cli;
mod utils;

fn main() -> anyhow::Result<()> {
    let matches = cli::get_app().get_matches();
    let values = matches
        .values_of("pattern")
        .expect("at least a pattern should be provided"); // should not happen thanks to clap
    let mut swaps: Vec<(Pattern, Predicate)> = vec![];
    for value in values {
        let swap = swap_from_str(value)?;
        swaps.push(swap);
    }
    let stdin = stdin();
    let stdout = stdout();
    let mut input: Box<dyn std::io::BufRead> = match matches.value_of("input") {
        None => Box::new(stdin.lock()),
        Some(input_path) => Box::new(BufReader::new(File::open(input_path)?)),
    };
    let mut output: Box<dyn std::io::Write> = match matches.value_of("output") {
        None => Box::new(stdout.lock()),
        Some(output_path) => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(output_path)?,
        ),
    };
    bswp::io::swap_io(&mut input, &mut output, &swaps)?;
    Ok(())
}
