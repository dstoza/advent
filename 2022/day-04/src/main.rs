#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::RangeInclusive,
};

enum Mode {
    Enclose,
    Overlap,
}

fn parse_range(string: &str) -> RangeInclusive<i32> {
    let mut split = string.split('-');
    split.next().unwrap().parse().unwrap()..=split.next().unwrap().parse().unwrap()
}

trait RangeOverlap {
    fn encloses(&self, other: &RangeInclusive<i32>) -> bool;
    fn overlaps(&self, other: &RangeInclusive<i32>) -> bool;
}

impl RangeOverlap for RangeInclusive<i32> {
    fn encloses(&self, other: &RangeInclusive<i32>) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }

    fn overlaps(&self, other: &RangeInclusive<i32>) -> bool {
        !(self.start() > other.end() || self.end() < other.start())
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let mode = match std::env::args().nth(2).expect("Mode not found").as_str() {
        "enclose" => Mode::Enclose,
        "overlap" => Mode::Overlap,
        _ => unimplemented!("Unknown mode. Valid options are 'enclose' and 'overlap'"),
    };

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);

    let match_count = reader
        .lines()
        .map(std::result::Result::unwrap)
        .filter(|line| {
            let mut split = line.split(',');
            let left = parse_range(split.next().unwrap());
            let right = parse_range(split.next().unwrap());
            match mode {
                Mode::Enclose => left.encloses(&right) || right.encloses(&left),
                Mode::Overlap => left.overlaps(&right),
            }
        })
        .count();

    println!("Found {match_count}");
}
