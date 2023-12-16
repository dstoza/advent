#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const BLANK: u8 = b'X';

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Beam {
    row: usize,
    column: usize,
    direction: Direction,
}

impl Beam {
    fn new(row: usize, column: usize, direction: Direction) -> Self {
        Self {
            row,
            column,
            direction,
        }
    }

    fn split(&self, direction: Direction) -> Self {
        let mut split = *self;
        split.direction = direction;
        split.step(split.direction);
        split
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
            Direction::Left => self.column -= 1,
            Direction::Right => self.column += 1,
        }
    }

    fn energize(&mut self, grid: &[Vec<u8>], visited: &mut [Vec<u8>]) {
        if visited[self.row][self.column] & self.direction as u8 != 0 {
            return;
        }

        visited[self.row][self.column] |= self.direction as u8;

        #[allow(clippy::match_on_vec_items)]
        match grid[self.row][self.column] {
            b'.' => {
                self.step(self.direction);
                self.energize(grid, visited);
            }
            b'/' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                self.step(self.direction);
                self.energize(grid, visited);
            }
            b'\\' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                self.step(self.direction);
                self.energize(grid, visited);
            }
            b'|' => match self.direction {
                Direction::Up | Direction::Down => {
                    self.step(self.direction);
                    self.energize(grid, visited);
                }
                Direction::Left | Direction::Right => {
                    self.split(Direction::Up).energize(grid, visited);
                    self.split(Direction::Down).energize(grid, visited);
                }
            },
            b'-' => match self.direction {
                Direction::Left | Direction::Right => {
                    self.step(self.direction);
                    self.energize(grid, visited);
                }
                Direction::Up | Direction::Down => {
                    self.split(Direction::Left).energize(grid, visited);
                    self.split(Direction::Right).energize(grid, visited);
                }
            },
            _ => (),
        }
    }
}

fn count_energized(mut from: Beam, grid: &[Vec<u8>]) -> usize {
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    from.energize(grid, &mut visited);

    let mut count = 0;
    for row in &visited[1..visited.len() - 1] {
        count += bytecount::count(&row[1..row.len() - 1], 0);
    }
    (visited.len() - 2) * (visited[0].len() - 2) - count
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    let mut padding = None;
    for line in reader.lines().map(std::result::Result::unwrap) {
        padding = padding.or_else(|| {
            let p = vec![BLANK; line.len() + 2];
            grid.push(p.clone());
            Some(p)
        });

        let mut line = Vec::from(line.as_bytes());
        line.insert(0, BLANK);
        line.push(BLANK);
        grid.push(line);
    }
    grid.push(padding.unwrap());

    println!(
        "{}",
        count_energized(Beam::new(1, 1, Direction::Right), &grid)
    );

    let first_row = 1;
    let last_row = grid.len() - 2;
    let first_column = 1;
    let last_column = grid[0].len() - 2;

    let mut max = 0;
    for row in first_row..=last_row {
        max = max.max(count_energized(
            Beam::new(row, first_column, Direction::Right),
            &grid,
        ));
        max = max.max(count_energized(
            Beam::new(row, last_column, Direction::Left),
            &grid,
        ));
    }
    for column in first_column..=last_column {
        max = max.max(count_energized(
            Beam::new(first_row, column, Direction::Down),
            &grid,
        ));
        max = max.max(count_energized(
            Beam::new(last_row, column, Direction::Up),
            &grid,
        ));
    }

    println!("{max}");
}
