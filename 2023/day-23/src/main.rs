#![warn(clippy::pedantic)]

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Cursor {
    row: usize,
    column: usize,
}

impl Cursor {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.row > 0 {
                    Some(Self::new(self.row - 1, self.column))
                } else {
                    None
                }
            }
            Direction::East => Some(Self::new(self.row, self.column + 1)),
            Direction::South => Some(Self::new(self.row + 1, self.column)),
            Direction::West => {
                if self.column > 0 {
                    Some(Self::new(self.row, self.column - 1))
                } else {
                    None
                }
            }
        }
    }

    fn grid_value<T: Copy>(&self, grid: &[Vec<T>]) -> Option<T> {
        if self.row >= grid.len() || self.column >= grid[0].len() {
            return None;
        }

        Some(grid[self.row][self.column])
    }

    fn grid_value_mut<'a, T>(&self, grid: &'a mut [Vec<T>]) -> Option<&'a mut T> {
        if self.row >= grid.len() || self.column >= grid[0].len() {
            return None;
        }

        Some(&mut grid[self.row][self.column])
    }
}

fn get_entrances(grid: &[Vec<u8>], cursor: &Cursor) -> Vec<Cursor> {
    let mut entrances = Vec::new();

    if let Some(north) = cursor.step(Direction::North) {
        let value = north.grid_value(grid).unwrap();
        if value == b'v' {
            entrances.push(north);
        }
    }

    let east = cursor.step(Direction::East).unwrap();
    if let Some(value) = east.grid_value(grid) {
        if value == b'<' {
            entrances.push(east);
        }
    }

    let south = cursor.step(Direction::South).unwrap();
    if let Some(value) = south.grid_value(grid) {
        if value == b'^' {
            entrances.push(south);
        }
    }

    if let Some(west) = cursor.step(Direction::West) {
        let value = west.grid_value(grid).unwrap();
        if value == b'>' {
            entrances.push(west);
        }
    }

    entrances
}

fn get_exits(grid: &[Vec<u8>], cursor: &Cursor) -> Vec<Cursor> {
    let mut exits = Vec::new();

    if let Some(north) = cursor.step(Direction::North) {
        let value = north.grid_value(grid).unwrap();
        if value != b'#' && value != b'v' {
            exits.push(north);
        }
    }

    let east = cursor.step(Direction::East).unwrap();
    if let Some(value) = east.grid_value(grid) {
        if value != b'#' && value != b'<' {
            exits.push(east);
        }
    }

    let south = cursor.step(Direction::South).unwrap();
    if let Some(value) = south.grid_value(grid) {
        if value != b'#' && value != b'^' {
            exits.push(south);
        }
    }

    if let Some(west) = cursor.step(Direction::West) {
        let value = west.grid_value(grid).unwrap();
        if value != b'#' && value != b'>' {
            exits.push(west);
        }
    }

    exits
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut distances = vec![vec![0u16; grid[0].len()]; grid.len()];

    let start_column = grid[0].iter().position(|value| *value == b'.').unwrap();

    let mut queue = VecDeque::from([Cursor::new(0, start_column)]);
    while let Some(cursor) = queue.pop_front() {
        if cursor.row == grid.len() - 1 {
            break;
        }

        let remaining_entrances = get_entrances(&grid, &cursor)
            .iter()
            .filter(|entrance| entrance.grid_value(&distances).unwrap() == 0)
            .count();

        if remaining_entrances > 0 {
            *cursor.grid_value_mut(&mut distances).unwrap() = 0;
            continue;
        }

        let current_distance = cursor.grid_value(&distances).unwrap();

        for exit in get_exits(&grid, &cursor) {
            let exit_distance = exit.grid_value_mut(&mut distances).unwrap();
            if *exit_distance == 0 {
                *exit_distance = current_distance + 1;
                queue.push_back(exit);
            }
        }
    }

    let max_distance = distances
        .last()
        .unwrap()
        .iter()
        .find(|value| **value != 0)
        .unwrap();
    
    println!("{max_distance}");
}
