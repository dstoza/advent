#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct MemoryGame {
    current_turn: u32,
    previous_number: u32,
    last_seen: Vec<u32>,
}

impl MemoryGame {
    fn new(initial_numbers: &str) -> Self {
        let mut current_turn = 1;
        let mut previous_number = 0;
        let mut last_seen = vec![0; 30_000_000];

        for number in initial_numbers.split(',').map(|number| {
            number
                .parse::<u32>()
                .expect("Failed to parse number as i32")
        }) {
            last_seen[number as usize] = current_turn;
            previous_number = number;
            current_turn += 1;
        }

        Self {
            current_turn,
            previous_number,
            last_seen,
        }
    }

    fn nth(&mut self, n: u32) -> u32 {
        while self.current_turn <= n {
            let current_number = if self.last_seen[self.previous_number as usize] > 0 {
                self.current_turn - 1 - self.last_seen[self.previous_number as usize]
            } else {
                0
            };

            self.last_seen[self.previous_number as usize] = self.current_turn - 1;

            self.previous_number = current_number;
            self.current_turn += 1;
        }

        self.previous_number
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read line"));

    let n: u32 = args[2].parse().expect("Failed to parse n as u32");

    let mut game = MemoryGame::new(line.trim());
    println!("nth number: {}", game.nth(n));
}
