#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Brick {
    x: RangeInclusive<u16>,
    y: RangeInclusive<u16>,
    z: RangeInclusive<u16>,
}

impl Brick {
    fn parse(string: &str) -> Self {
        let mut parts = string.split('~');
        let mut components = parts
            .next()
            .unwrap()
            .split(',')
            .map(|component| component.parse().unwrap())
            .zip(
                parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|component| component.parse().unwrap()),
            )
            .map(|(start, end)| start..=end);
        Self {
            x: components.next().unwrap(),
            y: components.next().unwrap(),
            z: components.next().unwrap(),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut bricks = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| Brick::parse(&line))
        .collect::<Vec<_>>();

    bricks.sort_unstable_by_key(|brick| *brick.z.start());

    for brick in bricks {
        println!("{brick:?}");
    }
}
