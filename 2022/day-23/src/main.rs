#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::RangeInclusive,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn new(row: i32, column: i32) -> Self {
        Self { row, column }
    }

    fn get_all_neighbors(self) -> [Self; 8] {
        [
            Self::new(self.row - 1, self.column - 1),
            Self::new(self.row - 1, self.column),
            Self::new(self.row - 1, self.column + 1),
            Self::new(self.row, self.column - 1),
            Self::new(self.row, self.column + 1),
            Self::new(self.row + 1, self.column - 1),
            Self::new(self.row + 1, self.column),
            Self::new(self.row + 1, self.column + 1),
        ]
    }

    fn get_neighbors_in_direction(self, direction: Direction) -> [Self; 3] {
        match direction {
            Direction::North => [
                Self::new(self.row - 1, self.column - 1),
                Self::new(self.row - 1, self.column),
                Self::new(self.row - 1, self.column + 1),
            ],
            Direction::South => [
                Self::new(self.row + 1, self.column - 1),
                Self::new(self.row + 1, self.column),
                Self::new(self.row + 1, self.column + 1),
            ],
            Direction::West => [
                Self::new(self.row - 1, self.column - 1),
                Self::new(self.row, self.column - 1),
                Self::new(self.row + 1, self.column - 1),
            ],
            Direction::East => [
                Self::new(self.row - 1, self.column + 1),
                Self::new(self.row, self.column + 1),
                Self::new(self.row + 1, self.column + 1),
            ],
        }
    }

    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.row - 1, self.column),
            Direction::South => Self::new(self.row + 1, self.column),
            Direction::West => Self::new(self.row, self.column - 1),
            Direction::East => Self::new(self.row, self.column + 1),
        }
    }
}

fn parse_elves(lines: impl Iterator<Item = String>) -> HashSet<Position> {
    let mut elves = HashSet::new();

    for (row, line) in lines.enumerate() {
        for (column, element) in line.chars().enumerate() {
            match element {
                '#' => elves.insert(Position::new(
                    i32::try_from(row).unwrap(),
                    i32::try_from(column).unwrap(),
                )),
                '.' => continue,
                _ => unimplemented!(),
            };
        }
    }

    elves
}

fn get_bounds(elves: &HashSet<Position>) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut min_row = i32::MAX;
    let mut max_row = i32::MIN;
    let mut min_column = i32::MAX;
    let mut max_column = i32::MIN;
    for elf in elves {
        min_row = min_row.min(elf.row);
        max_row = max_row.max(elf.row);
        min_column = min_column.min(elf.column);
        max_column = max_column.max(elf.column);
    }
    (min_row..=max_row, min_column..=max_column)
}

// For each destination position, a vector of source elves that would like to move to that position
type Proposals = HashMap<Position, Option<Position>>;

fn get_proposals(elves: &HashSet<Position>, direction_order: &[Direction]) -> Proposals {
    let mut proposals = Proposals::new();

    for elf in elves {
        if elf
            .get_all_neighbors()
            .iter()
            .all(|neighbor| !elves.contains(neighbor))
        {
            continue;
        }

        for direction in direction_order {
            if elf
                .get_neighbors_in_direction(*direction)
                .iter()
                .all(|neighbor| !elves.contains(neighbor))
            {
                match proposals.entry(elf.step(*direction)) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.insert(None);
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(Some(*elf));
                    }
                };
                break;
            }
        }
    }

    proposals
}

fn resolve_proposals(elves: &mut HashSet<Position>, proposals: Proposals) {
    for (destination, source) in proposals {
        if source.is_none() {
            continue;
        }

        elves.remove(&source.unwrap());
        elves.insert(destination);
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);
    let mut elves = parse_elves(lines);

    let mut direction_order = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        let proposals = get_proposals(&elves, &direction_order);
        resolve_proposals(&mut elves, proposals);
        direction_order.rotate_left(1);
    }

    let (row_bounds, column_bounds) = get_bounds(&elves);
    let empty_tiles = (row_bounds.end() - row_bounds.start() + 1)
        * (column_bounds.end() - column_bounds.start() + 1)
        - i32::try_from(elves.len()).unwrap();

    println!("Empty tiles: {empty_tiles}");

    let mut iterations = 11;
    loop {
        let proposals = get_proposals(&elves, &direction_order);
        if proposals.is_empty() {
            break;
        }

        resolve_proposals(&mut elves, proposals);
        direction_order.rotate_left(1);
        iterations += 1;
    }

    println!("Iterations: {iterations}");
}
