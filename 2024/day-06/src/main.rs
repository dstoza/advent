#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (position.0 - 1, position.1),
            Self::Right => (position.0, position.1 + 1),
            Self::Down => (position.0 + 1, position.1),
            Self::Left => (position.0, position.1 - 1),
        }
    }

    fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Default)]
struct PackedSortedVecs<T>
where
    T: Default,
{
    stride: usize,
    data: Vec<T>,
    lengths: Vec<usize>,
}

impl<T> PackedSortedVecs<T>
where
    T: Default + Copy + Ord,
{
    fn new() -> Self {
        Self {
            stride: 1,
            ..Self::default()
        }
    }

    fn get(&self, vec: usize) -> &[T] {
        &self.data[(vec * self.stride)..(vec * self.stride + self.lengths[vec])]
    }

    fn restride(&mut self, stride: usize) {
        let mut new_data = Vec::with_capacity(self.lengths.len() * stride);
        new_data.resize(self.lengths.len() * stride, T::default());

        for vec in 0..self.lengths.len() {
            let source = &self.data[(vec * self.stride)..((vec + 1) * self.stride)];
            let destination = &mut new_data[(vec * stride)..(vec * stride + self.stride)];
            destination.copy_from_slice(source);
        }

        self.data = new_data;
        self.stride = stride;
    }

    fn insert(&mut self, vec: usize, value: T) {
        if vec >= self.lengths.len() {
            self.lengths.resize(vec + 1, 0);
            self.data.resize((vec + 1) * self.stride, T::default());
        }

        if self.lengths[vec] == self.stride {
            self.restride(self.stride + 1);
        }

        self.data[vec * self.stride + self.lengths[vec]] = value;
        self.lengths[vec] += 1;
        self.data[vec * self.stride..(vec * self.stride + self.lengths[vec])].sort_unstable();
    }
}

impl<T> Display for PackedSortedVecs<T>
where
    T: Debug + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for vec in 0..self.lengths.len() {
            if vec != 0 {
                f.write_str("\n")?;
            }

            f.write_fmt(format_args!(
                "{:?}",
                &self.data[vec * self.stride..(vec * self.stride + self.lengths[vec])]
            ))?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct CycleDetector {
    start_row: usize,
    start_column: usize,
    by_row: PackedSortedVecs<usize>,
    by_column: PackedSortedVecs<usize>,
}

impl CycleDetector {
    fn new(grid: &[Vec<u8>]) -> Self {
        let mut start_row = 0;
        let mut start_column = 0;
        let mut by_row = PackedSortedVecs::new();
        let mut by_column = PackedSortedVecs::new();

        for (row, line) in grid.iter().enumerate() {
            for (column, cell) in line.iter().enumerate() {
                match *cell {
                    b'^' => {
                        start_row = row;
                        start_column = column;
                    }
                    b'#' => {
                        by_row.insert(row, column);
                        by_column.insert(column, row);
                    }
                    _ => (),
                }
            }
        }

        Self {
            start_row,
            start_column,
            by_row,
            by_column,
        }
    }

    fn insert_obstruction(&mut self, row: usize, column: usize) {
        self.by_row.insert(row, column);
        self.by_column.insert(column, row);
    }

    fn get_next(
        &self,
        row: usize,
        column: usize,
        direction: Direction,
    ) -> Option<((usize, usize), Direction)> {
        match direction {
            Direction::Up => {
                let data = self.by_column.get(column);
                let point = data.partition_point(|value| *value < row);
                if point == 0 {
                    None
                } else {
                    Some(((data[point - 1] + 1, column), direction.rotate()))
                }
            }
            Direction::Right => {
                let data = self.by_row.get(row);
                let point = data.partition_point(|value| *value < column);
                if point == data.len() {
                    None
                } else {
                    Some(((row, data[point] - 1), direction.rotate()))
                }
            }
            Direction::Down => {
                let data = self.by_column.get(column);
                let point = data.partition_point(|value| *value < row);
                if point == data.len() {
                    None
                } else {
                    Some(((data[point] - 1, column), direction.rotate()))
                }
            }
            Direction::Left => {
                let data = self.by_row.get(row);
                let point = data.partition_point(|value| *value < column);
                if point == 0 {
                    None
                } else {
                    Some(((row, data[point - 1] + 1), direction.rotate()))
                }
            }
        }
    }

    fn has_cycle(&self, obstruction_row: usize, obstruction_column: usize) -> bool {
        let mut obstructed = self.clone();
        obstructed.insert_obstruction(obstruction_row, obstruction_column);

        let mut row = self.start_row;
        let mut column = self.start_column;
        let mut direction = Direction::Up;
        let mut visited = HashSet::from([((self.start_row, self.start_column), Direction::Up)]);
        while let Some(next) = obstructed.get_next(row, column, direction) {
            if visited.contains(&next) {
                return true;
            }

            visited.insert(next);
            ((row, column), direction) = next;
        }

        false
    }
}

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

fn parse_padded_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for line in lines {
        let mut bytes = vec![b'*'];
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'*');

        if grid.is_empty() {
            grid.push(vec![b'*'; bytes.len()]);
        }

        grid.push(bytes);
    }

    grid.push(grid[0].clone());

    grid
}

fn get_start_position(grid: &[Vec<u8>]) -> (usize, usize) {
    let mut position = (0, 0);
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            if *cell == b'^' {
                position = (row_index, column_index);
                break;
            }
        }
    }

    position
}

fn get_next(
    grid: &[Vec<u8>],
    position: (usize, usize),
    direction: Direction,
    obstruction: Option<(usize, usize)>,
) -> u8 {
    let step = direction.step(position);
    if let Some(obstruction) = obstruction {
        if step == obstruction {
            return b'#';
        }
    }

    let (next_row, next_column) = step;
    grid[next_row][next_column]
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);
    let grid = parse_padded_grid(reader.lines().map(Result::unwrap));

    let start_position = get_start_position(&grid);
    let mut position = start_position;
    let mut visited = HashSet::new();
    let mut direction = Direction::Up;
    while grid[position.0][position.1] != b'*' {
        visited.insert(position);
        while get_next(&grid, position, direction, None) == b'#' {
            direction = direction.rotate();
        }

        position = direction.step(position);
    }

    let cycle_detector = CycleDetector::new(&grid);
    let mut count = 0;
    for (obstruction_row, obstruction_column) in &visited {
        if cycle_detector.has_cycle(*obstruction_row, *obstruction_column) {
            count += 1;
        }
    }

    println!("{}", visited.len());
    println!("{count}");
}
