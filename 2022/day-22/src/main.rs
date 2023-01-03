#![feature(iter_intersperse)]
#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug)]
enum Turn {
    Right = 1,
    Left = 3,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Facing {
    East,
    South,
    West,
    North,
}

impl Facing {
    fn turn(self, turn: Turn) -> Self {
        match (self as u8 + turn as u8) % 4 {
            0 => Self::East,
            1 => Self::South,
            2 => Self::West,
            3 => Self::North,
            _ => unreachable!(),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
        }
    }
}

struct WrapCache {
    cache: HashMap<(Position, Facing), Option<Position>>,
}

impl WrapCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn next(&mut self, board: &[Vec<u8>], position: Position, facing: Facing) -> Option<Position> {
        if let Some(hit) = self.cache.get(&(position, facing)) {
            return *hit;
        }

        let mut search = position.step(facing.opposite());
        while board[search.row][search.column] != b' ' {
            search = search.step(facing.opposite());
        }
        search = search.step(facing);

        let result = match board[search.row][search.column] {
            b'#' => None,
            b'.' => Some(search),
            _ => unimplemented!(),
        };

        self.cache.insert((position, facing), result);
        result
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn step(self, facing: Facing) -> Self {
        match facing {
            Facing::East => Self {
                row: self.row,
                column: self.column + 1,
            },
            Facing::South => Self {
                row: self.row + 1,
                column: self.column,
            },
            Facing::West => Self {
                row: self.row,
                column: self.column - 1,
            },
            Facing::North => Self {
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
            b' ' => wrap_cache.next(board, self, facing),
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

#[derive(Debug)]
enum Command {
    Step(usize),
    Turn(Turn),
}

fn parse_commands(line: String) -> Vec<Command> {
    line.split('R')
        .intersperse("R")
        .flat_map(|s| s.split('L').intersperse("L"))
        .map(|s| match s {
            "R" => Command::Turn(Turn::Right),
            "L" => Command::Turn(Turn::Left),
            _ => Command::Step(s.parse().unwrap()),
        })
        .collect()
}

fn run_commands(commands: &[Command], board: &[Vec<u8>]) {
    let mut wrap_cache = WrapCache::new();

    let column = board[1].iter().position(|b| *b == b'.').unwrap();
    let mut position = Position { row: 1, column };
    let mut facing = Facing::East;

    for command in commands {
        match command {
            Command::Turn(turn) => facing = facing.turn(*turn),
            Command::Step(steps) => {
                for _ in 0..*steps {
                    let next_position = position.next(board, &mut wrap_cache, facing);
                    if let Some(next_position) = next_position {
                        position = next_position;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let password = 1000 * position.row + 4 * position.column + facing as usize;
    println!("Password: {password}");
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let board = parse_board(&mut lines);
    let commands = parse_commands(lines.next().unwrap());

    run_commands(&commands, &board);
}
