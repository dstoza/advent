#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const BLANK: u8 = b'X';

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    fn advance(&mut self, grid: &[Vec<u8>], visited: &mut HashSet<Beam>) -> Vec<Self> {
        if visited.contains(self) {
            return Vec::new();
        }

        visited.insert(*self);

        match grid[self.row].get(self.column).unwrap() {
            b'.' => {
                self.step(self.direction);
                vec![*self]
            }
            b'/' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                self.step(self.direction);
                vec![*self]
            }
            b'\\' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                self.step(self.direction);
                vec![*self]
            }
            b'|' => match self.direction {
                Direction::Up | Direction::Down => {
                    self.step(self.direction);
                    vec![*self]
                }
                Direction::Left | Direction::Right => {
                    vec![self.split(Direction::Up), self.split(Direction::Down)]
                }
            },
            b'-' => match self.direction {
                Direction::Left | Direction::Right => {
                    self.step(self.direction);
                    vec![*self]
                }
                Direction::Up | Direction::Down => {
                    vec![self.split(Direction::Left), self.split(Direction::Right)]
                }
            },
            &BLANK => Vec::new(),
            _ => unimplemented!(),
        }
    }
}

fn count_energized(from: Beam, grid: &[Vec<u8>]) -> usize {
    let mut beams = vec![from];
    let mut visited = HashSet::new();
    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .flat_map(|mut beam| beam.advance(grid, &mut visited))
            .collect::<Vec<_>>();
    }

    let visited = visited
        .iter()
        .map(|beam| (beam.row, beam.column))
        .collect::<HashSet<_>>();
    visited
        .iter()
        .filter(|(row, column)| {
            *row != 0 && *row != grid.len() - 1 && *column != 0 && *column != grid[0].len() - 1
        })
        .count()
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
