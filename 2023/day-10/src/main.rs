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
    East,
    South,
    West,
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
            Direction::East => Some(Coordinates::new(self.row, self.column + 1)),
            Direction::South => Some(Coordinates::new(self.row + 1, self.column)),
            Direction::West => {
                if self.column > 0 {
                    Some(Coordinates::new(self.row, self.column - 1))
                } else {
                    None
                }
            }
        }
    }
}

trait Map {
    fn at(&self, coordinates: Coordinates) -> Option<u8>;
}

impl Map for &[Vec<u8>] {
    fn at(&self, coordinates: Coordinates) -> Option<u8> {
        if coordinates.row < self.len() && coordinates.column < self[coordinates.row].len() {
            Some(self[coordinates.row][coordinates.column])
        } else {
            None
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
    type Item = Coordinates;

    fn next(&mut self) -> Option<Coordinates> {
        let next = self.next;
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
        Some(next)
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
    for (a, b) in iterator {
        steps += 1;
        if !visited.insert(a) || !visited.insert(b) {
            break;
        }
    }

    println!("{steps}");
}
