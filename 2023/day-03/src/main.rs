#![warn(clippy::pedantic)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn is_symbol(byte: u8) -> bool {
    !byte.is_ascii_digit() && byte != b'.'
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut padding_line = None;
    let mut board = Vec::new();
    for mut line in reader.lines().map(std::result::Result::unwrap) {
        if padding_line.is_none() {
            let mut padding = Vec::new();
            padding.resize(line.len() + 2, b'.');
            board.extend_from_slice(&padding);
            padding_line = Some(padding);
        }
        line.insert(0, '.');
        line.push('.');
        board.extend_from_slice(line.as_bytes());
    }
    board.extend_from_slice(padding_line.as_ref().unwrap());
    let width = padding_line.unwrap().len();

    let mut sum = 0;
    let mut value = None;
    let mut adjacent = false;
    let mut adjacent_asterisks = Vec::new();
    let mut adjacent_values = HashMap::new();
    for (row, chunk) in board.chunks(width).enumerate() {
        for (column, byte) in chunk.iter().enumerate() {
            if byte.is_ascii_digit() {
                if value.is_none() {
                    for r in row - 1..=row + 1 {
                        let b = board[r * width + column - 1];
                        adjacent |= is_symbol(b);
                        if b == b'*' {
                            adjacent_asterisks.push((r, column - 1));
                        }
                    }
                }

                for r in row - 1..=row + 1 {
                    let b = board[r * width + column];
                    adjacent |= is_symbol(b);
                    if b == b'*' {
                        adjacent_asterisks.push((r, column));
                    }
                }

                value = Some(value.map_or_else(
                    || u32::from(byte - b'0'),
                    |value| value * 10 + u32::from(byte - b'0'),
                ));
            } else if let Some(v) = value {
                for r in row - 1..=row + 1 {
                    let b = board[r * width + column];
                    adjacent |= is_symbol(b);
                    if b == b'*' {
                        adjacent_asterisks.push((r, column));
                    }
                }

                if adjacent {
                    sum += v;
                }

                for asterisk in &adjacent_asterisks {
                    adjacent_values
                        .entry(*asterisk)
                        .and_modify(|values: &mut Vec<u32>| values.push(v))
                        .or_insert_with(|| vec![v]);
                }

                value = None;
                adjacent = false;
                adjacent_asterisks.clear();
            }
        }
    }

    println!("{sum}");

    let gear_sum: u32 = adjacent_values
        .values()
        .filter_map(|values| {
            if values.len() == 2 {
                Some(values.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum();

    println!("{gear_sum}");
}
