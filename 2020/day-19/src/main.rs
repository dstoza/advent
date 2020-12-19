#![deny(clippy::all, clippy::pedantic)]

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Rule {
    Indirect(Vec<Vec<u8>>),
    Direct(String),
}

struct MessageValidator {
    rules: HashMap<u8, Rule>,
}

impl MessageValidator {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn parse_indirect(indirect: &str) -> Vec<Vec<u8>> {
        let split = indirect.split(" | ");
        let indirect: Vec<Vec<u8>> = split
            .map(|alternative| {
                let alternative = alternative.trim();
                let split = alternative.split(' ');
                let children: Vec<u8> = split
                    .map(|child| child.parse::<u8>().expect("Failed to fit child in u8"))
                    .collect();
                children
            })
            .collect();
        indirect
    }

    fn add_rule(&mut self, rule: &str) {
        let mut split = rule.split(':');

        let id: u8 = split
            .next()
            .expect("Failed to find ID in split")
            .parse()
            .expect("Failed to parse rule ID");

        /*
        if id == 8 {
            self.rules
                .insert(8, Rule::Indirect(vec![vec![42], vec![42, 8]]));
            return;
        } else if id == 11 {
            self.rules
                .insert(11, Rule::Indirect(vec![vec![42, 31], vec![42, 11, 31]]));
            return;
        }
        */

        let rule = split.next().expect("Failed to find rule").trim();
        self.rules.insert(
            id,
            match &rule[0..=0] {
                "\"" => Rule::Direct(String::from(&rule[1..=1])),
                _ => Rule::Indirect(MessageValidator::parse_indirect(&rule[..])),
            },
        );
    }

    fn message_matches_rule(&self, rule: &Rule, message: &str, level: i32) -> (bool, usize) {
        for _ in 0..level {
            print!("  ");
        }
        println!("message_matches_rule {:?} {}", rule, message);

        if message.is_empty() {
            return (false, 0);
        }

        match rule {
            Rule::Direct(string) => (&message[0..string.len()] == string, 1),
            Rule::Indirect(alternatives) => {
                for alternative in alternatives {
                    let mut cursor = 0;
                    let mut match_found = true;
                    for child_id in alternative {
                        let (child_matches, advance) = self.message_matches_rule(
                            &self.rules[child_id],
                            &message[cursor..],
                            level + 1,
                        );
                        if !child_matches {
                            match_found = false;
                            break;
                        }
                        cursor += advance;
                    }

                    if match_found {
                        return (true, cursor);
                    }
                }
                (false, 0)
            }
        }
    }

    fn message_is_valid(&self, message: &str) -> bool {
        let (matches, cursor) = self.message_matches_rule(&self.rules[&0], message, 0);
        matches && cursor == message.len()
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

    let mut validator = MessageValidator::new();

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        validator.add_rule(trimmed);

        line.clear();
    }

    let mut valid_messages = 0;

    line.clear();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        let message = line.trim();
        let valid = validator.message_is_valid(message);
        if valid {
            valid_messages += 1;
        }

        line.clear();
    }

    println!("{} valid messages", valid_messages);
}
