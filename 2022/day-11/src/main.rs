#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Debug)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn from_string(string: &str) -> Self {
        let raw_operation = string.strip_prefix("  Operation: new = ").unwrap();
        if raw_operation == "old * old" {
            Operation::Square
        } else if raw_operation.starts_with("old *") {
            Operation::Multiply(
                raw_operation
                    .strip_prefix("old * ")
                    .unwrap()
                    .parse()
                    .unwrap(),
            )
        } else {
            Operation::Add(
                raw_operation
                    .strip_prefix("old + ")
                    .unwrap()
                    .parse()
                    .unwrap(),
            )
        }
    }

    fn apply(&self, value: &mut usize) {
        match self {
            Self::Multiply(m) => *value *= m,
            Self::Add(a) => *value += a,
            Self::Square => *value *= *value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_by: usize,
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    fn parse_from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        Self {
            items: lines
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation: Operation::from_string(&lines.next().unwrap()),
            divisible_by: lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
            on_true: lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
            on_false: lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}

fn parse_monkeys(mut lines: impl Iterator<Item = String>) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    while lines.next().is_some() {
        monkeys.push(Monkey::parse_from_lines(&mut lines));
        lines.next();
    }
    monkeys
}

fn simulate_round(monkeys: &mut [Monkey], modulo: Option<usize>) -> Vec<(usize, usize)> {
    let mut inspection_counts = Vec::new();

    for m in 0..monkeys.len() {
        let items: Vec<_> = monkeys[m].items.drain(..).collect();
        inspection_counts.push((m, items.len()));
        for mut item in items {
            let monkey = &mut monkeys[m];
            monkey.operation.apply(&mut item);

            if let Some(m) = modulo {
                item %= m;
            } else {
                item /= 3;
            }

            if item % monkey.divisible_by == 0 {
                monkeys[monkey.on_true].items.push(item);
            } else {
                monkeys[monkey.on_false].items.push(item);
            }
        }
    }

    inspection_counts
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let part = std::env::args()
        .nth(2)
        .expect("Part not found. Expected 'part1' or 'part2'");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let mut monkeys = parse_monkeys(lines);

    let (modulo, round_count) = if part == "part1" {
        (None, 20)
    } else {
        (
            Some(monkeys.iter().map(|monkey| monkey.divisible_by).product()),
            10_000,
        )
    };

    let mut inspection_counts = HashMap::new();
    for _ in 0..round_count {
        for (monkey, count) in simulate_round(&mut monkeys, modulo).drain(..) {
            *inspection_counts.entry(monkey).or_insert(0) += count;
        }
    }

    let mut inspection_counts: Vec<_> = inspection_counts.drain().collect();
    inspection_counts.sort_unstable_by_key(|(_monkey, counts)| *counts);
    inspection_counts.reverse();
    let monkey_business: usize = inspection_counts
        .iter()
        .take(2)
        .map(|(_monkey, counts)| *counts)
        .product();
    println!("Monkey business: {monkey_business}");
}
