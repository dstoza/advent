#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn transpose(rows: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut columns = vec![Vec::new(); rows[0].len()];
    for row in rows {
        for (column, byte) in row.iter().enumerate() {
            columns[column].push(*byte);
        }
    }
    columns
}

fn find_reflection(line: &[Vec<u8>]) -> Option<usize> {
    for split in 1..line.len() {
        let mut zipped = line[0..split].iter().rev().zip(line[split..].iter());
        if zipped.all(|(l, r)| l == r) {
            return Some(split);
        }
    }
    None
}

fn reflection_score(rows: &[Vec<u8>]) -> usize {
    if let Some(reflection) = find_reflection(rows) {
        return reflection * 100;
    }

    let columns = transpose(rows);
    if let Some(reflection) = find_reflection(&columns) {
        return reflection;
    }

    unreachable!()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    let mut total = 0;
    for line in reader.lines().map(std::result::Result::unwrap) {
        if line.is_empty() {
            total += reflection_score(&rows);
            rows.clear();
            continue;
        }

        rows.push(Vec::from(line.as_bytes()));
    }
    total += reflection_score(&rows);

    println!("{total}");
}
