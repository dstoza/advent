#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::Range,
};

#[derive(Clone, Copy, Debug)]
struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn row_range(self, other: Self) -> Range<usize> {
        let range = self.row..other.row;
        if range.is_empty() {
            other.row..self.row
        } else {
            range
        }
    }

    fn column_range(self, other: Self) -> Range<usize> {
        let range = self.column..other.column;
        if range.is_empty() {
            other.column..self.column
        } else {
            range
        }
    }
}

fn total_distance(
    galaxies: &[Coordinates],
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion_factor: usize,
) -> usize {
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(start, from)| {
            galaxies.iter().skip(start).map(|to| {
                let row_range = from.row_range(*to);
                let rows = (row_range.end - row_range.start)
                    + expansion_factor
                        * empty_rows
                            .iter()
                            .filter(|row| row_range.contains(row))
                            .count();

                let column_range = from.column_range(*to);
                let columns = (column_range.end - column_range.start)
                    + expansion_factor
                        * empty_columns
                            .iter()
                            .filter(|column| column_range.contains(column))
                            .count();

                rows + columns
            })
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut empty_rows = Vec::new();
    let mut column_empty = Vec::new();
    let galaxies = reader
        .lines()
        .map(std::result::Result::unwrap)
        .enumerate()
        .flat_map(|(row, line)| {
            let line = line.as_bytes();

            if column_empty.is_empty() {
                column_empty = vec![true; line.len()];
            }

            let mut row_empty = true;
            let galaxies = line
                .iter()
                .enumerate()
                .filter_map(|(column, byte)| {
                    if *byte == b'#' {
                        row_empty = false;
                        column_empty[column] = false;
                        Some(Coordinates::new(row, column))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if row_empty {
                empty_rows.push(row);
            }

            galaxies
        })
        .collect::<Vec<_>>();

    let empty_columns = column_empty
        .iter()
        .enumerate()
        .filter_map(|(column, empty)| if *empty { Some(column) } else { None })
        .collect::<Vec<_>>();

    println!(
        "{}",
        total_distance(&galaxies, &empty_rows, &empty_columns, 1)
    );

    println!(
        "{}",
        total_distance(&galaxies, &empty_rows, &empty_columns, 999_999)
    );
}
