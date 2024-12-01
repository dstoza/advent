#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let (mut left, mut right): (Vec<_>, Vec<_>) = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let difference: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("difference {difference}");

    let mut frequencies: HashMap<i32, _> = HashMap::new();
    for r in right {
        frequencies
            .entry(r)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let similarity: isize = left
        .iter()
        .map(|l| (*l as isize) * frequencies.get(l).copied().unwrap_or(0))
        .sum();

    println!("similarity {similarity}");
}
