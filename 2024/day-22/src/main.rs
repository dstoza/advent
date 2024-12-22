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

struct Generator {
    secret: u64,
}

impl Generator {
    fn new(secret: u64) -> Self {
        Self { secret }
    }

    fn step(&mut self) {
        self.secret ^= self.secret << 6;
        self.secret &= (1 << 24) - 1;
        self.secret ^= self.secret >> 5;
        self.secret &= (1 << 24) - 1;
        self.secret ^= self.secret << 11;
        self.secret &= (1 << 24) - 1;
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines().map(Result::unwrap) {
        let initial = line.parse().unwrap();
        let mut generator = Generator::new(initial);
        for _ in 0..2000 {
            generator.step();
        }
        sum += generator.secret;
    }

    println!("{sum}");
}
