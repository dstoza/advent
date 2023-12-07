#![warn(clippy::pedantic)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn all() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.row > 0 {
                    Some(Coordinates::new(self.row - 1, self.column))
                } else {
                    None
                }
            }
            Direction::NorthEast => {
                if self.row > 0 {
                    Some(Coordinates::new(self.row - 1, self.column + 1))
                } else {
                    None
                }
            }
            Direction::East => Some(Coordinates::new(self.row, self.column + 1)),
            Direction::SouthEast => Some(Coordinates::new(self.row + 1, self.column + 1)),
            Direction::South => Some(Coordinates::new(self.row + 1, self.column)),
            Direction::SouthWest => {
                if self.column > 0 {
                    Some(Coordinates::new(self.row + 1, self.column - 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if self.column > 0 {
                    Some(Coordinates::new(self.row, self.column - 1))
                } else {
                    None
                }
            }
            Direction::NorthWest => {
                if self.row > 0 && self.column > 0 {
                    Some(Coordinates::new(self.row - 1, self.column - 1))
                } else {
                    None
                }
            }
        }
    }
}

trait At {
    fn at(&self, coordinates: Coordinates) -> Option<u8>;
}

impl At for &[Vec<u8>] {
    fn at(&self, coordinates: Coordinates) -> Option<u8> {
        if coordinates.row < self.len() && coordinates.column < self[coordinates.row].len() {
            Some(self[coordinates.row][coordinates.column])
        } else {
            None
        }
    }
}

impl At for &mut [Vec<u8>] {
    fn at(&self, coordinates: Coordinates) -> Option<u8> {
        (&(**self)).at(coordinates)
    }
}

trait Set {
    fn set(&mut self, coordinates: Coordinates, value: u8);
}

impl Set for &mut [Vec<u8>] {
    fn set(&mut self, coordinates: Coordinates, value: u8) {
        if coordinates.row < self.len() && coordinates.column < self[coordinates.row].len() {
            self[coordinates.row][coordinates.column] = value;
        }
    }
}

fn find_start(grid: &[Vec<u8>]) -> Option<Coordinates> {
    for (row, line) in grid.iter().enumerate() {
        for (column, byte) in line.iter().enumerate() {
            if *byte == b'S' {
                return Some(Coordinates::new(row, column));
            }
        }
    }
    None
}

fn can_enter(value: u8, from: Direction) -> bool {
    matches!(
        (value, from),
        (b'|', Direction::North | Direction::South)
            | (b'-', Direction::West | Direction::East)
            | (b'L', Direction::North | Direction::East)
            | (b'J', Direction::North | Direction::West)
            | (b'7', Direction::West | Direction::South)
            | (b'F', Direction::East | Direction::South)
    )
}

fn find_eligible_neighbors(grid: &[Vec<u8>], start: Coordinates) -> Vec<Direction> {
    Direction::all()
        .iter()
        .filter_map(|direction| {
            if let Some((value, from)) = start
                .step(*direction)
                .and_then(|coordinates| grid.at(coordinates))
                .map(|value| (value, direction.opposite()))
            {
                if can_enter(value, from) {
                    return Some(*direction);
                }
            }
            None
        })
        .collect()
}

#[derive(Clone)]
struct PipeIterator<'a> {
    grid: &'a [Vec<u8>],
    next: Coordinates,
    from: Direction,
}

impl<'a> PipeIterator<'a> {
    fn new(grid: &'a [Vec<u8>], first: Coordinates, from: Direction) -> Self {
        Self {
            grid,
            next: first,
            from,
        }
    }
}

fn pipe_direction(value: u8, from: Direction) -> Option<Direction> {
    match (value, from) {
        (b'|', Direction::North) | (b'7', Direction::West) | (b'F', Direction::East) => {
            Some(Direction::South)
        }
        (b'|', Direction::South) | (b'L', Direction::East) | (b'J', Direction::West) => {
            Some(Direction::North)
        }
        (b'-', Direction::West) | (b'L', Direction::North) | (b'F', Direction::South) => {
            Some(Direction::East)
        }
        (b'-', Direction::East) | (b'J', Direction::North) | (b'7', Direction::South) => {
            Some(Direction::West)
        }
        _ => None,
    }
}

impl<'a> Iterator for PipeIterator<'a> {
    type Item = (Coordinates, Direction);

    fn next(&mut self) -> Option<(Coordinates, Direction)> {
        let next = self.next;
        let from = self.from;
        let Some(value) = self.grid.at(self.next) else {
            return None;
        };
        let Some(direction) = pipe_direction(value, self.from) else {
            return None;
        };
        let Some(new_next) = next.step(direction) else {
            return None;
        };
        self.next = new_next;
        self.from = direction.opposite();
        Some((next, from))
    }
}

fn maybe_set(mut grid: &mut [Vec<u8>], coordinates: Option<Coordinates>, value: u8) {
    let Some(coordinates) = coordinates else {
        return;
    };

    if let Some(b'.') = grid.at(coordinates) {
        grid.set(coordinates, value);
    }
}

fn fill_tracker(mut grid: &mut [Vec<u8>], coordinates: Coordinates, value: u8, from: Direction) {
    grid.set(coordinates, b'*');

    match value {
        b'|' => {
            let (a, b) = match from {
                Direction::North => (b'L', b'R'),
                Direction::South => (b'R', b'L'),
                _ => unreachable!(),
            };
            for direction in [Direction::NorthEast, Direction::East, Direction::SouthEast] {
                maybe_set(grid, coordinates.step(direction), a);
            }
            for direction in [Direction::NorthWest, Direction::West, Direction::SouthWest] {
                maybe_set(grid, coordinates.step(direction), b);
            }
        }
        b'-' => {
            let (a, b) = match from {
                Direction::West => (b'L', b'R'),
                Direction::East => (b'R', b'L'),
                _ => unreachable!(),
            };
            for direction in [Direction::NorthWest, Direction::North, Direction::NorthEast] {
                maybe_set(grid, coordinates.step(direction), a);
            }
            for direction in [Direction::SouthWest, Direction::South, Direction::SouthEast] {
                maybe_set(grid, coordinates.step(direction), b);
            }
        }
        b'L' => {
            let (a, b) = match from {
                Direction::North => (b'L', b'R'),
                Direction::East => (b'R', b'L'),
                _ => unreachable!(),
            };
            maybe_set(grid, coordinates.step(Direction::NorthEast), a);
            for direction in [
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
            ] {
                maybe_set(grid, coordinates.step(direction), b);
            }
        }
        b'J' => {
            let (a, b) = match from {
                Direction::North => (b'L', b'R'),
                Direction::West => (b'R', b'L'),
                _ => unreachable!(),
            };
            for direction in [
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
            ] {
                maybe_set(grid, coordinates.step(direction), a);
            }
            maybe_set(grid, coordinates.step(Direction::NorthWest), b);
        }
        b'7' => {
            let (a, b) = match from {
                Direction::West => (b'L', b'R'),
                Direction::South => (b'R', b'L'),
                _ => unreachable!(),
            };
            for direction in [
                Direction::NorthWest,
                Direction::North,
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
            ] {
                maybe_set(grid, coordinates.step(direction), a);
            }
            maybe_set(grid, coordinates.step(Direction::SouthWest), b);
        }
        b'F' => {
            let (a, b) = match from {
                Direction::East => (b'L', b'R'),
                Direction::South => (b'R', b'L'),
                _ => unreachable!(),
            };
            maybe_set(grid, coordinates.step(Direction::SouthEast), a);
            for direction in [
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
                Direction::North,
                Direction::NorthEast,
            ] {
                maybe_set(grid, coordinates.step(direction), b);
            }
        }
        _ => (),
    }
}

fn expand(tracker: &mut [Vec<u8>]) {
    for row in 0..tracker.len() {
        for column in 0..tracker[0].len() {
            let coordinates = Coordinates::new(row, column);
            let value = tracker.at(coordinates).unwrap();
            if value == b'L' || value == b'R' {
                maybe_set(tracker, coordinates.step(Direction::North), value);
                maybe_set(tracker, coordinates.step(Direction::East), value);
                maybe_set(tracker, coordinates.step(Direction::South), value);
                maybe_set(tracker, coordinates.step(Direction::West), value);
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(std::string::String::into_bytes)
        .collect::<Vec<_>>();

    let start = find_start(&grid).unwrap();
    let eligible_neighbors = find_eligible_neighbors(&grid, start);

    assert!(eligible_neighbors.len() == 2);

    let iterators = eligible_neighbors
        .iter()
        .map(|neighbor| {
            let first = start.step(*neighbor).unwrap();
            let from = neighbor.opposite();
            PipeIterator::new(&grid, first, from)
        })
        .collect::<Vec<_>>();

    let iterator = iterators[0].clone().zip(iterators[1].clone());

    let mut steps = 0;
    let mut visited = HashSet::new();
    for ((a, _), (b, _)) in iterator {
        steps += 1;
        if !visited.insert(a) || !visited.insert(b) {
            break;
        }
    }

    println!("{steps}");

    let mut tracker = vec![vec![b'.'; grid[0].len()]; grid.len()];
    tracker.as_mut_slice().set(start, b'*');

    for (coordinates, from) in iterators[0].clone() {
        fill_tracker(
            &mut tracker,
            coordinates,
            grid.as_slice().at(coordinates).unwrap(),
            from,
        );
    }

    // for line in &tracker {
    //     for byte in line {
    //         print!("{}", *byte as char);
    //     }
    //     println!();
    // }

    for _ in 0..10 {
        expand(&mut tracker);
    }

    // for line in &tracker {
    //     for byte in line {
    //         print!("{}", *byte as char);
    //     }
    //     println!();
    // }

    let inside = tracker
        .iter()
        .flat_map(|line| line.iter())
        .filter(|b| **b == b'L')
        .count();
    println!("{inside}");
}