#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Condition {
    Greater(String, u16),
    Less(String, u16),
    Always,
}

impl Condition {
    fn parse(string: &str) -> Self {
        if string.contains('>') {
            let mut split = string.split('>');
            Self::Greater(
                String::from(split.next().unwrap()),
                split.next().and_then(|string| string.parse().ok()).unwrap(),
            )
        } else if string.contains('<') {
            let mut split = string.split('<');
            Self::Less(
                String::from(split.next().unwrap()),
                split.next().and_then(|string| string.parse().ok()).unwrap(),
            )
        } else {
            Self::Always
        }
    }
}

#[derive(Debug)]
enum Target {
    Workflow(String),
    Reject,
    Accept,
}

impl Target {
    fn parse(string: &str) -> Self {
        match string {
            "R" => Self::Reject,
            "A" => Self::Accept,
            other => Self::Workflow(String::from(other)),
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    target: Target,
}

impl Rule {
    fn parse(string: &str) -> Self {
        if string.contains(':') {
            let mut parts = string.split(':');
            let condition = Condition::parse(parts.next().unwrap());
            let target = Target::parse(parts.next().unwrap());
            Self { condition, target }
        } else {
            let target = Target::parse(string);
            Self {
                condition: Condition::Always,
                target,
            }
        }
    }
}

fn parse_workflow(string: &str) -> Vec<Rule> {
    let split = string.split(',');
    split.map(Rule::parse).collect()
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn new(x: u16, m: u16, a: u16, s: u16) -> Self {
        Self { x, m, a, s }
    }

    fn parse(string: &str) -> Self {
        let mut split = string.split(',');
        Self {
            x: split
                .next()
                .and_then(|string| string.trim_start_matches("x=").parse().ok())
                .unwrap(),
            m: split
                .next()
                .and_then(|string| string.trim_start_matches("m=").parse().ok())
                .unwrap(),
            a: split
                .next()
                .and_then(|string| string.trim_start_matches("a=").parse().ok())
                .unwrap(),
            s: split
                .next()
                .and_then(|string| string.trim_start_matches("s=").parse().ok())
                .unwrap(),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    for line in reader.lines().map(std::result::Result::unwrap) {
        if line.is_empty() {
            continue;
        }

        if line.starts_with('{') {
            parts.push(Part::parse(
                line.trim_start_matches('{').trim_end_matches('}'),
            ));
        } else {
            let mut split = line.split('{');
            let name = String::from(split.next().unwrap());
            let workflow = parse_workflow(split.next().unwrap().trim_end_matches('}'));
            workflows.insert(name, workflow);
        }
    }

    for workflow in workflows {
        println!("{workflow:?}");
    }

    for part in parts {
        println!("{part:?}");
    }
}
