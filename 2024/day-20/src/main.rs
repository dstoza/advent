#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn neighbors(self) -> [Self; 4] {
        [
            Self::new(self.row + 1, self.column),
            Self::new(self.row - 1, self.column),
            Self::new(self.row, self.column + 1),
            Self::new(self.row, self.column - 1),
        ]
    }

    fn jumps(self, width: usize, height: usize) -> Vec<[Self; 2]> {
        let mut jumps = Vec::new();

        if self.row > 1 {
            jumps.push([
                Self::new(self.row - 1, self.column),
                Self::new(self.row - 2, self.column),
            ]);
        }
        if self.row < height - 2 {
            jumps.push([
                Self::new(self.row + 1, self.column),
                Self::new(self.row + 2, self.column),
            ]);
        }
        if self.column > 1 {
            jumps.push([
                Self::new(self.row, self.column - 1),
                Self::new(self.row, self.column - 2),
            ]);
        }
        if self.column < width - 2 {
            jumps.push([
                Self::new(self.row, self.column + 1),
                Self::new(self.row, self.column + 2),
            ]);
        }

        jumps
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.as_bytes().to_owned())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut start = None;
    let mut end = None;
    for (row, line) in grid.iter().enumerate() {
        for (column, cell) in line.iter().enumerate() {
            if *cell == b'S' {
                start = Some(Position::new(row, column));
            }
            if *cell == b'E' {
                end = Some(Position::new(row, column));
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let mut queue = VecDeque::from([start]);
    let mut path = Vec::new();
    while let Some(current) = queue.pop_front() {
        path.push(current);

        if current == end {
            break;
        }

        for neighbor in current.neighbors() {
            if path.iter().any(|previous| *previous == neighbor) {
                continue;
            }

            if grid[neighbor.row][neighbor.column] != b'#' {
                queue.push_back(neighbor);
            }
        }
    }

    let steps = path
        .iter()
        .enumerate()
        .map(|(index, position)| (*position, index))
        .collect::<HashMap<_, _>>();

    let mut cheats = HashMap::new();

    for step in path {
        let from_time = *steps.get(&step).unwrap();
        for jumps in step.jumps(width, height) {
            if steps.contains_key(&jumps[0]) {
                continue;
            }

            let Some(destination) = steps.get(&jumps[1]) else {
                continue;
            };

            if *destination > from_time {
                let skipped = *destination - from_time - 2;
                cheats
                    .entry(skipped)
                    .and_modify(|count| *count += 1)
                    .or_insert(1usize);
            }
        }
    }

    let count = cheats
        .iter()
        .map(|(skipped, count)| if *skipped >= 100 { *count } else { 0 })
        .sum::<usize>();

    println!("{count}");

    // let mut cheat_counts = cheats
    //     .iter()
    //     .map(|(skipped, count)| (*skipped, *count))
    //     .collect::<Vec<_>>();
    // cheat_counts.sort_unstable_by_key(|(skipped, _)| *skipped);
    // for count in cheat_counts {
    //     println!("{count:?}");
    // }
}
