#![warn(clippy::pedantic)]

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Step {
    position: Vector,
    direction: Direction,
    visited: Vec<Vector>,
    cost: usize,
}

impl Step {
    fn new(position: Vector, direction: Direction) -> Self {
        Self {
            position,
            direction,
            visited: Vec::new(),
            cost: 0,
        }
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn shortest_path(
    grid: &[Vec<u8>],
    start: Vector,
    start_direction: Direction,
    end: Vector,
    end_direction: Direction,
) -> Option<usize> {
    let mut shortest = None;
    let mut best_to_cell = HashMap::new();
    let mut queue = BinaryHeap::from([Step::new(start, start_direction)]);
    while let Some(step) = queue.pop() {
        if let Some(best) = best_to_cell.get(&step.position) {
            if *best < step.cost {
                continue;
            }
        } else {
            best_to_cell.insert(step.position, step.cost);
        }

        if step.position == end && step.direction == end_direction {
            shortest = shortest
                .map(|minimum: usize| minimum.min(step.cost))
                .or(Some(step.cost));

            continue;
        }

        if let Some(minimum) = shortest {
            if step.cost > minimum {
                continue;
            }
        }

        for (direction, neighbor) in step.position.neighbors() {
            if step.visited.iter().any(|visited| *visited == neighbor) {
                continue;
            }

            let cell = grid[neighbor.row][neighbor.column];
            if cell != b'.' && cell != b'E' {
                continue;
            }

            let mut next_step = step.clone();
            next_step.cost += if direction == step.direction { 1 } else { 1001 };
            next_step.visited.push(step.position);
            next_step.position = neighbor;
            next_step.direction = direction;
            queue.push(next_step);
        }
    }

    shortest
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    println!("running part {}", args.part);

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

    let mut shortest: Option<usize> = None;
    for direction in Direction::all() {
        let length = shortest_path(&grid, start, Direction::East, end, direction);
        if let Some(length) = length {
            shortest = shortest.map_or(Some(length), |shortest| Some(shortest.min(length)));
        }
    }

    let shortest = shortest.unwrap();

    let mut middles = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (column, cell) in line.iter().enumerate() {
            if *cell == b'.' {
                middles.push(Vector::new(row, column));
            }
        }
    }

    let mut count = 2;
    for middle in middles {
        println!("{middle:?}");
        if Direction::all().iter().any(|middle_direction| {
            let Some(first) =
                shortest_path(&grid, start, Direction::East, middle, *middle_direction)
            else {
                return false;
            };

            let mut second: Option<usize> = None;
            for end_direction in Direction::all() {
                if let Some(length) =
                    shortest_path(&grid, middle, *middle_direction, end, end_direction)
                {
                    second = second.map_or(Some(length), |best| Some(best.min(length)));
                }
            }

            let Some(second) = second else {
                return false;
            };

            first + second == shortest
        }) {
            println!("{middle:?}");
            count += 1;
        }
    }

    println!("count {count}");
}
