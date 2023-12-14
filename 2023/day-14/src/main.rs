#![warn(clippy::pedantic)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug, Hash)]
struct Grid {
    rows: Vec<u8>,
    columns: Vec<u8>,
    width: usize,
    height: usize,
}

fn compress_left(line: &mut [u8]) {
    for segment in line.split_mut(|b| *b == b'#') {
        segment.sort_unstable_by(|l, r| r.cmp(l));
    }
}

fn compress_right(line: &mut [u8]) {
    for segment in line.split_mut(|b| *b == b'#') {
        segment.sort_unstable();
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

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    let mut width = None;
    for line in reader.lines().map(std::result::Result::unwrap) {
        let line = line.as_bytes();
        width = width.or(Some(line.len()));
        rows.extend_from_slice(line);
    }
    let mut just_north = Grid::new(rows, width.unwrap());
    let mut cycled = just_north.clone();
    just_north.slide_north();
    println!("{}", just_north.load());

    let mut last_seen = HashMap::new();
    for cycle in 1..1_000_000_000 {
        cycled.run_cycle();
        let mut hasher = DefaultHasher::new();
        cycled.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(last) = last_seen.get(&hash) {
            let cycle_length = cycle - last;
            if (1_000_000_000 - cycle) % cycle_length == 0 {
                println!("{}", cycled.load());
                break;
            }
        }

        last_seen.insert(hash, cycle);
    }
}
