#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

enum Turn {
    Right = 1,
    Left = 3,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn turn(self, turn: Turn) -> Self {
        match (self as u8 + turn as u8) % 4 {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }
}

struct WrapCache {
    cache: HashMap<(Position, Facing), Option<Position>>,
}

impl WrapCache {
    fn next(&mut self, position: Position, facing: Facing) -> Option<Position> {
        if let Some(hit) = self.cache.get(&(position, facing)) {
            return *hit;
        }

        // TODO: Step backwards until we find a blank space, then forwards one, check if it's a wall, then store/return the result

        None
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn step(self, facing: Facing) -> Self {
        match facing {
            Facing::Right => Self {
                row: self.row,
                column: self.column + 1,
            },
            Facing::Down => Self {
                row: self.row + 1,
                column: self.column,
            },
            Facing::Left => Self {
                row: self.row,
                column: self.column - 1,
            },
            Facing::Up => Self {
                row: self.row - 1,
                column: self.column,
            },
        }
    }

    fn next(self, board: &[Vec<u8>], wrap_cache: &mut WrapCache, facing: Facing) -> Option<Self> {
        let next_coordinates = self.step(facing);

        match board[next_coordinates.row][next_coordinates.column] {
            b'#' => None,
            b'.' => Some(next_coordinates),
            b' ' => wrap_cache.next(self, facing),
            _ => unimplemented!(),
        }
    }
}

fn parse_board(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut board = vec![Vec::new()];
    let mut max_length = 0;
    for line in lines {
        if line.is_empty() {
            break;
        }

        board.push(
            " ".as_bytes()
                .iter()
                .chain(line.as_bytes())
                .chain(" ".as_bytes().iter())
                .copied()
                .collect(),
        );
        max_length = max_length.max(board.last().unwrap().len());
    }

    for row in &mut board {
        row.resize(max_length, b' ');
    }

    board.push(vec![b' '; max_length]);

    board
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);
    let board = parse_board(lines);
    for line in board {
        println!("{:?}", line);
    }
}
