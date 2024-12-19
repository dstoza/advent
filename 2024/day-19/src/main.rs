#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

fn possible_arrangements(
    design: &str,
    patterns: &[String],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(cached) = cache.get(design) {
        return *cached;
    }

    let arrangements = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .map(|pattern| possible_arrangements(&design[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(design.to_owned(), arrangements);
    arrangements
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(Result::unwrap);
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    let possible: usize = lines
        .skip(1)
        .map(|design| {
            let arrangements = possible_arrangements(&design, &patterns, &mut cache);
            if args.part == 1 {
                usize::from(arrangements > 0)
            } else {
                arrangements
            }
        })
        .sum();

    println!("{possible}");
}
