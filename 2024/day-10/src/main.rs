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

fn count_score(grid: &[Vec<u8>], row: usize, column: usize) -> usize {
    if grid[row][column] != b'0' {
        return 0;
    }

    let mut frontier = HashSet::from(neighbors(row, column));
    let mut next = b'1';

    while !frontier.is_empty() && next < b'9' {
        frontier = frontier
            .into_iter()
            .filter(|(row, column)| {
                (0..grid.len()).contains(row)
                    && (0..grid[*row].len()).contains(column)
                    && grid[*row][*column] == next
            })
            .flat_map(|(row, column)| neighbors(row, column))
            .collect::<HashSet<_>>();

        next += 1;
    }

    frontier
        .into_iter()
        .filter(|(row, column)| grid[*row][*column] == b'9')
        .count()
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    println!("running part {}", args.part);

    let grid = parse_padded_grid(reader.lines().map(Result::unwrap));

    let mut sum = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            sum += count_score(&grid, row, column);
        }
    }

    println!("{sum}");
}
