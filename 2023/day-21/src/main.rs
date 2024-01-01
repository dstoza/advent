#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    fn neighbors(self) -> Vec<Self> {
        let mut neighbors = vec![
            Self::new(self.row, self.column + 1),
            Self::new(self.row + 1, self.column),
        ];
        if self.column > 0 {
            neighbors.push(Self::new(self.row, self.column - 1));
        }
        if self.row > 0 {
            neighbors.push(Self::new(self.row - 1, self.column));
        }
        neighbors
    }

    fn get_value(self, grid: &[Vec<u8>]) -> Option<u8> {
        if self.row >= grid.len() || self.column >= grid[0].len() {
            return None;
        }

        Some(grid[self.row][self.column])
    }
}

impl Coordinates {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

fn get_fill_counts(grid: &[Vec<u8>], start: Coordinates) -> Vec<usize> {
    let mut counts = Vec::new();

    let mut first_seen = HashMap::new();
    let mut open = vec![start];
    for step in 0.. {
        let mut next = Vec::new();
        for neighbor in open.into_iter().flat_map(Coordinates::neighbors) {
            let Some(value) = neighbor.get_value(grid) else {
                continue;
            };

            if value != b'#' && !first_seen.contains_key(&neighbor) {
                first_seen.insert(neighbor, step);
                next.push(neighbor);
            }
        }
        open = next;

        counts.push(
            first_seen
                .iter()
                .filter(|(_, first)| **first % 2 == step % 2)
                .count(),
        );

        if counts.len() > 2 && counts[counts.len() - 1] == counts[counts.len() - 3] {
            counts.resize(counts.len() - 1, 0);
            break;
        }
    }

    counts
}

#[allow(clippy::too_many_lines)]
fn simulate_grid(grid: &[Vec<u8>], start: Coordinates, tile_factor: usize) {
    let tile_height = grid.len() / tile_factor;
    let tile_width = grid[0].len() / tile_factor;

    let mut first_seen = HashMap::new();
    let mut grid_first_seen = HashMap::new();
    let mut grid_fill_counts: HashMap<Coordinates, Vec<usize>> = HashMap::new();

    let mut open = vec![start];
    for step in 0..120 {
        let mut next = Vec::new();
        for neighbor in open.into_iter().flat_map(Coordinates::neighbors) {
            let Some(value) = neighbor.get_value(grid) else {
                continue;
            };

            if value != b'#' && !first_seen.contains_key(&neighbor) {
                first_seen.insert(neighbor, step);
                next.push(neighbor);
            }
        }
        open = next;

        let mut current_grid_counts = HashMap::new();
        for (coordinates, first_seen) in &first_seen {
            let grid_cell = Coordinates::new(
                coordinates.row / tile_width,
                coordinates.column / tile_height,
            );
            grid_first_seen.entry(grid_cell).or_insert(*first_seen);
            current_grid_counts
                .entry(grid_cell)
                .and_modify(|count| *count += usize::from(step % 2 == first_seen % 2))
                .or_insert(usize::from(step % 2 == first_seen % 2));
        }

        for (grid_cell, count) in &current_grid_counts {
            grid_fill_counts
                .entry(*grid_cell)
                .and_modify(|counts| counts.push(*count))
                .or_default();
        }
    }

    println!("top left");
    for row in 0..tile_factor / 2 {
        for column in 0..tile_factor / 2 {
            let fill_counts = &grid_fill_counts
                .get(&Coordinates::new(row, column))
                .unwrap();
            let sliced = &fill_counts[0..6.min(fill_counts.len())];
            print!("{sliced:2?} ");
        }
        println!();
    }

    println!("top right");
    for row in 0..tile_factor / 2 {
        for column in tile_factor / 2 + 1..tile_factor {
            let fill_counts = &grid_fill_counts
                .get(&Coordinates::new(row, column))
                .unwrap();
            let sliced = &fill_counts[0..6.min(fill_counts.len())];
            print!("{sliced:2?} ");
        }
        println!();
    }

    println!("bottom left");
    for row in tile_factor / 2 + 1..tile_factor {
        for column in 0..tile_factor / 2 {
            let fill_counts = &grid_fill_counts
                .get(&Coordinates::new(row, column))
                .unwrap();
            let sliced = &fill_counts[0..6.min(fill_counts.len())];
            print!("{sliced:2?} ");
        }
        println!();
    }

    println!("bottom right");
    for row in tile_factor / 2 + 1..tile_factor {
        for column in tile_factor / 2 + 1..tile_factor {
            let fill_counts = &grid_fill_counts
                .get(&Coordinates::new(row, column))
                .unwrap();
            let sliced = &fill_counts[0..6.min(fill_counts.len())];
            print!("{sliced:2?} ");
        }
        println!();
    }

    println!("horizontal");
    for column in 0..tile_factor {
        let fill_counts = &grid_fill_counts
            .get(&Coordinates::new(tile_factor / 2, column))
            .unwrap();
        let sliced = &fill_counts[0..6.min(fill_counts.len())];
        println!("{sliced:2?} ");
    }

    println!("vertical");
    for row in 0..tile_factor {
        let fill_counts = &grid_fill_counts
            .get(&Coordinates::new(row, tile_factor / 2))
            .unwrap();
        let sliced = &fill_counts[0..6.min(fill_counts.len())];
        print!("{sliced:2?} ");

        println!();
    }

    for row in 0..tile_factor {
        for column in 0..tile_factor {
            let grid_cell = Coordinates::new(row, column);
            print!("{:3} ", grid_first_seen.get(&grid_cell).unwrap_or(&0));
        }
        println!();
    }
}

fn tile_grid(grid: &[Vec<u8>], factor: usize) -> Vec<Vec<u8>> {
    let mut tiled = grid
        .iter()
        .map(|line| {
            let mut expanded = Vec::new();
            for _ in 0..factor {
                expanded.extend_from_slice(line);
            }
            for value in &mut expanded {
                if *value == b'S' {
                    *value = b'.';
                }
            }
            expanded
        })
        .collect::<Vec<_>>();

    let row_count = tiled.len();

    for _ in 0..factor - 1 {
        tiled.extend_from_within(0..row_count);
    }

    tiled
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(column, value)| (row, column, *value))
        })
        .find(|(_, _, value)| *value == b'S')
        .map(|(row, column, _)| Coordinates::new(row, column))
        .unwrap();

    let mut first_seen = HashMap::new();
    let mut open = vec![start];
    for step in 1..=64 {
        let mut next = Vec::new();
        for neighbor in open.into_iter().flat_map(Coordinates::neighbors) {
            let Some(value) = neighbor.get_value(&grid) else {
                continue;
            };

            if value != b'#' && !first_seen.contains_key(&neighbor) {
                first_seen.insert(neighbor, step);
                next.push(neighbor);
            }
        }
        open = next;
    }

    let plots = first_seen
        .iter()
        .filter(|(_, first)| **first % 2 == 0)
        .count();
    println!("{plots}");

    let tile_factor = 9;

    let tiled = tile_grid(&grid, tile_factor);

    let start = Coordinates::new(
        start.row + tile_factor / 2 * grid[0].len(),
        start.column + tile_factor / 2 * grid.len(),
    );

    simulate_grid(&tiled, start, tile_factor);
}
