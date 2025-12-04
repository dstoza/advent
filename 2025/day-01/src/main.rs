#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut position = 50;
    let mut zeros = 0;
    let mut crossings = 0;

    for line in reader.lines().map(Result::unwrap) {
        let direction = line.chars().next().unwrap();
        let amount: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();

        let distance_to_zero = match direction {
            'L' => position,
            'R' => (100 - position) % 100,
            _ => unreachable!(),
        };

        if amount >= distance_to_zero {
            crossings += i32::from(position != 0) + (amount - distance_to_zero) / 100;
        }

        let raw = match direction {
            'L' => position - amount,
            'R' => position + amount,
            _ => unreachable!(),
        };
        position = ((raw % 100) + 100) % 100;

        if position == 0 {
            zeros += 1;
        }
    }

    println!("zeros: {zeros}");
    println!("crossings: {crossings}");
}
