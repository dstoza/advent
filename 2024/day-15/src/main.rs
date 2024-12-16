#![warn(clippy::pedantic)]
#![allow(clippy::match_on_vec_items)]

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

fn expand_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut expanded = Vec::new();
    for line in grid {
        let mut expanded_line: Vec<u8> = Vec::new();
        for byte in line {
            match *byte {
                b'#' => expanded_line.extend_from_slice(b"##"),
                b'O' => expanded_line.extend_from_slice(b"[]"),
                b'.' => expanded_line.extend_from_slice(b".."),
                b'@' => expanded_line.extend_from_slice(b"@."),
                _ => unreachable!(),
            }
        }
        expanded.push(expanded_line);
    }

    expanded
}

fn push(grid: &mut [Vec<u8>], row: usize, column: usize, direction: u8, dry_run: bool) -> bool {
    if grid[row][column] == b'#' {
        return false;
    }

    if grid[row][column] == b'.' {
        return true;
    }

    match direction {
        b'^' | b'v' => {
            let other_row = if direction == b'^' { row - 1 } else { row + 1 };
            match grid[row][column] {
                b'[' | b']' => {
                    let other_column = if grid[row][column] == b'[' {
                        column + 1
                    } else {
                        column - 1
                    };

                    if push(grid, other_row, column, direction, dry_run)
                        && push(grid, other_row, other_column, direction, dry_run)
                    {
                        if !dry_run {
                            grid[other_row][column] = grid[row][column];
                            grid[other_row][other_column] = grid[row][other_column];
                            grid[row][column] = b'.';
                            grid[row][other_column] = b'.';
                        }
                        return true;
                    }
                }
                b'O' => {
                    if push(grid, other_row, column, direction, dry_run) {
                        if !dry_run {
                            grid[other_row][column] = grid[row][column];
                            grid[row][column] = b'.';
                        }
                        return true;
                    }
                }
                _ => unreachable!(),
            }
        }
        b'<' | b'>' => {
            let other_column = if direction == b'<' {
                column - 1
            } else {
                column + 1
            };
            if push(grid, row, other_column, direction, dry_run) {
                if !dry_run {
                    grid[row][other_column] = grid[row][column];
                    grid[row][column] = b'.';
                }
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
            if *cell == b'O' || *cell == b'[' {
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
    let grid = parse_grid(&mut lines);
    let mut grid = if args.part == 2 {
        expand_grid(&grid)
    } else {
        grid
    };

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
                b'^' | b'v' => {
                    let other_row = if direction == b'^' { row - 1 } else { row + 1 };
                    if push(&mut grid, other_row, column, direction, true) {
                        push(&mut grid, other_row, column, direction, false);
                        grid[other_row][column] = grid[row][column];
                        grid[row][column] = b'.';
                        row = other_row;
                    }
                }
                b'<' | b'>' => {
                    let other_column = if direction == b'<' {
                        column - 1
                    } else {
                        column + 1
                    };
                    if push(&mut grid, row, other_column, direction, true) {
                        push(&mut grid, row, other_column, direction, false);
                        grid[row][other_column] = grid[row][column];
                        grid[row][column] = b'.';
                        column = other_column;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let sum = coordinate_sum(&grid);
    println!("{sum}");
}
