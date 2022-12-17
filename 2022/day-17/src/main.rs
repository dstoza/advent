#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Rock<'a> {
    position: Position,
    shape: &'a [Vec<Position>],
}

impl<'a> Rock<'a> {
    fn move_left(&mut self, chamber: &Chamber) {}

    fn move_right(&mut self, chamber: &Chamber) {}

    fn move_down(&mut self, chamber: &Chamber) -> bool {
        true
    }
}

struct Chamber {
    columns: Vec<Vec<bool>>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            columns: vec![Vec::new(); 7],
        }
    }

    fn place(rock: Rock) {}
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let shapes = vec![
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(3, 0),
        ],
        vec![
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(1, 2),
        ],
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ],
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(0, 3),
        ],
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
    ];

    let chamber = Chamber::new();
    
}
