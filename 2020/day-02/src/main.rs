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

fn password_is_valid(line: &str) -> bool {
    println!("checking {}", line.trim());
    let captures = PARSE_LINE
        .captures(&line)
        .expect(format!("Failed to match [{}]", &line).as_str());
    println!("min: {}", captures.get(1).expect("Failed to get min").as_str());
    println!("max: {}", captures.get(2).expect("Failed to get max").as_str());
    println!("char: {}", captures.get(3).expect("Failed to get char").as_str());
    println!("password: {}", captures.get(4).expect("Failed to get password").as_str());
    false
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
