#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut top = vec![0, 0, 0];
    for line in reader.lines().map(std::result::Result::unwrap) {
        if let Ok(value) = line.parse::<i32>() {
            sum += value;
        } else {
            if sum > top[0] {
                top[0] = sum;
                top.sort_unstable();
            }
            sum = 0;
        }
    }

    let max = top[0];
    println!("max: {max}");
    let top_sum: i32 = top.iter().sum();
    println!("top: {top_sum}");
}
