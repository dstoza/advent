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
    #[arg(short, long)]
    max_distance: usize,

    #[arg(short, long, default_value_t = 0)]
    filter_distance: usize,

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

    fn neighbors(self, max_distance: usize, width: usize, height: usize) -> Vec<(usize, Self)> {
        let min_row = ((self.row as isize) - max_distance as isize).max(0) as usize;
        let max_row = (self.row + max_distance).min(height - 1);

        let mut neighbors = Vec::new();
        for row in min_row..=max_row {
            let remaining_distance = max_distance - row.abs_diff(self.row);
            let min_column = ((self.column as isize) - remaining_distance as isize).max(0) as usize;
            let max_column = (self.column + remaining_distance).min(width - 1);
            for column in min_column..=max_column {
                if row == self.row && column == self.column {
                    continue;
                }

                let distance = row.abs_diff(self.row) + column.abs_diff(self.column);
                neighbors.push((distance, Self::new(row, column)))
            }
        }

        neighbors
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

        for (_, neighbor) in current.neighbors(1, width, height) {
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
        for (distance, target) in step.neighbors(args.max_distance, width, height) {
            let Some(target_time) = steps.get(&target) else {
                continue;
            };

            if *target_time > from_time + distance {
                let skipped = *target_time - from_time - distance;
                cheats
                    .entry(skipped)
                    .and_modify(|count| *count += 1)
                    .or_insert(1usize);
            }
        }
    }

    let count = cheats
        .iter()
        .map(|(skipped, count)| {
            if *skipped >= args.filter_distance {
                *count
            } else {
                0
            }
        })
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
