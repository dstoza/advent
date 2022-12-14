#![warn(clippy::pedantic)]

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

type Heightmap = Vec<Vec<u8>>;

#[derive(Clone, Copy, Debug, Default)]
struct Location {
    row: usize,
    column: usize,
}

impl Location {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

fn parse_map(lines: impl Iterator<Item = String>) -> (Heightmap, Location, Location) {
    let mut heightmap = Heightmap::new();
    let mut start = None;
    let mut end = None;

    for (row, line) in lines.enumerate() {
        heightmap.push(Vec::new());
        let current_row = heightmap.last_mut().unwrap();
        for (column, c) in line.chars().enumerate() {
            match c {
                'a'..='z' => current_row.push(c as u8 - b'a'),
                'S' => {
                    current_row.push(0);
                    start = Some(Location::new(row, column));
                }
                'E' => {
                    current_row.push(25);
                    end = Some(Location::new(row, column));
                }
                _ => unimplemented!(),
            }
        }
    }

    (heightmap, start.unwrap(), end.unwrap())
}

fn get_potential_moves(from: Location, heightmap: &Heightmap) -> Vec<Location> {
    let from_height = heightmap[from.row][from.column];
    let mut potential_moves = Vec::new();

    // Left
    if from.column > 0 && heightmap[from.row][from.column - 1] + 1 >= from_height {
        potential_moves.push(Location::new(from.row, from.column - 1));
    }

    // Right
    if from.column < heightmap[from.row].len() - 1
        && heightmap[from.row][from.column + 1] + 1 >= from_height
    {
        potential_moves.push(Location::new(from.row, from.column + 1));
    }

    // Up
    if from.row > 0 && heightmap[from.row - 1][from.column] + 1 >= from_height {
        potential_moves.push(Location::new(from.row - 1, from.column));
    }

    // Down
    if from.row < heightmap.len() - 1 && heightmap[from.row + 1][from.column] + 1 >= from_height {
        potential_moves.push(Location::new(from.row + 1, from.column));
    }

    potential_moves
}

fn get_all_shortest_paths(heightmap: &Heightmap, end: Location) -> Vec<Vec<usize>> {
    let mut shortest_paths = vec![vec![usize::MAX; heightmap[0].len()]; heightmap.len()];
    shortest_paths[end.row][end.column] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(end);

    while !queue.is_empty() {
        let from = queue.pop_front().unwrap();
        let path_length_to_here = shortest_paths[from.row][from.column];

        for potential_move in get_potential_moves(from, heightmap) {
            if path_length_to_here + 1 < shortest_paths[potential_move.row][potential_move.column] {
                shortest_paths[potential_move.row][potential_move.column] = path_length_to_here + 1;
                queue.push_back(potential_move);
            }
        }
    }

    shortest_paths
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let (heightmap, start, end) = parse_map(lines);

    let shortest_paths = get_all_shortest_paths(&heightmap, end);

    println!(
        "From current position: {}",
        shortest_paths[start.row][start.column]
    );

    let heightmap_flattened = heightmap.iter().flat_map(|row| row.iter());
    let best_complete_length = shortest_paths
        .iter()
        .flat_map(|row| row.iter())
        .zip(heightmap_flattened)
        .filter_map(|(shortest_path, height)| {
            if *height == 0 {
                Some(shortest_path)
            } else {
                None
            }
        })
        .min()
        .unwrap();
    println!("From best position: {}", best_complete_length);
}
