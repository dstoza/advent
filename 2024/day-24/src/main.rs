#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
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

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn from_str(string: &str) -> Self {
        match string {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gate {
    a: String,
    op: Op,
    b: String,
    out: String,
}

impl Gate {
    fn parse(line: String) -> Self {
        let mut split = line.split_whitespace();
        let a = split.next().unwrap().to_owned();
        let op = Op::from_str(split.next().unwrap());
        let b = split.next().unwrap().to_owned();
        let out = split.nth(1).unwrap().to_owned();
        Self { a, op, b, out }
    }

    fn resolve(&self, gates: &HashMap<String, bool>) -> Option<bool> {
        let a = gates.get(&self.a)?;
        let b = gates.get(&self.b)?;
        match self.op {
            Op::And => Some(*a && *b),
            Op::Or => Some(*a || *b),
            Op::Xor => Some(*a ^ *b),
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(Result::unwrap);

    let mut wires = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut split = line.split(": ");
        let name = split.next().unwrap().to_owned();
        let value = split.next().unwrap().parse::<u8>().unwrap() != 0;
        wires.insert(name, value);
    }

    let mut gates = lines.map(Gate::parse).collect::<VecDeque<_>>();
    while let Some(gate) = gates.pop_front() {
        if let Some(result) = gate.resolve(&wires) {
            wires.insert(gate.out, result);
        } else {
            gates.push_back(gate);
        }
    }

    let mut z_wires = wires
        .iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .collect::<Vec<_>>();
    z_wires.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let mut result = 0u64;
    for bit in z_wires.iter().rev().map(|(_, value)| **value) {
        result = result << 1 | u64::from(bit);
    }

    println!("{result}");
}
