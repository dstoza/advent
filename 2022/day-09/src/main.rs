#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn is_adjacent(self, other: Self) -> bool {
        (self.x - 1..=self.x + 1).contains(&other.x) && (self.y - 1..=self.y + 1).contains(&other.y)
    }

    fn move_towards(&mut self, other: Self) {
        if self.is_adjacent(other) {
            return;
        }

        self.x = other.x - (other.x - self.x) / 2;
        self.y = other.y - (other.y - self.y) / 2;
    }
}

fn simulate_rope(lines: impl Iterator<Item = String>, rope_length: usize) -> usize {
    let mut tail_visits = HashSet::new();

    let mut head_position = Position::new();
    let mut tail_positions = vec![Position::new(); rope_length];

    for line in lines {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let step_count: i32 = split.next().unwrap().parse().unwrap();

        for _ in 0..step_count {
            match direction {
                "L" => {
                    head_position.x -= 1;
                }
                "R" => {
                    head_position.x += 1;
                }
                "U" => {
                    head_position.y += 1;
                }
                "D" => {
                    head_position.y -= 1;
                }
                _ => unreachable!(),
            }

            tail_positions[0].move_towards(head_position);
            for t in 1..tail_positions.len() {
                let previous_position = tail_positions[t - 1];
                tail_positions[t].move_towards(previous_position);
            }
            tail_visits.insert(tail_positions[tail_positions.len() - 1]);
        }
    }

    tail_visits.len()
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let rope_length = std::env::args()
        .nth(2)
        .expect("Rope length not found")
        .parse()
        .unwrap();

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let tail_positions = simulate_rope(lines, rope_length);
    println!("{tail_positions} tail positions visited");
}
