use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref PARSE_LINE: Regex =
        Regex::new(r"(\d+)-(\d+) (.): (.*)").expect("Failed to compile regular expression");
}

struct Policy {
    min: usize,
    max: usize,
    character: char,
}

impl Policy {
    fn allows(&self, password: &str) -> bool {
        let mut count = 0usize;
        for c in password.chars() {
            if c == self.character {
                count += 1;
            }
            if count > self.max {
                return false;
            }
        }
        count >= self.min
    }
}

fn password_is_valid(line: &str) -> bool {
    let captures = PARSE_LINE
        .captures(&line)
        .expect(format!("Failed to match [{}]", &line).as_str());

    let policy = Policy {
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
            .chars()
            .nth(0)
            .expect("Failed to find character"),
    };
    let password = captures.get(4).expect("Failed to parse password").as_str();

    policy.allows(password)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut valid_password_count = 0;
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        if password_is_valid(&line) {
            valid_password_count += 1;
        }

        line.clear();
    }

    println!("{} valid passwords", valid_password_count);
}
