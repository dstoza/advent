#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[macro_use]
extern crate lazy_static;

use regex::{Captures, Regex};

lazy_static! {
    static ref PARSE_LINE: Regex =
        Regex::new(r"(\d+)-(\d+) (.): (.*)").expect("Failed to compile regular expression");
}

#[derive(Clone, Copy)]
enum PolicyType {
    Range,
    Position,
}

trait Policy {
    fn allows(&self, password: &str) -> bool;
}

struct PositionPolicy {
    first: usize,
    second: usize,
    character: u8,
}

impl PositionPolicy {
    fn new(captures: &Captures) -> Self {
        Self {
            first: captures
                .get(1)
                .expect("Failed to parse first")
                .as_str()
                .parse::<usize>()
                .expect("Failed to parse first as usize"),
            second: captures
                .get(2)
                .expect("Failed to parse second")
                .as_str()
                .parse::<usize>()
                .expect("Failed to parse second as usize"),
            character: captures
                .get(3)
                .expect("Failed to parse character")
                .as_str()
                .as_bytes()[0],
        }
    }
}

impl Policy for PositionPolicy {
    fn allows(&self, password: &str) -> bool {
        let first_matches = password.as_bytes()[self.first - 1] == self.character;
        let second_matches = password.as_bytes()[self.second - 1] == self.character;
        first_matches ^ second_matches
    }
}

struct RangePolicy {
    min: usize,
    max: usize,
    character: u8,
}

impl RangePolicy {
    fn new(captures: &Captures) -> Self {
        Self {
            min: captures
                .get(1)
                .expect("Failed to parse min")
                .as_str()
                .parse::<usize>()
                .expect("Failed to parse min as usize"),
            max: captures
                .get(2)
                .expect("Failed to parse max")
                .as_str()
                .parse::<usize>()
                .expect("Failed to parse max as usize"),
            character: captures
                .get(3)
                .expect("Failed to parse character")
                .as_str()
                .as_bytes()[0],
        }
    }
}

impl Policy for RangePolicy {
    fn allows(&self, password: &str) -> bool {
        let mut count = 0_usize;
        for c in password.as_bytes() {
            if *c == self.character {
                count += 1;
            }
            if count > self.max {
                return false;
            }
        }
        count >= self.min
    }
}

fn password_is_valid(line: &str, policy_type: PolicyType) -> bool {
    let captures = PARSE_LINE
        .captures(&line)
        .unwrap_or_else(|| panic!("Failed to match [{}]", line));

    let policy = {
        match policy_type {
            PolicyType::Position => Box::new(PositionPolicy::new(&captures)) as Box<dyn Policy>,
            PolicyType::Range => Box::new(RangePolicy::new(&captures)) as Box<dyn Policy>,
        }
    };
    let password = captures.get(4).expect("Failed to parse password").as_str();

    policy.allows(password)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }

    let policy_type = match args[2].as_str() {
        "position" => PolicyType::Position,
        "range" => PolicyType::Range,
        _ => panic!("Unexpected policy type {}", args[2].as_str()),
    };

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut valid_password_count = 0;
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        if password_is_valid(&line, policy_type) {
            valid_password_count += 1;
        }

        line.clear();
    }

    println!("{} valid passwords", valid_password_count);
}
