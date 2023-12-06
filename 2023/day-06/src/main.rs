#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn count_winners(time: u64, distance: u64) -> u64 {
    for hold in 0..time {
        let d = hold * (time - hold);
        if d > distance {
            return time + 1 - hold * 2;
        }
    }
    0
}

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
        .map(|time| time.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let error_margin: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| count_winners(*time, *distance))
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

    println!("{}", count_winners(time, distance));
}
