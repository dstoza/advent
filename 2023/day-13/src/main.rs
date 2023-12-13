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

enum Reflection {
    Row(usize),
    Column(usize),
}

fn find_line_reflections(line: &[Vec<u8>]) -> Vec<usize> {
    let mut reflections = Vec::new();
    for split in 1..line.len() {
        let mut zipped = line[0..split].iter().rev().zip(line[split..].iter());
        if zipped.all(|(l, r)| l == r) {
            reflections.push(split);
        }
    }
    reflections
}

fn find_all_reflections(rows: &[Vec<u8>]) -> Vec<Reflection> {
    let mut reflections = Vec::new();

    reflections.extend(
        find_line_reflections(rows)
            .iter()
            .map(|index| Reflection::Row(*index)),
    );

    let columns = transpose(rows);
    reflections.extend(
        find_line_reflections(&columns)
            .iter()
            .map(|index| Reflection::Column(*index)),
    );

    reflections
}

fn reflection_score(reflections: &[Reflection]) -> usize {
    reflections
        .iter()
        .map(|reflection| match reflection {
            Reflection::Row(value) => *value * 100,
            Reflection::Column(value) => *value,
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    let mut total = 0;
    for line in reader.lines().map(std::result::Result::unwrap) {
        if line.is_empty() {
            total += reflection_score(&find_all_reflections(&rows));
            rows.clear();
            continue;
        }

        rows.push(Vec::from(line.as_bytes()));
    }
    total += reflection_score(&find_all_reflections(&rows));

    println!("{total}");
}
