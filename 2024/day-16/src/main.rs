#![warn(clippy::pedantic)]

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    fn turns_to(self, other: Self) -> usize {
        match (self, other) {
            (Self::North, Self::North)
            | (Self::East, Self::East)
            | (Self::South, Self::South)
            | (Self::West, Self::West) => 0,
            (Self::North, Self::South)
            | (Self::East, Self::West)
            | (Self::South, Self::North)
            | (Self::West, Self::East) => 2,
            _ => 1,
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
    row: usize,
    column: usize,
}

impl Vector {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn neighbors(&self) -> [(Direction, Self); 4] {
        [
            (Direction::North, Vector::new(self.row - 1, self.column)),
            (Direction::East, Vector::new(self.row, self.column + 1)),
            (Direction::South, Vector::new(self.row + 1, self.column)),
            (Direction::West, Vector::new(self.row, self.column - 1)),
        ]
    }
}

fn all_shortest_paths(
    grid: &[Vec<u8>],
    start: Vector,
    start_direction: Direction,
) -> Vec<Vec<Vec<usize>>> {
    let mut costs = vec![vec![vec![usize::MAX; Direction::all().len()]; grid[0].len()]; grid.len()];
    costs[start.row][start.column][start_direction as usize] = 0;

    let mut queue = VecDeque::from([(start, start_direction)]);
    while let Some((position, direction)) = queue.pop_front() {
        let cost = costs[position.row][position.column][direction as usize];
        for (neighbor_direction, neighbor) in position.neighbors() {
            if grid[neighbor.row][neighbor.column] == b'#' {
                continue;
            }

            let cost_to_neighbor = cost + 1 + 1000 * direction.turns_to(neighbor_direction);
            if costs[neighbor.row][neighbor.column][neighbor_direction as usize] > cost_to_neighbor
            {
                costs[neighbor.row][neighbor.column][neighbor_direction as usize] =
                    cost_to_neighbor;
                queue.push_back((neighbor, neighbor_direction));
            }
        }
    }

    costs
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(Result::unwrap)
        .map(String::into_bytes)
        .collect::<Vec<_>>();

    let mut start = None;
    let mut end = None;
    for (row, line) in grid.iter().enumerate() {
        for (column, cell) in line.iter().enumerate() {
            if *cell == b'S' {
                start = Some(Vector::new(row, column));
            } else if *cell == b'E' {
                end = Some(Vector::new(row, column));
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let paths_from_start = all_shortest_paths(&grid, start, Direction::East);
    let shortest = *paths_from_start[end.row][end.column].iter().min().unwrap();

    println!("{shortest}");

    let end_direction = Direction::all()
        .map(|direction| {
            (
                direction,
                paths_from_start[end.row][end.column][direction as usize],
            )
        })
        .into_iter()
        .min_by_key(|(_direction, cost)| *cost)
        .unwrap()
        .0
        .opposite();

    let paths_from_end = all_shortest_paths(&grid, end, end_direction);

    let mut intersections = HashSet::new();

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            if grid[row][column] != b'.' {
                continue;
            }

            for to_direction in Direction::all() {
                for from_direction in Direction::all() {
                    let cost_to = paths_from_start[row][column][to_direction as usize];
                    let cost_from = paths_from_end[row][column][from_direction as usize];
                    if cost_to == usize::MAX || cost_from == usize::MAX {
                        continue;
                    }

                    if cost_to + cost_from + 1000 * to_direction.turns_to(from_direction.opposite())
                        == shortest
                    {
                        intersections.insert(Vector::new(row, column));
                    }
                }
            }
        }
    }

    println!("{}", intersections.len() + 2);
}
