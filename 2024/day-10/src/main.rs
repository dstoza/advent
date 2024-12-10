#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
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

fn parse_padded_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for line in lines {
        let mut bytes = vec![b'*'];
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'*');

        if grid.is_empty() {
            grid.push(vec![b'*'; bytes.len()]);
        }

        grid.push(bytes);
    }

    grid.push(grid[0].clone());

    grid
}

fn neighbors(row: usize, column: usize) -> [(usize, usize); 4] {
    [
        (row - 1, column),
        (row, column + 1),
        (row + 1, column),
        (row, column - 1),
    ]
}

fn extend(grid: &[Vec<u8>], paths: Vec<Vec<(usize, usize)>>) -> Vec<Vec<(usize, usize)>> {
    if paths.is_empty() {
        return Vec::new();
    }

    let (end_row, end_column) = paths[0].last().unwrap();
    let next_value = grid[*end_row][*end_column] + 1;

    let mut extended_paths = Vec::new();
    for path in paths {
        let (end_row, end_column) = path.last().unwrap();
        for (row, column) in neighbors(*end_row, *end_column) {
            if grid[row][column] == next_value {
                let mut extended_path = path.clone();
                extended_path.push((row, column));
                extended_paths.push(extended_path);
            }
        }
    }

    extended_paths
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let grid = parse_padded_grid(reader.lines().map(Result::unwrap));

    let mut score_sum = 0;
    let mut rating_sum = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if grid[row][column] == b'0' {
                let mut paths = vec![vec![(row, column)]];
                for _ in 0..9 {
                    paths = extend(&grid, paths);
                }

                score_sum += paths
                    .iter()
                    .map(|path| *path.last().unwrap())
                    .collect::<HashSet<_>>()
                    .len();
                rating_sum += paths.len();
            }
        }
    }

    println!("{score_sum} {rating_sum}");
}
