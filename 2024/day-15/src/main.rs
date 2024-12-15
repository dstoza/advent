#![warn(clippy::pedantic)]

use std::{
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

fn parse_grid(lines: &mut impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_owned())
        .collect()
}

fn push(grid: &mut [Vec<u8>], row: usize, column: usize, direction: u8) -> bool {
    if grid[row][column] == b'#' {
        return false;
    }

    if grid[row][column] == b'.' {
        return true;
    }

    match direction {
        b'^' => {
            if push(grid, row - 1, column, b'^') {
                grid[row - 1][column] = grid[row][column];
                grid[row][column] = b'.';
                return true;
            }
        }
        b'>' => {
            if push(grid, row, column + 1, b'>') {
                grid[row][column + 1] = grid[row][column];
                grid[row][column] = b'.';
                return true;
            }
        }
        b'v' => {
            if push(grid, row + 1, column, b'v') {
                grid[row + 1][column] = grid[row][column];
                grid[row][column] = b'.';
                return true;
            }
        }
        b'<' => {
            if push(grid, row, column - 1, b'<') {
                grid[row][column - 1] = grid[row][column];
                grid[row][column] = b'.';
                return true;
            }
        }
        _ => unreachable!(),
    }

    false
}

fn coordinate_sum(grid: &[Vec<u8>]) -> usize {
    let mut sum = 0;
    for (row, line) in grid.iter().enumerate() {
        for (column, cell) in line.iter().enumerate() {
            if *cell == b'O' {
                sum += 100 * row + column;
            }
        }
    }

    sum
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(Result::unwrap);
    let mut grid = parse_grid(&mut lines);

    let mut position = (0, 0);
    for (row, line) in grid.iter().enumerate() {
        for (column, cell) in line.iter().enumerate() {
            if *cell == b'@' {
                position = (row, column);
            }
        }
    }

    let (mut row, mut column) = position;
    for line in lines.map(|line| line.as_bytes().to_owned()) {
        for direction in line {
            match direction {
                b'^' => {
                    if push(&mut grid, row - 1, column, b'^') {
                        grid[row - 1][column] = grid[row][column];
                        grid[row][column] = b'.';
                        row -= 1;
                    }
                }
                b'>' => {
                    if push(&mut grid, row, column + 1, b'>') {
                        grid[row][column + 1] = grid[row][column];
                        grid[row][column] = b'.';
                        column += 1;
                    }
                }
                b'v' => {
                    if push(&mut grid, row + 1, column, b'v') {
                        grid[row + 1][column] = grid[row][column];
                        grid[row][column] = b'.';
                        row += 1;
                    }
                }
                b'<' => {
                    if push(&mut grid, row, column - 1, b'<') {
                        grid[row][column - 1] = grid[row][column];
                        grid[row][column] = b'.';
                        column -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let sum = coordinate_sum(&grid);
    println!("{sum}");
}
