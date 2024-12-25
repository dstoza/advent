#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

#[derive(Debug, Eq, PartialEq)]
enum Kind {
    Lock,
    Key,
}

#[derive(Debug)]
struct Pins {
    kind: Kind,
    heights: [u8; 5],
}

impl Pins {
    fn parse(lines: &mut impl Iterator<Item = String>) -> Self {
        let top = lines.next().unwrap();
        let kind = if top.starts_with('.') {
            Kind::Key
        } else {
            Kind::Lock
        };

        let mut heights = [0; 5];
        for _ in 0..5 {
            let line = lines.next().unwrap();
            for (pin, value) in line.as_bytes().iter().enumerate() {
                if *value == b'#' {
                    heights[pin] += 1;
                }
            }
        }

        lines.next();

        Self { kind, heights }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(Result::unwrap);
    let mut pins = Vec::new();
    loop {
        pins.push(Pins::parse(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }

    let locks = pins
        .iter()
        .filter_map(|p| {
            if p.kind == Kind::Lock {
                Some(p.heights)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let keys = pins
        .iter()
        .filter_map(|p| {
            if p.kind == Kind::Key {
                Some(p.heights)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let count: usize = locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    lock.iter()
                        .copied()
                        .zip(key.iter().copied())
                        .all(|(lock, key)| lock + key <= 5)
                })
                .count()
        })
        .sum();

    println!("{count}");
}
