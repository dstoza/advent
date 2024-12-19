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

fn is_possible(design: &str, patterns: &[String], cache: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(cached) = cache.get(design) {
        return *cached;
    }

    for pattern in patterns {
        if design.starts_with(pattern) && is_possible(&design[pattern.len()..], patterns, cache) {
            cache.insert(design.to_owned(), true);
            return true;
        }
    }

    cache.insert(design.to_owned(), false);
    false
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

    let possible = lines
        .skip(1)
        .filter(|design| is_possible(design, &patterns, &mut cache))
        .count();

    println!("{possible}");
}
