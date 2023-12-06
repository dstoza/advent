#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let error_margin: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            (0..*time)
                .map(|hold| hold * (*time - hold))
                .filter(|d| *d > *distance)
                .count()
        })
        .product();

    println!("{error_margin}");

    let time = times
        .iter()
        .map(std::string::ToString::to_string)
        .fold(String::new(), |mut acc, s| {
            acc.push_str(&s);
            acc
        })
        .parse::<u64>()
        .unwrap();

    let distance = distances
        .iter()
        .map(std::string::ToString::to_string)
        .fold(String::new(), |mut acc, s| {
            acc.push_str(&s);
            acc
        })
        .parse::<u64>()
        .unwrap();

    let longer = (0..time)
        .map(|hold| hold * (time - hold))
        .filter(|d| *d > distance)
        .count();

    println!("{longer}");
}
