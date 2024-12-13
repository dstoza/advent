#![warn(clippy::pedantic)]

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Mul, Sub},
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
    x: u64,
    y: u64,
}

impl Vector {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    fn steps_to(&self, destination: &Self) -> Option<u64> {
        if self.x > destination.x || self.y > destination.y {
            return None;
        }

        if destination.x % self.x != 0 || destination.y % self.y != 0 {
            return None;
        }

        let x_steps = destination.x / self.x;
        if self.y * x_steps == destination.y {
            Some(x_steps)
        } else {
            None
        }
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x > other.x || self.y > other.y {
            Ordering::Greater
        } else if self.x == other.x && self.y == other.y {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Mul<u64> for Vector {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
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

    fn minimum_tokens(&self) -> u64 {
        (0..=100)
            .filter_map(|a_presses| {
                let distance = self.button_a * a_presses;
                if distance > self.prize {
                    return None;
                }

                let remaining = self.prize - distance;
                self.button_b
                    .steps_to(&remaining)
                    .map(|b_presses| a_presses * 3 + b_presses)
            })
            .min()
            .unwrap_or(0)
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

    let minimum_tokens: u64 = machines.iter().map(Machine::minimum_tokens).sum();
    println!("{minimum_tokens}");
}
