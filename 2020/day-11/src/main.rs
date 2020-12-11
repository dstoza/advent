#![deny(clippy::all, clippy::pedantic)]

use std::{
    convert::TryInto,
    env,
    fmt::{Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

struct Change {
    address: usize,
    cell: Cell,
}

#[derive(Clone)]
struct Layout {
    map: Vec<Cell>,
    column_count: i32,
    row_count: i32,
}

impl Layout {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            column_count: -1,
            row_count: 0,
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

    fn get_address(&self, row: i32, column: i32) -> usize {
        (row * self.column_count + column)
            .try_into()
            .expect("Failed to store address in usize")
    }

    fn get_cell(&self, row: i32, column: i32) -> Cell {
        self.map[self.get_address(row, column)]
    }

    fn has_adjacent_occupant(
        &self,
        mut row: i32,
        mut column: i32,
        delta_x: i32,
        delta_y: i32,
        line_of_sight: bool,
    ) -> bool {
        loop {
            row += delta_y;
            column += delta_x;

            if row < 0 || row >= self.row_count {
                return false;
            }
            if column < 0 || column >= self.column_count {
                return false;
            }

            match self.get_cell(row, column) {
                Cell::Floor => (),
                Cell::Empty => return false,
                Cell::Occupied => return true,
            }

            if !line_of_sight {
                return false;
            }
        }
    }

    fn count_adjacent_occupants(
        &self,
        row: i32,
        column: i32,
        expecting_zero: bool,
        line_of_sight: bool,
    ) -> i32 {
        let mut count = 0;
        for delta_y in -1..=1 {
            for delta_x in -1..=1 {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }

                if self.has_adjacent_occupant(row, column, delta_x, delta_y, line_of_sight) {
                    count += 1;
                    if expecting_zero {
                        return count;
                    }
                }
            }
        }

        count
    }

    fn collect_changes(&self, line_of_sight: bool) -> Vec<Change> {
        let mut changes = Vec::new();

        let abandonment_threshold = if line_of_sight { 5 } else { 4 };

        for row in 0..self.row_count {
            for column in 0..self.column_count {
                match self.get_cell(row, column) {
                    Cell::Floor => continue,
                    Cell::Empty => {
                        if self.count_adjacent_occupants(row, column, true, line_of_sight) == 0 {
                            changes.push(Change {
                                address: self.get_address(row, column),
                                cell: Cell::Occupied,
                            })
                        }
                    }
                    Cell::Occupied => {
                        if self.count_adjacent_occupants(row, column, false, line_of_sight)
                            >= abandonment_threshold
                        {
                            changes.push(Change {
                                address: self.get_address(row, column),
                                cell: Cell::Empty,
                            })
                        }
                    }
                }
            }
        }

        changes
    }

    fn apply_changes(&mut self, mut changes: Vec<Change>) {
        for change in changes.drain(..) {
            self.map[change.address] = change.cell;
        }
    }

    fn evolve(&mut self, line_of_sight: bool) -> bool {
        let changes = self.collect_changes(line_of_sight);
        if changes.is_empty() {
            return false;
        }

        self.apply_changes(changes);
        true
    }

    fn count_occupants(&self) -> i32 {
        self.map
            .iter()
            .map(|cell| match cell {
                Cell::Occupied => 1,
                _ => 0,
            })
            .sum()
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                write!(
                    f,
                    "{}",
                    match self.get_cell(row, column) {
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

    let mut layout = Layout::new();

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

    while layout.evolve(line_of_sight) {}
    println!("Occupied seats: {}", layout.count_occupants());
}
