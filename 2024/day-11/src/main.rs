#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Steps to simulate
    #[arg(short, long, value_parser = clap::value_parser!(usize))]
    steps: usize,

    /// File to open
    filename: String,
}

fn stones_after_step(
    initial: usize,
    steps: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if steps == 0 {
        return 1;
    }

    if let Some(stones) = cache.get(&(initial, steps)) {
        return *stones;
    }

    if initial == 0 {
        let stones = stones_after_step(1, steps - 1, cache);
        cache.insert((initial, steps), stones);
        return stones;
    }

    let as_string = initial.to_string();
    if as_string.len() % 2 == 0 {
        let left = as_string[0..as_string.len() / 2].parse().unwrap();
        let right = as_string[as_string.len() / 2..].parse().unwrap();
        let stones =
            stones_after_step(left, steps - 1, cache) + stones_after_step(right, steps - 1, cache);
        cache.insert((initial, steps), stones);
        return stones;
    }

    let stones = stones_after_step(initial * 2024, steps - 1, cache);
    cache.insert((initial, steps), stones);
    stones
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let stones = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let stones = stones
        .iter()
        .map(|stone| stones_after_step(*stone, args.steps, &mut cache))
        .sum::<usize>();

    println!("{stones}");
}
