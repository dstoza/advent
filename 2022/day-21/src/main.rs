#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

type Value = u64;

enum Operation {
    Constant(Value),
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

type Monkeys = HashMap<String, Operation>;

fn parse_monkeys(lines: impl Iterator<Item = String>) -> Monkeys {
    let mut monkeys = HashMap::new();

    for line in lines {
        let mut line_split = line.split(": ");
        let name = line_split.next().unwrap();
        let operation = line_split.next().unwrap();
        if let Ok(value) = operation.parse::<Value>() {
            monkeys.insert(String::from(name), Operation::Constant(value));
        } else {
            let mut operation_split = operation.split(' ');
            let lhs = operation_split.next().unwrap();
            match operation_split.next().unwrap() {
                "+" => monkeys.insert(
                    String::from(name),
                    Operation::Addition(
                        String::from(lhs),
                        String::from(operation_split.next().unwrap()),
                    ),
                ),
                "-" => monkeys.insert(
                    String::from(name),
                    Operation::Subtraction(
                        String::from(lhs),
                        String::from(operation_split.next().unwrap()),
                    ),
                ),
                "*" => monkeys.insert(
                    String::from(name),
                    Operation::Multiplication(
                        String::from(lhs),
                        String::from(operation_split.next().unwrap()),
                    ),
                ),
                "/" => monkeys.insert(
                    String::from(name),
                    Operation::Division(
                        String::from(lhs),
                        String::from(operation_split.next().unwrap()),
                    ),
                ),
                _ => unimplemented!(),
            };
        }
    }

    monkeys
}

fn compute_value(name: &String, monkeys: &Monkeys) -> Value {
    let operation = &monkeys[name];
    match operation {
        Operation::Constant(value) => *value,
        Operation::Addition(lhs, rhs) => compute_value(lhs, monkeys) + compute_value(rhs, monkeys),
        Operation::Subtraction(lhs, rhs) => {
            compute_value(lhs, monkeys).saturating_sub(compute_value(rhs, monkeys))
        }
        Operation::Multiplication(lhs, rhs) => {
            compute_value(lhs, monkeys) * compute_value(rhs, monkeys)
        }
        Operation::Division(lhs, rhs) => compute_value(lhs, monkeys) / compute_value(rhs, monkeys),
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn get_next_power_of_10(value: Value) -> Value {
    10f64.powf((value as f64).log10().ceil()) as u64
}

type GetValues = dyn Fn(&Monkeys) -> (Value, Value);

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let mut monkeys = parse_monkeys(lines);
    let root_value = compute_value(&String::from("root"), &monkeys);
    println!("Root value: {root_value}");

    let (lhs, rhs) = if let Operation::Addition(lhs, rhs) = &monkeys[&String::from("root")] {
        (lhs.clone(), rhs.clone())
    } else {
        unreachable!()
    };

    let left_value = compute_value(&lhs, &monkeys);
    let right_value = compute_value(&rhs, &monkeys);
    let get_values: Box<GetValues> = if left_value < right_value {
        Box::new(move |monkeys| (compute_value(&lhs, monkeys), compute_value(&rhs, monkeys)))
    } else {
        Box::new(move |monkeys| (compute_value(&rhs, monkeys), compute_value(&lhs, monkeys)))
    };

    let humn_value = compute_value(&String::from("humn"), &monkeys);
    let mut step = get_next_power_of_10(root_value);
    let mut additional = step;
    loop {
        monkeys.insert(
            String::from("humn"),
            Operation::Constant(humn_value + additional),
        );
        let (lesser, greater) = get_values(&monkeys);
        match lesser.cmp(&greater) {
            std::cmp::Ordering::Less => additional += step,
            std::cmp::Ordering::Greater => {
                additional -= step;
                step /= 10;
            }
            std::cmp::Ordering::Equal => break,
        };
    }

    println!("humn: {}", humn_value + additional);
}
