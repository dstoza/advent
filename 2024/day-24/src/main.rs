#![warn(clippy::pedantic)]

use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Gate {
    a: String,
    op: Op,
    b: String,
    out: String,
}

impl Gate {
    fn new(mut a: String, op: Op, mut b: String, out: String) -> Self {
        if a.cmp(&b) == Ordering::Greater {
            std::mem::swap(&mut a, &mut b);
        }
        Self { a, op, b, out }
    }

    #[allow(clippy::needless_pass_by_value, reason = "Allows easier map")]
    fn parse(line: String) -> Self {
        let mut split = line.split_whitespace();
        let a = split.next().unwrap().to_owned();
        let op = Op::from_str(split.next().unwrap());
        let b = split.next().unwrap().to_owned();
        let out = split.nth(1).unwrap().to_owned();
        Self::new(a, op, b, out)
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

    fn almost_match(&self, other: &Self) -> Option<(String, String)> {
        if self.op != other.op {
            return None;
        }

        let score = u8::from(self.a == other.a)
            + u8::from(self.b == other.b)
            + u8::from(self.out == other.out);

        if score != 2 {
            return None;
        }

        if self.a != other.a {
            return Some((self.a.clone(), other.a.clone()));
        }

        if self.b != other.b {
            return Some((self.b.clone(), other.b.clone()));
        }

        if self.out != other.out {
            return Some((self.out.clone(), other.out.clone()));
        }

        unreachable!()
    }
}

fn get_output(gates: &[Gate], term: &Gate) -> String {
    gates
        .iter()
        .find_map(|gate| {
            if let Some((output, _)) = gate.almost_match(term) {
                return Some(output);
            }

            None
        })
        .unwrap()
}

fn find_flip(gates: &[Gate]) -> Option<(String, String)> {
    let carry_term = Gate::new(
        String::from("x00"),
        Op::And,
        String::from("y00"),
        String::new(),
    );
    let mut carry = gates
        .iter()
        .find_map(|gate| {
            if let Some((carry, _)) = gate.almost_match(&carry_term) {
                return Some(carry);
            }

            None
        })
        .unwrap();

    let mut bit = 1;
    loop {
        let x = format!("x{bit:02}");
        let y = format!("y{bit:02}");

        if !gates
            .iter()
            .any(|gate| gate.a == x && gate.op == Op::Xor && gate.b == y)
        {
            return None;
        }

        let sum = get_output(
            gates,
            &Gate::new(x.clone(), Op::Xor, y.clone(), String::new()),
        );

        let generate = get_output(gates, &Gate::new(x, Op::And, y, String::new()));

        let z = format!("z{bit:02}");
        let out = Gate::new(carry.clone(), Op::Xor, sum.clone(), z);
        if !gates.iter().any(|gate| *gate == out) {
            return Some(
                gates
                    .iter()
                    .find_map(|gate| gate.almost_match(&out))
                    .unwrap(),
            );
        }

        let propagate = get_output(gates, &Gate::new(carry, Op::And, sum, String::new()));
        carry = get_output(
            gates,
            &Gate::new(generate, Op::Or, propagate, String::new()),
        );

        bit += 1;
    }
}

fn apply_flip(gates: &mut [Gate], flip: (String, String)) {
    let (a, b) = flip;
    for gate in &mut *gates {
        if gate.out == a {
            gate.out = String::from("replace");
        }
        if gate.out == b {
            gate.out.clone_from(&a);
        }
    }
    for gate in gates {
        if gate.out == "replace" {
            gate.out.clone_from(&b);
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
    let mut gate_list = gates.iter().cloned().collect::<Vec<_>>();
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

    let mut flips = Vec::new();
    while let Some(flip) = find_flip(&gate_list) {
        flips.push(flip.0.clone());
        flips.push(flip.1.clone());
        apply_flip(&mut gate_list, flip);
    }

    flips.sort_unstable();
    println!("{}", flips.join(","));
}
