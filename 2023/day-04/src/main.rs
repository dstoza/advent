#![warn(clippy::pedantic)]
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let matches = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let numbers = line.split(": ").nth(1).unwrap();
            let mut split = numbers.split(" | ");
            let winners = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let mine = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            mine.iter()
                .filter(|number| winners.contains(number))
                .count()
        })
        .collect::<Vec<_>>();

    let points: u32 = matches.iter().map(|matches| (1u32 << matches) / 2).sum();
    println!("{points}");

    let mut copies = VecDeque::new();
    let copy_sum: u32 = matches
        .iter()
        .map(|count| {
            let current_copies = 1 + copies.pop_front().unwrap_or(0);

            if copies.len() < *count {
                copies.resize(*count, 0);
            }

            for c in copies.iter_mut().take(*count) {
                *c += current_copies;
            }

            current_copies
        })
        .sum();

    println!("{copy_sum}");
}
