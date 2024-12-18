#![warn(clippy::pedantic)]

use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Maximum grid coordinate
    #[arg(short, long)]
    max: u8,

    /// Time to run
    #[arg(short, long)]
    time: usize,

    /// File to open
    filename: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn neighbors(self, max: u8) -> Vec<Self> {
        let mut neighbors = Vec::new();

        if self.x > 0 {
            neighbors.push(Self::new(self.x - 1, self.y));
        }
        if self.x < max {
            neighbors.push(Self::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            neighbors.push(Self::new(self.x, self.y - 1));
        }
        if self.y < max {
            neighbors.push(Self::new(self.x, self.y + 1));
        }

        neighbors
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    nodes: Vec<Position>,
    max: u8,
}

impl Path {
    fn new(start: Position, max: u8) -> Self {
        Self {
            nodes: vec![start],
            max,
        }
    }

    fn push(&mut self, position: Position) {
        self.nodes.push(position);
    }

    fn cost(&self) -> usize {
        let last = self.nodes.last().unwrap();
        let remaining = 2 * self.max - last.x - last.y;
        self.nodes.len() + usize::from(remaining)
    }
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost().cmp(&self.cost())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(corruptions: &HashMap<Position, usize>, max: u8, time: usize) -> Option<Path> {
    let mut best_cost = HashMap::new();

    let mut queue = BinaryHeap::from([Path::new(Position::new(0, 0), max)]);
    while let Some(path) = queue.pop() {
        let last = *path.nodes.last().unwrap();

        if let Some(best) = best_cost.get(&last) {
            if *best <= path.cost() {
                continue;
            }
        }

        best_cost.insert(last, path.cost());

        if last == Position::new(max, max) {
            return Some(path);
        }

        for neighbor in last.neighbors(max) {
            if path.nodes.iter().any(|previous| *previous == neighbor) {
                continue;
            }

            if let Some(t) = corruptions.get(&neighbor) {
                if *t < time {
                    continue;
                }
            }

            let mut next_path = path.clone();
            next_path.push(neighbor);
            queue.push(next_path);
        }
    }

    None
}

fn byte_for_time(corruptions: &[Position], time: usize) -> Position {
    corruptions[time - 1]
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let corruption_list = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut split = line.split(',');
            Position::new(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let corruption_map = corruption_list
        .iter()
        .enumerate()
        .map(|(time, position)| (*position, time))
        .collect::<HashMap<_, _>>();

    let path = shortest_path(&corruption_map, args.max, args.time).unwrap();
    println!("{}", path.nodes.len() - 1);

    let mut time = args.time;
    while !path
        .nodes
        .iter()
        .any(|node| *node == byte_for_time(&corruption_list, time))
    {
        time += 1;
    }

    while let Some(path) = shortest_path(&corruption_map, args.max, time) {
        while !path
            .nodes
            .iter()
            .any(|node| *node == byte_for_time(&corruption_list, time))
        {
            time += 1;
        }
    }

    let byte = byte_for_time(&corruption_list, time);
    println!("{},{}", byte.x, byte.y);
}
