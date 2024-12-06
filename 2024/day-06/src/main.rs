#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (position.0 - 1, position.1),
            Self::Right => (position.0, position.1 + 1),
            Self::Down => (position.0 + 1, position.1),
            Self::Left => (position.0, position.1 - 1),
        }
    }

    fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

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

fn get_start_position(grid: &[Vec<u8>]) -> (usize, usize) {
    let mut position = (0, 0);
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            if *cell == b'^' {
                position = (row_index, column_index);
                break;
            }
        }
    }

    position
}

fn get_next(grid: &[Vec<u8>], position: (usize, usize), direction: Direction) -> u8 {
    let (next_row, next_column) = direction.step(position);
    grid[next_row][next_column]
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);
    let grid = parse_padded_grid(reader.lines().map(Result::unwrap));

    let mut position = get_start_position(&grid);
    let mut visited = HashSet::new();
    let mut direction = Direction::Up;
    while grid[position.0][position.1] != b'*' {
        visited.insert(position);
        while get_next(&grid, position, direction) == b'#' {
            direction = direction.rotate();
        }

        position = direction.step(position);
    }

    println!("{}", visited.len());
}
