#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
enum Rule {
    Indirect(Vec<Vec<u8>>),
    Direct(String),
}

struct MessageValidator {
    rules: Vec<Rule>,
}

impl MessageValidator {
    fn new() -> Self {
        let mut rules = Vec::new();
        rules.resize(256, Rule::Indirect(Vec::new()));
        Self { rules }
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

        if id == 8 {
            self.rules[8] = Rule::Indirect(vec![vec![42], vec![42, 8]]);
            return;
        } else if id == 11 {
            self.rules[11] = Rule::Indirect(vec![vec![42, 31], vec![42, 11, 31]]);
            return;
        }

        let contents = split.next().expect("Failed to find rule").trim();
        self.rules[id as usize] = match &contents[0..=0] {
            "\"" => Rule::Direct(String::from(&contents[1..=1])),
            _ => Rule::Indirect(MessageValidator::parse_indirect(&contents[..])),
        };
    }

    fn message_matches_rule(&self, rule: &Rule, message: &str) -> Vec<usize> {
        if message.is_empty() {
            return Vec::new();
        }

        match rule {
            Rule::Direct(string) => {
                if &message[0..string.len()] == string {
                    vec![1]
                } else {
                    Vec::new()
                }
            }
            Rule::Indirect(alternatives) => {
                let mut lengths = Vec::new();

                for alternative in alternatives {
                    let mut cursors = vec![0];
                    for child_id in alternative {
                        let mut new_cursors = Vec::new();
                        for cursor in cursors {
                            let lengths = self.message_matches_rule(
                                &self.rules[(*child_id) as usize],
                                &message[cursor..],
                            );
                            for length in lengths {
                                new_cursors.push(cursor + length);
                            }
                        }
                        cursors = new_cursors;
                        if cursors.is_empty() {
                            break;
                        }
                    }

                    lengths.append(&mut cursors);
                }

                lengths
            }
        }
    }

    fn message_is_valid(&self, message: &str) -> bool {
        let match_lengths = self.message_matches_rule(&self.rules[0], message);
        match_lengths.iter().any(|length| *length == message.len())
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
