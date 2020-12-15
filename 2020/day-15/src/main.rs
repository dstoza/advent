#![deny(clippy::all, clippy::pedantic)]

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct MemoryGame {
    current_turn: i32,
    previous_number: i32,
    last_seen: HashMap<i32, i32>,
}

impl MemoryGame {
    fn new(initial_numbers: &str) -> Self {
        let mut current_turn = 1;
        let mut previous_number = 0;
        let mut last_seen = HashMap::new();

        for number in initial_numbers.split(',').map(|number| {
            number
                .parse::<i32>()
                .expect("Failed to parse number as i32")
        }) {
            last_seen.insert(number, current_turn);
            previous_number = number;
            current_turn += 1;
        }

        Self {
            current_turn,
            previous_number,
            last_seen,
        }
    }

    fn nth(&mut self, n: i32) -> i32 {
        while self.current_turn <= n {
            let current_number = match self.last_seen.get(&self.previous_number) {
                Some(last_seen) => self.current_turn - 1 - *last_seen,
                None => 0,
            };

            self.last_seen
                .insert(self.previous_number, self.current_turn - 1);

            self.previous_number = current_number;
            self.current_turn += 1;
        }

        self.previous_number
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read line"));

    let mut game = MemoryGame::new(line.trim());
    println!("nth number: {}", game.nth(30_000_000));
}
