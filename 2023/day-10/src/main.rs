#![warn(clippy::pedantic)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Direction {
    fn cardinal() -> [Self; 4] {
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

    fn next(self) -> Self {
        ((self as u8 + 1) % 8).into()
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::East,
            3 => Direction::SouthEast,
            4 => Direction::South,
            5 => Direction::SouthWest,
            6 => Direction::West,
            7 => Direction::NorthWest,
            _ => unreachable!(),
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
        let Some(row) = (match direction {
            Direction::NorthWest | Direction::North | Direction::NorthEast => {
                if self.row > 0 {
                    Some(self.row - 1)
                } else {
                    None
                }
            }
            Direction::West | Direction::East => Some(self.row),
            Direction::SouthWest | Direction::South | Direction::SouthEast => Some(self.row + 1),
        }) else {
            return None;
        };

        let Some(column) = (match direction {
            Direction::NorthWest | Direction::West | Direction::SouthWest => {
                if self.column > 0 {
                    Some(self.column - 1)
                } else {
                    None
                }
            }
            Direction::North | Direction::South => Some(self.column),
            Direction::NorthEast | Direction::East | Direction::SouthEast => Some(self.column + 1),
        }) else {
            return None;
        };

        Some(Coordinates::new(row, column))
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

fn get_connections(value: u8) -> Option<[Direction; 2]> {
    match value {
        b'|' => Some([Direction::North, Direction::South]),
        b'-' => Some([Direction::West, Direction::East]),
        b'L' => Some([Direction::North, Direction::East]),
        b'J' => Some([Direction::North, Direction::West]),
        b'7' => Some([Direction::West, Direction::South]),
        b'F' => Some([Direction::East, Direction::South]),
        _ => None,
    }
}

fn can_enter(value: u8, from: Direction) -> bool {
    let Some(connections) = get_connections(value) else {
        return false;
    };

    connections.into_iter().any(|direction| direction == from)
}

fn find_eligible_neighbors(grid: &[Vec<u8>], start: Coordinates) -> Vec<Direction> {
    Direction::cardinal()
        .iter()
        .filter_map(|direction| {
            start
                .step(*direction)
                .and_then(|coordinates| grid.at(coordinates))
                .and_then(|value| {
                    let from = direction.opposite();
                    if can_enter(value, from) {
                        Some(*direction)
                    } else {
                        None
                    }
                })
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
    get_connections(value)
        .and_then(|connections| connections.into_iter().find(|direction| *direction != from))
}

impl<'a> Iterator for PipeIterator<'a> {
    type Item = (Coordinates, Direction);

    fn next(&mut self) -> Option<(Coordinates, Direction)> {
        let next = self.next;
        let from = self.from;
        self.grid
            .at(self.next)
            .and_then(|value| pipe_direction(value, self.from))
            .and_then(|direction| next.step(direction).map(|new_next| (new_next, direction)))
            .map(|(new_next, direction)| {
                self.next = new_next;
                self.from = direction.opposite();
                (next, from)
            })
    }
}

fn maybe_set(mut grid: &mut [Vec<u8>], coordinates: Option<Coordinates>, value: u8) -> bool {
    let Some(coordinates) = coordinates else {
        return false;
    };

    if let Some(b'.') = grid.at(coordinates) {
        grid.set(coordinates, value);
        true
    } else {
        false
    }
}

fn fill_tracker(mut grid: &mut [Vec<u8>], coordinates: Coordinates, value: u8, from: Direction) {
    grid.set(coordinates, b'*');

    let to = pipe_direction(value, from).unwrap();

    let mut direction = from.next();
    while direction != to {
        maybe_set(grid, coordinates.step(direction), b'L');
        direction = direction.next();
    }

    direction = direction.next();
    while direction != from {
        maybe_set(grid, coordinates.step(direction), b'R');
        direction = direction.next();
    }
}

fn flood_fill(tracker: &mut [Vec<u8>], start: Coordinates) {
    let value = tracker.at(start).unwrap();
    for direction in Direction::cardinal() {
        let neighbor = start.step(direction);
        if maybe_set(tracker, neighbor, value) {
            flood_fill(tracker, neighbor.unwrap());
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let mut line = line.into_bytes();
            line.insert(0, b'.');
            line.push(b'.');
            line
        })
        .collect::<Vec<_>>();
    grid.insert(0, vec![b'.'; grid[0].len()]);
    grid.push(vec![b'.'; grid[0].len()]);

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

    for row in 0..tracker.len() {
        for column in 0..tracker[0].len() {
            let coordinates = Coordinates::new(row, column);
            if let Some(value) = tracker.as_slice().at(coordinates) {
                if value == b'L' || value == b'R' {
                    flood_fill(tracker.as_mut_slice(), coordinates);
                }
            }
        }
    }

    let outside = tracker[0][0];
    let inside = match outside {
        b'L' => b'R',
        b'R' => b'L',
        _ => unreachable!(),
    };
    let inside_count = tracker
        .iter()
        .flat_map(|line| line.iter())
        .filter(|b| **b == inside)
        .count();

    println!("{inside_count}");
}
