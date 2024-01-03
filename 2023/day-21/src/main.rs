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

fn extrapolate(counts: &[usize], step: usize) -> usize {
    if step < counts.len() {
        counts[step]
    } else {
        let last_parity = (counts.len() - 1) % 2;
        let step_parity = step % 2;
        if step_parity == last_parity {
            counts[counts.len() - 1]
        } else {
            counts[counts.len() - 2]
        }
    }
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

    fn count(&self, step: usize) -> usize {
        if step < self.start {
            return 0;
        }

        let mut count = 0;

        let mut step = step - self.start;
        for starts in 1.. {
            count += extrapolate(&self.sequence, step) * starts;

            if step < self.period {
                break;
            }

            step -= self.period;
        }

        count
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

    fn count(&self, step: usize) -> usize {
        if step < self.starts[0] {
            return 0;
        }

        let mut count = 0;

        for (start, sequence) in self.starts.iter().zip(self.sequences.iter()) {
            if step < *start {
                break;
            }

            count += extrapolate(sequence, step - *start);
        }

        let last_start = *self.starts.last().unwrap();
        if step < last_start + self.period {
            return count;
        }

        let last_sequence = self.sequences.last().unwrap();
        let mut step = step - last_start - self.period;
        loop {
            count += extrapolate(last_sequence, step);
            if step < self.period {
                break;
            }
            step -= self.period;
        }

        count
    }
}

#[allow(clippy::too_many_lines)]
fn analyze_grid(
    grid: &[Vec<u8>],
    start: Coordinates,
    tile_factor: usize,
) -> (Vec<Straight>, Vec<Diagonal>) {
    let tile_height = grid.len() / tile_factor;
    let tile_width = grid[0].len() / tile_factor;
    assert_eq!(tile_width, tile_height);

    let mut straight_up = None;
    let mut straight_down = None;
    let mut straight_left = None;
    let mut straight_right = None;

    let mut top_left = None;
    let mut top_right = None;
    let mut bottom_left = None;
    let mut bottom_right = None;

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
                .or_insert_with(|| vec![*count]);
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

        if top_left.is_some()
            && top_right.is_some()
            && bottom_left.is_some()
            && bottom_right.is_some()
            && straight_up.is_some()
            && straight_down.is_some()
            && straight_left.is_some()
            && straight_right.is_some()
        {
            let straights = vec![
                straight_up.unwrap(),
                straight_down.unwrap(),
                straight_left.unwrap(),
                straight_right.unwrap(),
            ];
            let diagonals = vec![
                top_left.unwrap(),
                top_right.unwrap(),
                bottom_left.unwrap(),
                bottom_right.unwrap(),
            ];
            return (straights, diagonals);
        }
    }

    unreachable!()
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

#[allow(clippy::too_many_lines)]
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

    let center_counts = get_fill_counts(&grid, start);

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

    let tile_factor = 13;

    let tiled = tile_grid(&grid, tile_factor);

    let tiled_start = Coordinates::new(
        start.row + tile_factor / 2 * grid[0].len(),
        start.column + tile_factor / 2 * grid.len(),
    );

    let (straights, diagonals) = analyze_grid(&tiled, tiled_start, tile_factor);

    let step_count = 26_501_365;
    let simulated = diagonals
        .iter()
        .map(|diagonal| diagonal.count(step_count - 1))
        .chain(
            straights
                .iter()
                .map(|straight| straight.count(step_count - 1)),
        )
        .sum::<usize>()
        + extrapolate(&center_counts, step_count - 1);
    println!("{simulated}");
}
