#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

enum Match {
    Exact,
    Possible,
    None,
}

#[derive(Debug)]
enum Status {
    Exact(usize),
    Range(RangeInclusive<usize>),
}

impl Status {
    fn from_damaged(damaged: &[u8]) -> Vec<Self> {
        if damaged.is_empty() {
            return Vec::new();
        }

        let damaged = std::str::from_utf8(damaged)
            .unwrap()
            .trim_start_matches('.')
            .trim_end_matches('.')
            .as_bytes();

        let minimum = if damaged.starts_with(&[b'#']) {
            match damaged.iter().position(|b| *b == b'.' || *b == b'?') {
                Some(position) => position,
                None => damaged.len(),
            }
        } else {
            0
        };

        let maximum = match damaged[minimum..].iter().position(|b| *b == b'.') {
            Some(position) => minimum + position,
            None => minimum + damaged[minimum..].len(),
        };

        let mut result = if minimum == maximum {
            vec![Self::Exact(minimum)]
        } else {
            vec![Self::Range(minimum..=maximum)]
        };

        result.append(&mut Self::from_damaged(&damaged[maximum..]));

        result
    }

    fn from_known(known: &str) -> Vec<Self> {
        known
            .split(',')
            .map(|length| Self::Exact(length.parse().unwrap()))
            .collect()
    }

    fn get_match(damaged: &[Self], known: &[Self]) -> Match {
        for (damaged, known) in damaged.iter().zip(known.iter()) {
            match (damaged, known) {
                (Self::Exact(d), Self::Exact(k)) => {
                    if *d != *k {
                        return Match::None;
                    }
                }
                (Self::Range(range), Self::Exact(known)) => {
                    if *range.start() == 0 || range.contains(known) {
                        return Match::Possible;
                    }
                    return Match::None;
                }
                _ => (),
            }
        }

        if damaged.len() == known.len() {
            Match::Exact
        } else if damaged.len() > known.len()
            && damaged.iter().any(|d| matches!(d, Status::Range(_)))
        {
            Match::Possible
        } else {
            Match::None
        }
    }
}

fn replace_first_unknown(damaged: &[u8], with: u8) -> Vec<u8> {
    if let Some(position) = damaged.iter().position(|b| *b == b'?') {
        let mut replaced = Vec::from(&damaged[..position]);
        replaced.push(with);
        replaced.extend_from_slice(&damaged[position + 1..]);
        replaced
    } else {
        Vec::from(damaged)
    }
}

fn count_possibilities(damaged: &[u8], known: &[Status]) -> usize {
    let damaged_status = Status::from_damaged(damaged);
    match Status::get_match(&damaged_status, known) {
        Match::Exact => 1,
        Match::None => 0,
        Match::Possible => {
            let with_operational = replace_first_unknown(damaged, b'.');
            let with_damaged = replace_first_unknown(damaged, b'#');
            count_possibilities(with_operational.as_slice(), known)
                + count_possibilities(with_damaged.as_slice(), known)
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines().map(std::result::Result::unwrap) {
        let mut split = line.split_whitespace();
        let damaged = split.next().unwrap();
        let known = Status::from_known(split.next().unwrap());
        let possibilities = count_possibilities(damaged.as_bytes(), &known);
        sum += possibilities;
    }
    println!("{sum}");
}
