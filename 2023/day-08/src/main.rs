#![warn(clippy::pedantic)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn try_parse(byte: u8) -> Option<Self> {
        match byte {
            b'L' => Some(Direction::Left),
            b'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: String,
    right: String,
}

impl Pair {
    fn parse(string: &str) -> Self {
        string
            .strip_prefix('(')
            .and_then(|string| string.strip_suffix(')'))
            .map(|string| {
                let mut split = string.split(", ");
                let left = String::from(split.next().unwrap());
                let right = String::from(split.next().unwrap());
                Pair { left, right }
            })
            .unwrap()
    }
}

fn greatest_common_denominator(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * b / greatest_common_denominator(a, b)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let directions = lines
        .next()
        .unwrap()
        .bytes()
        .filter_map(Direction::try_parse)
        .collect::<Vec<_>>();
    lines.next();

    let map = lines
        .map(|line| {
            let mut split = line.split(" = ");
            let node = String::from(split.next().unwrap());
            let pair = Pair::parse(split.next().unwrap());
            (node, pair)
        })
        .collect::<HashMap<_, _>>();

    if map.contains_key("ZZZ") {
        let mut steps = 0;
        let mut direction = directions.iter().cycle();
        let mut current = String::from("AAA");
        while current != "ZZZ" {
            steps += 1;
            match direction.next().unwrap() {
                Direction::Left => {
                    current = map[&current].left.clone();
                }
                Direction::Right => {
                    current = map[&current].right.clone();
                }
            }
        }

        println!("{steps}");
    }

    let ghost_steps = map
        .keys()
        .filter_map(|key| {
            if key.as_bytes()[2] != b'A' {
                return None;
            }

            let mut steps = 0;
            let mut direction = directions.iter().cycle();
            let mut current = key.clone();
            while !current.ends_with('Z') {
                steps += 1;
                match direction.next().unwrap() {
                    Direction::Left => {
                        current = map[&current].left.clone();
                    }
                    Direction::Right => {
                        current = map[&current].right.clone();
                    }
                }
            }
            let first_z = current.clone();

            // Verify that the cycle repeats to the same **Z with the same period
            for _ in 0..steps {
                match direction.next().unwrap() {
                    Direction::Left => {
                        current = map[&current].left.clone();
                    }
                    Direction::Right => {
                        current = map[&current].right.clone();
                    }
                }
            }
            assert!(current == first_z);

            Some(steps)
        })
        .reduce(least_common_multiple)
        .unwrap();

    println!("{ghost_steps}");
}
