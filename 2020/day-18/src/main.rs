#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
enum Command {
    Add,
    Multiply,
}

#[derive(Clone, Copy, Debug)]
struct Operation {
    command: Command,
    value: i64,
}

fn get_next_value(advanced: bool, expression: &str) -> (i64, usize) {
    match &expression[0..1] {
        "(" => evaluate_expression(advanced, &expression[1..]),
        _ => (
            expression[0..1]
                .parse()
                .expect("Failed to parse digit as i64"),
            1,
        ),
    }
}

fn flatten_operations(advanced: bool, operations: &[Operation]) -> i64 {
    if advanced {
        let mut reduced = Vec::new();
        reduced.push(operations[0]);
        for operation in operations.iter().skip(1) {
            match operation.command {
                Command::Add => {
                    reduced
                        .last_mut()
                        .expect("Failed to get last reduced element")
                        .value += operation.value
                }
                Command::Multiply => reduced.push(*operation),
            }
        }

        return flatten_operations(false, &reduced);
    }

    let result = operations
        .iter()
        .fold(0, |value, operation| match operation.command {
            Command::Add => value + operation.value,
            Command::Multiply => value * operation.value,
        });

    result
}

fn evaluate_expression(advanced: bool, expression: &str) -> (i64, usize) {
    let mut cursor = 0;
    let mut operations = Vec::new();

    let (value, advance) = get_next_value(advanced, expression);
    operations.push(Operation {
        command: Command::Add,
        value,
    });
    cursor += advance;

    while cursor < expression.len() {
        if &expression[cursor..=cursor] == ")" {
            return (flatten_operations(advanced, &operations), cursor + 2);
        }

        let command = match &expression[cursor..cursor + 3] {
            " + " => Command::Add,
            " * " => Command::Multiply,
            _ => panic!(
                "Unexpected continuation [{}]",
                &expression[cursor..cursor + 3]
            ),
        };

        let (value, advance) = get_next_value(advanced, &expression[cursor + 3..]);
        cursor += 3 + advance;
        operations.push(Operation { command, value })
    }

    (flatten_operations(advanced, &operations), cursor)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut new_math_sum = 0;
    let mut advanced_math_sum = 0;

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        {
            let (value, _) = evaluate_expression(false, line.trim());
            new_math_sum += value;
        }
        {
            let (value, _) = evaluate_expression(true, line.trim());
            advanced_math_sum += value;
        }

        line.clear();
    }

    println!("New math sum: {}", new_math_sum);
    println!("Advanced math sum: {}", advanced_math_sum);
}
