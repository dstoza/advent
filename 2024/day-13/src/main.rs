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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Vector,
    button_b: Vector,
    prize: Vector,
}

impl Machine {
    fn new(button_a: Vector, button_b: Vector, prize: Vector) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn compute_minimum(&self, inflate: bool) -> i64 {
        const INFLATION_AMOUNT: i64 = 10_000_000_000_000;

        let prize_x = if inflate {
            self.prize.x + INFLATION_AMOUNT
        } else {
            self.prize.x
        };

        let prize_y = if inflate {
            self.prize.y + INFLATION_AMOUNT
        } else {
            self.prize.y
        };

        let numerator = prize_y * self.button_b.x - prize_x * self.button_b.y;
        let denominator = self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y;
        if numerator % denominator != 0 {
            return 0;
        }

        let a_presses = numerator / denominator;

        let numerator = prize_x - a_presses * self.button_a.x;
        if numerator % self.button_b.x != 0 {
            return 0;
        }

        let b_presses = numerator / self.button_b.x;

        if a_presses < 0 || b_presses < 0 {
            return 0;
        }

        a_presses * 3 + b_presses
    }
}

fn parse_button(line: &str, button_regex: &Regex) -> Vector {
    let [x, y] = button_regex
        .captures(line)
        .unwrap()
        .extract()
        .1
        .map(|value| value.parse().unwrap());
    Vector::new(x, y)
}

fn parse_machine(
    lines: &mut impl Iterator<Item = String>,
    button_regex: &Regex,
    prize_regex: &Regex,
) -> Machine {
    let button_a = parse_button(&lines.next().unwrap(), button_regex);
    let button_b = parse_button(&lines.next().unwrap(), button_regex);

    let [x, y] = prize_regex
        .captures(&lines.next().unwrap())
        .unwrap()
        .extract()
        .1
        .map(|value| value.parse().unwrap());
    let prize = Vector::new(x, y);

    Machine::new(button_a, button_b, prize)
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let button_regex = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machines = Vec::new();
    let mut lines = reader.lines().map(Result::unwrap);
    machines.push(parse_machine(&mut lines, &button_regex, &prize_regex));
    while lines.next().is_some() {
        machines.push(parse_machine(&mut lines, &button_regex, &prize_regex));
    }

    let minimum_tokens: i64 = machines
        .iter()
        .map(|machine| machine.compute_minimum(args.part == 2))
        .sum();
    println!("{minimum_tokens}");
}
