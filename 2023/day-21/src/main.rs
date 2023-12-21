#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    fn neighbors(self) -> Vec<Self> {
        let mut neighbors = vec![
            Self::new(self.row, self.column + 1),
            Self::new(self.row + 1, self.column),
        ];
        if self.column > 0 {
            neighbors.push(Self::new(self.row, self.column - 1));
        }
        if self.row > 0 {
            neighbors.push(Self::new(self.row - 1, self.column));
        }
        neighbors
    }

    fn get_value(self, grid: &[Vec<u8>]) -> Option<u8> {
        if self.row >= grid.len() || self.column >= grid[0].len() {
            return None;
        }

        Some(grid[self.row][self.column])
    }
}

impl Coordinates {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(column, value)| (row, column, *value))
        })
        .find(|(_, _, value)| *value == b'S')
        .map(|(row, column, _)| Coordinates::new(row, column))
        .unwrap();

    let mut first_seen = HashMap::new();
    let mut open = vec![start];
    for step in 1..=64 {
        let mut next = Vec::new();
        for neighbor in open.into_iter().flat_map(Coordinates::neighbors) {
            let Some(value) = neighbor.get_value(&grid) else {
                continue;
            };

            if value != b'#' && !first_seen.contains_key(&neighbor) {
                first_seen.insert(neighbor, step);
                next.push(neighbor);
            }
        }
        open = next;
    }

    let plots = first_seen
        .iter()
        .filter(|(_, first)| **first % 2 == 0)
        .count();
    println!("{plots}");
}
