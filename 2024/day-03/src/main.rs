#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let token_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let lines = reader.lines().map(Result::unwrap);
    let mut enabled = true;
    let mut sum = 0;
    for line in lines {
        for capture in token_regex.captures_iter(&line) {
            let token = &capture[0];
            match token {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {
                    if args.part == 2 && !enabled {
                        continue;
                    }
                    let left = capture[1].parse::<u32>().unwrap();
                    let right = capture[2].parse::<u32>().unwrap();
                    sum += left * right;
                }
            }
        }
    }

    println!("{sum}");
}
