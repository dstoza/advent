#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn neighbors(self, width: usize, height: usize) -> Vec<(Direction, Self)> {
        let mut neighbors = Vec::new();

        if self.row > 0 {
            neighbors.push((Direction::Up, Position::new(self.row - 1, self.column)));
        }
        if self.row < height - 1 {
            neighbors.push((Direction::Down, Position::new(self.row + 1, self.column)));
        }
        if self.column > 0 {
            neighbors.push((Direction::Left, Position::new(self.row, self.column - 1)));
        }
        if self.column < width - 1 {
            neighbors.push((Direction::Right, Position::new(self.row, self.column + 1)));
        }

        neighbors
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn paths(keypad: &[Vec<u8>], start_row: usize, start_column: usize) -> HashMap<u8, Vec<Direction>> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::from([(Position::new(start_row, start_column), Vec::new())]);
    while let Some((position, path)) = queue.pop_front() {
        let value = keypad[position.row][position.column];
        

        for (direction, neighbor) in position.neighbors(keypad[0].len(), keypad.len()) {
            let mut path_to_neighbor = path.clone();
            path_to_neighbor.push(direction);
            queue.push_back((neighbor, path_to_neighbor));
        }
    }

    HashMap::new()
}

fn all_paths(keypad: &[Vec<u8>]) -> HashMap<(u8, u8), Vec<Direction>> {
    HashMap::new()
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    println!("running part {}", args.part);

    for line in reader.lines().map(Result::unwrap) {
        println!("{line}");
    }
}
