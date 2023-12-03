#![warn(clippy::pedantic)]
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Debug)]
struct Map {
    destination: i64,
    source: i64,
    length: i64,
}

impl Map {
    fn parse(line: &str) -> Self {
        let mut split = line.split_whitespace();
        Self {
            destination: split.next().unwrap().parse().unwrap(),
            source: split.next().unwrap().parse().unwrap(),
            length: split.next().unwrap().parse().unwrap(),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = Vec::new();
    let mut current_maps = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if !current_maps.is_empty() {
                maps.push(current_maps.clone());
                current_maps.clear();
            }
            lines.next().unwrap();
            continue;
        }

        current_maps.push(Map::parse(&line));
    }

    maps.push(current_maps);

    for map in &maps {
        println!("{map:?}");
    }
}
