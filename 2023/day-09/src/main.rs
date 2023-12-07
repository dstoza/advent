#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn extrapolate(sequence: &[i64]) -> (i64, i64) {
    if sequence.iter().all(|x| *x == 0) {
        return (0, 0);
    }

    let differences = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    let (next, previous) = extrapolate(&differences);
    (
        sequence.last().unwrap() + next,
        sequence.first().unwrap() - previous,
    )
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let (next_sum, previous_sum) = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            extrapolate(&sequence)
        })
        .reduce(|(next_sum, previous_sum), (next, previous)| {
            (next_sum + next, previous_sum + previous)
        })
        .unwrap();

    println!("{next_sum} {previous_sum}");
}
