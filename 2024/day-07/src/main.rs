#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use std::{
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

fn parse(lines: impl Iterator<Item = String>) -> Vec<(u64, Vec<u64>)> {
    let mut result = Vec::new();
    for line in lines {
        let mut split = line.split(':');
        let value: u64 = split.next().unwrap().parse().unwrap();
        let terms: Vec<_> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|term| term.parse::<u64>().unwrap())
            .collect();
        result.push((value, terms));
    }

    result
}

const OPERATIONS: &[&str] = &["+", "*", "||"];

fn is_possible(
    accumulator: u64,
    target: u64,
    terms: &[u64],
    operation: &str,
    allow_concatenation: bool,
) -> bool {
    if terms.is_empty() {
        return accumulator == target;
    }

    if !allow_concatenation && operation == "||" {
        return false;
    }

    if accumulator > target {
        return false;
    }

    match operation {
        "+" => OPERATIONS.iter().any(|operation| {
            is_possible(
                accumulator + terms[0],
                target,
                &terms[1..],
                operation,
                allow_concatenation,
            )
        }),
        "*" => OPERATIONS.iter().any(|operation| {
            is_possible(
                accumulator * terms[0],
                target,
                &terms[1..],
                operation,
                allow_concatenation,
            )
        }),
        "||" => {
            let shift = 10f64
                .powi(((terms[0] + 1) as f64).log10().ceil().round() as i32)
                .round() as u64;
            let concatenated = accumulator * shift + terms[0];
            OPERATIONS.iter().any(|operation| {
                is_possible(
                    concatenated,
                    target,
                    &terms[1..],
                    operation,
                    allow_concatenation,
                )
            })
        }
        _ => unreachable!(),
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let parsed = parse(reader.lines().map(Result::unwrap));
    let mut total = 0;
    for (value, terms) in parsed {
        if is_possible(0, value, &terms, "+", args.part == 2) {
            total += value;
        }
    }

    println!("{total}");
}
