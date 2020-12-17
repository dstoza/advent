#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{
    convert::TryInto,
    env,
    fmt::{Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
};

extern crate test;

#[derive(Clone, Copy)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone)]
struct Layout {
    line_of_sight: bool,
    map: Vec<Cell>,
    column_count: i32,
    row_count: i32,
    adjacent_indices: Vec<u16>,
    updated_indices: Vec<u16>,
    occupied_seats: Vec<bool>,
}

impl Layout {
    fn new(line_of_sight: bool) -> Self {
        Self {
            line_of_sight,
            map: Vec::new(),
            column_count: -1,
            row_count: 0,
            adjacent_indices: Vec::new(),
            updated_indices: Vec::new(),
            occupied_seats: Vec::new(),
        }
    }

    fn add_line(&mut self, line: &str) {
        for byte in line.as_bytes() {
            self.map.push(match byte {
                b'.' => Cell::Floor,
                b'L' => Cell::Empty,
                b'#' => Cell::Occupied,
                _ => panic!("Unexpected byte [{}]", byte),
            })
        }

        let incoming_column_count: i32 = line
            .len()
            .try_into()
            .expect("Couldn't store column count in i32");
        if self.column_count < 0 {
            self.column_count = incoming_column_count;
        } else if incoming_column_count != self.column_count {
            panic!(
                "Incoming column count {} different from stored column count {}",
                incoming_column_count, self.column_count
            );
        }

        self.row_count += 1;
    }

    fn get_index(&self, row: i32, column: i32) -> u16 {
        (row * self.column_count + column)
            .try_into()
            .expect("Failed to store address in u16")
    }

    fn get_adjacent_seat_index(
        &self,
        mut row: i32,
        mut column: i32,
        delta_x: i32,
        delta_y: i32,
    ) -> Option<u16> {
        loop {
            row += delta_y;
            column += delta_x;

            if row < 0 || row >= self.row_count {
                return None;
            }
            if column < 0 || column >= self.column_count {
                return None;
            }

            let index = self.get_index(row, column);
            match self
                .map
                .get(index as usize)
                .unwrap_or_else(|| panic!("Index {} not found in map", index))
            {
                Cell::Floor => (),
                Cell::Empty | Cell::Occupied => return Some(index),
            }

            if !self.line_of_sight {
                return None;
            }
        }
    }

    fn get_adjacent_indices(&self, row: i32, column: i32) -> Vec<u16> {
        let mut indices = Vec::new();

        for delta_y in -1..=1 {
            for delta_x in -1..=1 {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }

                if let Some(index) = self.get_adjacent_seat_index(row, column, delta_x, delta_y) {
                    indices.push(index);
                }
            }
        }

        indices
    }

    fn finalize(&mut self) {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let index = self.get_index(row, column);
                if let Cell::Floor = self.map[index as usize] {
                    self.adjacent_indices.append(&mut vec![u16::max_value(); 8]);
                    continue;
                }

                let mut adjacent_indices = self.get_adjacent_indices(row, column);
                adjacent_indices.resize(8, u16::max_value());
                self.adjacent_indices.append(&mut adjacent_indices);
                self.updated_indices.push(index);
            }
        }
        self.occupied_seats
            .resize(self.adjacent_indices.len() / 8, false);
    }

    fn count_adjacent_occupants(&self, index: u16) -> i32 {
        let mut count = 0;
        for adjacent_index in
            &self.adjacent_indices[((index as usize) * 8)..((index as usize) * 8 + 8)]
        {
            if *adjacent_index == u16::max_value() {
                break;
            }

            if self.occupied_seats[*adjacent_index as usize] {
                count += 1;
            }
        }
        count
    }

    fn collect_changes(&self) -> Vec<u16> {
        let mut changes = Vec::new();

        let abandonment_threshold = if self.line_of_sight { 5 } else { 4 };

        for index in &self.updated_indices {
            if self.occupied_seats[*index as usize] {
                if self.count_adjacent_occupants(*index) >= abandonment_threshold {
                    changes.push(*index);
                }
            } else if self.count_adjacent_occupants(*index) == 0 {
                changes.push(*index);
            }
        }

        changes
    }

    fn apply_changes(&mut self, changes: Vec<u16>) {
        for change in &changes {
            self.occupied_seats[*change as usize] ^= true;
        }
        self.updated_indices = changes;
    }

    fn evolve(&mut self) -> bool {
        let changes = self.collect_changes();
        if changes.is_empty() {
            return false;
        }

        self.apply_changes(changes);
        true
    }

    fn count_occupants(&self) -> i32 {
        self.occupied_seats
            .iter()
            .map(|occupied| if *occupied { 1 } else { 0 })
            .sum()
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let index = self.get_index(row, column);
                write!(
                    f,
                    "{}",
                    match self
                        .map
                        .get(index as usize)
                        .unwrap_or_else(|| panic!("Index {} not found in map", index))
                    {
                        Cell::Floor => '.',
                        Cell::Empty => 'L',
                        Cell::Occupied => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        return;
    }

    let line_of_sight = args.len() == 3 && args[2] == "los";

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut layout = Layout::new(line_of_sight);

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        layout.add_line(line.trim());

        line.clear();
    }

    layout.finalize();

    while layout.evolve() {}
    println!("Occupied seats: {}", layout.count_occupants());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_layout(line_of_sight: bool) -> Layout {
        let file = File::open("input.txt").expect("Failed to open input.txt");
        let mut reader = BufReader::new(file);

        let mut layout = Layout::new(line_of_sight);

        let mut line = String::new();
        loop {
            let bytes = reader
                .read_line(&mut line)
                .unwrap_or_else(|_| panic!("Failed to read line"));
            if bytes == 0 {
                break;
            }
            layout.add_line(line.trim());
            line.clear();
        }

        layout.finalize();

        layout
    }

    #[bench]
    fn bench_adjacent(bencher: &mut Bencher) {
        let layout = get_layout(false);
        bencher.iter(|| {
            let mut cloned = layout.clone();
            while cloned.evolve() {}
            assert_eq!(cloned.count_occupants(), 2361);
        });
    }

    #[bench]
    fn bench_line_of_sight(bencher: &mut Bencher) {
        let layout = get_layout(true);
        bencher.iter(|| {
            let mut cloned = layout.clone();
            while cloned.evolve() {}
            assert_eq!(cloned.count_occupants(), 2119);
        });
    }
}
