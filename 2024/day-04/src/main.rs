#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

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

fn is_xmas(
    grid: &[Vec<u8>],
    start_row: i32,
    start_column: i32,
    row_increment: i32,
    column_increment: i32,
) -> bool {
    if !(0..grid.len() as i32).contains(&(start_row + row_increment * 3)) {
        return false;
    }

    if !(0..grid[0].len() as i32).contains(&(start_column + column_increment * 3)) {
        return false;
    }

    let xmas = [b'X', b'M', b'A', b'S'];
    for step in 0..4 {
        if grid[(start_row + row_increment * step) as usize]
            [(start_column + column_increment * step) as usize]
            != xmas[step as usize]
        {
            return false;
        }
    }

    true
}

fn is_x_mas(grid: &[Vec<u8>], start_row: usize, start_column: usize) -> bool {
    if !(1..grid.len() - 1).contains(&start_row) {
        return false;
    }

    if !(1..grid[0].len() - 1).contains(&start_column) {
        return false;
    }

    if grid[start_row][start_column] != b'A' {
        return false;
    }

    match (
        grid[start_row - 1][start_column - 1],
        grid[start_row + 1][start_column + 1],
    ) {
        (b'M', b'S') | (b'S', b'M') => (),
        _ => return false,
    };

    match (
        grid[start_row + 1][start_column - 1],
        grid[start_row - 1][start_column + 1],
    ) {
        (b'M', b'S') | (b'S', b'M') => (),
        _ => return false,
    };

    true
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

    let mut count = 0;
    let directions = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    for start_row in 0..grid.len() {
        for start_column in 0..grid[start_row].len() {
            if args.part == 1 {
                for (row_increment, column_increment) in directions {
                    if is_xmas(
                        &grid,
                        start_row as i32,
                        start_column as i32,
                        row_increment,
                        column_increment,
                    ) {
                        count += 1;
                    }
                }
            } else if is_x_mas(&grid, start_row, start_column) {
                count += 1;
            }
        }
    }

    println!("{count}");
}
