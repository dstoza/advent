#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::clippy::comparison_chain)]

use std::{
    collections::{HashMap, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct XmasValidator {
    preamble_length: usize,
    valid_sums: HashMap<i64, usize>,
    window: VecDeque<i64>,
    values: Vec<i64>,
}

impl XmasValidator {
    fn new(preamble_length: usize) -> Self {
        Self {
            preamble_length,
            valid_sums: HashMap::new(),
            window: VecDeque::new(),
            values: Vec::new(),
        }
    }

    fn remove_oldest(&mut self) {
        let removed = self
            .window
            .pop_front()
            .expect("Failed to pop front when at preamble length");
        for previous_value in &self.window {
            let sum = removed + previous_value;

            let sum_count = self
                .valid_sums
                .get_mut(&sum)
                .expect("Failed to find sum in map");

            *sum_count -= 1;
            if *sum_count == 0 {
                self.valid_sums.remove(&sum);
            }
        }
    }

    fn add_value(&mut self, value: i64) -> bool {
        self.values.push(value);

        let is_valid =
            self.window.len() < self.preamble_length || self.valid_sums.get(&value).is_some();

        if self.window.len() == self.preamble_length {
            self.remove_oldest();
        }

        for previous_value in &self.window {
            *self.valid_sums.entry(value + previous_value).or_default() += 1;
        }
        self.window.push_back(value);

        is_valid
    }

    fn find_weakness(&self, invalid_number: i64) -> i64 {
        let mut first = 0_usize;
        let mut last = 1_usize;
        let mut sum = self.values[first] + self.values[last];
        while sum != invalid_number {
            if sum < invalid_number {
                last += 1;
                sum += self.values[last];
            } else if sum > invalid_number {
                sum -= self.values[first];
                first += 1;
            }
        }

        let mut min = self.values[first];
        let mut max = self.values[first];
        for value in &self.values[first..=last] {
            min = min.min(*value);
            max = max.max(*value);
        }
        min + max
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut validator = XmasValidator::new(25);

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        let value = line.trim().parse().expect("Failed to parse line as i64");
        if !validator.add_value(value) {
            println!("First invalid value: {}", value);
            println!("Weakness: {}", validator.find_weakness(value));
            break;
        }

        line.clear();
    }
}
