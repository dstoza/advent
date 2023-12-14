#![feature(test)]
#![warn(clippy::pedantic)]

extern crate test;

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
struct Grid {
    rows: Vec<u8>,
    columns: Vec<u8>,
    width: usize,
    height: usize,
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rows.hash(state);
    }
}

fn compress_left(line: &mut [u8]) {
    for segment in line.split_mut(|b| *b == b'#') {
        if segment.len() < 2 {
            continue;
        }

        let mut left = 0;
        let mut right = segment.len() - 1;
        while left != right {
            match (segment[left], segment[right]) {
                (b'O', _) => left += 1,
                (_, b'O') => {
                    segment.swap(left, right);
                    right -= 1;
                }
                _ => right -= 1,
            }
        }
    }
}

fn compress_right(line: &mut [u8]) {
    for segment in line.split_mut(|b| *b == b'#') {
        if segment.len() < 2 {
            continue;
        }

        let mut left = 0;
        let mut right = segment.len() - 1;
        while left != right {
            match (segment[left], segment[right]) {
                (_, b'O') => right -= 1,
                (b'O', _) => {
                    segment.swap(left, right);
                    left += 1;
                }
                _ => left += 1,
            }
        }
    }
}

impl Grid {
    fn new(rows: Vec<u8>, width: usize) -> Self {
        let height = rows.len() / width;
        let columns = vec![b'.'; rows.len()];
        let mut grid = Self {
            rows,
            columns,
            width,
            height,
        };
        grid.rows_to_columns();
        grid
    }

    fn rows_to_columns(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                self.columns[row + column * self.height] = self.rows[row * self.width + column];
            }
        }
    }

    fn columns_to_rows(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                self.rows[row * self.width + column] = self.columns[row + column * self.height];
            }
        }
    }

    fn load(&self) -> usize {
        self.rows
            .chunks(self.width)
            .enumerate()
            .map(|(index, row)| {
                let distance = self.height - index;
                distance * bytecount::count(row, b'O')
            })
            .sum()
    }

    fn slide_north(&mut self) {
        for column in self.columns.chunks_mut(self.height) {
            compress_left(column);
        }
        self.columns_to_rows();
    }

    fn slide_south(&mut self) {
        for column in self.columns.chunks_mut(self.height) {
            compress_right(column);
        }
        self.columns_to_rows();
    }

    fn slide_west(&mut self) {
        for row in self.rows.chunks_mut(self.width) {
            compress_left(row);
        }
        self.rows_to_columns();
    }

    fn slide_east(&mut self) {
        for row in self.rows.chunks_mut(self.width) {
            compress_right(row);
        }
        self.rows_to_columns();
    }

    fn run_cycle(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }
}

fn load_grid() -> Grid {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut rows = Vec::new();
    let mut width = None;
    for line in reader.lines().map(std::result::Result::unwrap) {
        let line = line.as_bytes();
        width = width.or(Some(line.len()));
        rows.extend_from_slice(line);
    }
    Grid::new(rows, width.unwrap())
}

fn get_billion_load(mut grid: Grid) -> usize {
    let mut last_seen = HashMap::new();
    for cycle in 1..1_000_000_000 {
        grid.run_cycle();

        let hash = {
            let mut hasher = DefaultHasher::new();
            grid.hash(&mut hasher);
            hasher.finish()
        };

        if let Some(last) = last_seen.get(&hash) {
            let cycle_length = cycle - last;
            if (1_000_000_000 - cycle) % cycle_length == 0 {
                return grid.load();
            }
        }

        last_seen.insert(hash, cycle);
    }

    unreachable!()
}

fn main() {
    let mut just_north = load_grid();
    let cycled = just_north.clone();
    just_north.slide_north();
    println!("{}", just_north.load());

    println!("{}", get_billion_load(cycled));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn billion_bench(bencher: &mut test::Bencher) {
        let grid = load_grid();
        bencher.iter(|| assert!(get_billion_load(grid.clone()) == 96105));
    }
}
