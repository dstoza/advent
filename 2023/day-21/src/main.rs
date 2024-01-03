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

fn get_completed(counts: &[usize]) -> Option<Vec<usize>> {
    for (index, count) in counts.iter().enumerate() {
        if index < 2 {
            continue;
        }

        if *count == counts[index - 2] {
            return Some(counts[..index].to_vec());
        }
    }

    None
}

#[derive(Debug)]
struct Diagonal {
    start: usize,
    sequence: Vec<usize>,
    period: usize,
}

impl Diagonal {
    fn new(start: usize, sequence: Vec<usize>, period: usize) -> Self {
        Self {
            start,
            sequence,
            period,
        }
    }
}

#[derive(Debug)]
struct Straight {
    starts: Vec<usize>,
    sequences: Vec<Vec<usize>>,
    period: usize,
}

impl Straight {
    fn new(starts: Vec<usize>, sequences: Vec<Vec<usize>>, period: usize) -> Self {
        Self {
            starts,
            sequences,
            period,
        }
    }
}

#[allow(clippy::too_many_lines)]
fn simulate_grid(grid: &[Vec<u8>], start: Coordinates, tile_factor: usize) {
    let tile_height = grid.len() / tile_factor;
    let tile_width = grid[0].len() / tile_factor;
    assert_eq!(tile_width, tile_height);

    let mut top_left = None;
    let mut top_right = None;
    let mut bottom_left = None;
    let mut bottom_right = None;

    let mut straight_up = None;
    let mut straight_down = None;
    let mut straight_left = None;
    let mut straight_right = None;

    let mut first_seen = HashMap::new();
    let mut grid_first_seen = HashMap::new();
    let mut grid_fill_counts: HashMap<Coordinates, Vec<usize>> = HashMap::new();

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

        for (diagonal, coordinates) in [
            (
                &mut top_left,
                Coordinates::new(tile_factor / 2 - 1, tile_factor / 2 - 1),
            ),
            (
                &mut top_right,
                Coordinates::new(tile_factor / 2 - 1, tile_factor / 2 + 1),
            ),
            (
                &mut bottom_left,
                Coordinates::new(tile_factor / 2 + 1, tile_factor / 2 - 1),
            ),
            (
                &mut bottom_right,
                Coordinates::new(tile_factor / 2 + 1, tile_factor / 2 + 1),
            ),
        ] {
            if diagonal.is_some() {
                continue;
            }

            if let Some(sequence) =
                get_completed(grid_fill_counts.get(&coordinates).unwrap_or(&Vec::new()))
            {
                *diagonal = Some(Diagonal::new(
                    *grid_first_seen.get(&coordinates).unwrap(),
                    sequence,
                    tile_width,
                ));
            }
        }

        for (straight, coordinates) in [
            (
                &mut straight_up,
                (0..tile_factor / 2)
                    .rev()
                    .map(|row| Coordinates::new(row, tile_factor / 2))
                    .collect::<Vec<_>>(),
            ),
            (
                &mut straight_down,
                (tile_factor / 2 + 1..tile_factor)
                    .map(|row| Coordinates::new(row, tile_factor / 2))
                    .collect(),
            ),
            (
                &mut straight_left,
                (0..tile_factor / 2)
                    .rev()
                    .map(|column| Coordinates::new(tile_factor / 2, column))
                    .collect(),
            ),
            (
                &mut straight_right,
                (tile_factor / 2 + 1..tile_factor)
                    .map(|column| Coordinates::new(tile_factor / 2, column))
                    .collect(),
            ),
        ] {
            if straight.is_some() {
                continue;
            }

            let candidates = coordinates
                .iter()
                .filter_map(|tile| {
                    let Some(fill_counts) = grid_fill_counts
                        .get(tile)
                        .and_then(|counts| get_completed(counts))
                    else {
                        return None;
                    };

                    Some((grid_first_seen.get(tile).unwrap(), fill_counts))
                })
                .collect::<Vec<_>>();

            for (index, (start, sequence)) in candidates.iter().enumerate() {
                if index < 1 {
                    continue;
                }

                if *sequence == candidates[index - 1].1 {
                    *straight = Some(Straight::new(
                        candidates
                            .iter()
                            .take(index)
                            .map(|(start, _)| **start)
                            .collect(),
                        candidates
                            .iter()
                            .take(index)
                            .map(|(_, sequence)| sequence)
                            .cloned()
                            .collect(),
                        **start - candidates[index - 1].0,
                    ));
                }
            }
        }

        if top_left.is_some()
            && top_right.is_some()
            && bottom_left.is_some()
            && bottom_right.is_some()
            && straight_up.is_some()
            && straight_down.is_some()
            && straight_left.is_some()
            && straight_right.is_some()
        {
            println!("completed at {step}");
            println!("top_left {top_left:?}");
            println!("top_right {top_right:?}");
            println!("bottom_left {bottom_left:?}");
            println!("bottom_right {bottom_right:?}");
            println!("straight_up {straight_up:?}");
            println!("straight_down {straight_down:?}");
            println!("straight_left {straight_left:?}");
            println!("straight_right {straight_right:?}");
            break;
        }
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
