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

fn is_safe(report: &[u32]) -> bool {
    let in_order = report.is_sorted() || report.is_sorted_by(|a, b| *a > *b);
    if !in_order {
        return false;
    }

    report
        .windows(2)
        .all(|window| (1..=3).contains(&(window[0].abs_diff(window[1]))))
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let safe_reports: usize = reader
        .lines()
        .map(Result::unwrap)
        .filter(|line| {
            let report = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            if is_safe(&report) {
                return true;
            }

            if args.part == 2 {
                for skipped in 0..report.len() {
                    let mut with_skip = Vec::new();
                    with_skip.extend_from_slice(&report[0..skipped]);
                    with_skip.extend_from_slice(&report[skipped + 1..]);
                    if is_safe(&with_skip) {
                        return true;
                    }
                }
            }

            false
        })
        .count();

    println!("safe {safe_reports}");
}
