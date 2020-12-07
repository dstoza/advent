#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct PathFollower {
    right: usize,
    down: usize,
    column: usize,
    row: usize,
    tree_count: usize,
}

impl PathFollower {
    fn new(right: usize, down: usize) -> Self {
        Self {
            right,
            down,
            column: 0,
            row: 0,
            tree_count: 0,
        }
    }

    fn add_line(&mut self, line: &[u8]) {
        if self.row % self.down != 0 {
            self.row += 1;
            return;
        }

        if line[self.column % line.len()] == b'#' {
            self.tree_count += 1;
        }

        self.column += self.right;
        self.row += 1;
    }

    fn get_tree_count(&self) -> usize {
        self.tree_count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut followers = Vec::new();
    followers.push(PathFollower::new(1, 1));
    followers.push(PathFollower::new(3, 1));
    followers.push(PathFollower::new(5, 1));
    followers.push(PathFollower::new(7, 1));
    followers.push(PathFollower::new(1, 2));

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim().as_bytes();
        for follower in &mut followers {
            follower.add_line(trimmed);
        }

        line.clear();
    }

    println!(
        "Follower product: {}",
        followers
            .drain(..)
            .map(|follower| follower.get_tree_count())
            .product::<usize>()
    );
}
