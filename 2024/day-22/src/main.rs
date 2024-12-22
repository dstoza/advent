#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

struct Generator {
    secret: i32,
}

impl Generator {
    fn new(secret: i32) -> Self {
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

fn sequences_for_buyer(secret: i32) -> HashMap<(i8, i8, i8, i8), i8> {
    let mut generator = Generator::new(secret);
    let mut previous = secret;
    let mut changes = Vec::new();
    for _ in 0..2000 {
        generator.step();
        #[allow(clippy::cast_possible_truncation)]
        let change = (generator.secret % 10 - previous % 10) as i8;
        changes.push((change, (generator.secret % 10) as i8));
        previous = generator.secret;
    }

    let mut sequences = HashMap::new();
    for window in changes.windows(4) {
        let sequence = (window[0].0, window[1].0, window[2].0, window[3].0);
        sequences.entry(sequence).or_insert(window[3].1);
    }

    sequences
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut sequences = HashMap::new();
    for line in reader.lines().map(Result::unwrap) {
        let secret = line.parse().unwrap();
        let mut generator = Generator::new(secret);
        for _ in 0..2000 {
            generator.step();
        }
        sum += i64::from(generator.secret);

        let local_sequences = sequences_for_buyer(secret);
        for (sequence, value) in local_sequences {
            sequences
                .entry(sequence)
                .and_modify(|v| *v += i32::from(value))
                .or_insert(i32::from(value));
        }
    }

    println!("{sum}");
    println!("{}", sequences.values().max().unwrap());
}
