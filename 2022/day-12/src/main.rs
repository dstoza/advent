#![warn(clippy::pedantic)]

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

type Heightmap = Vec<Vec<u8>>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    if from.column > 0 && heightmap[from.row][from.column - 1] <= from_height + 1 {
        potential_moves.push(Location::new(from.row, from.column - 1));
    }

    // Right
    if from.column < heightmap[from.row].len() - 1
        && heightmap[from.row][from.column + 1] <= from_height + 1
    {
        potential_moves.push(Location::new(from.row, from.column + 1));
    }

    // Up
    if from.row > 0 && heightmap[from.row - 1][from.column] <= from_height + 1 {
        potential_moves.push(Location::new(from.row - 1, from.column));
    }

    // Down
    if from.row < heightmap.len() - 1 && heightmap[from.row + 1][from.column] <= from_height + 1 {
        potential_moves.push(Location::new(from.row + 1, from.column));
    }

    potential_moves
}

fn get_shortest_path_length(
    heightmap: &Heightmap,
    start: Location,
    end: Location,
) -> Option<usize> {
    let mut shortest_to_location = Vec::new();
    for _ in 0..heightmap.len() {
        shortest_to_location.push(vec![usize::MAX; heightmap[0].len()]);
    }

    let mut queue = VecDeque::new();
    queue.push_back(vec![start]);

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();

        let last_location = *path.last().unwrap();
        if shortest_to_location[last_location.row][last_location.column] <= path.len() {
            continue;
        }

        shortest_to_location[last_location.row][last_location.column] = path.len();

        let potential_moves = get_potential_moves(*path.last().unwrap(), heightmap);

        if potential_moves.contains(&end) {
            return Some(path.len());
        }

        for potential_move in potential_moves.iter().filter(|m| !path.contains(m)) {
            let mut new_path = path.clone();
            new_path.push(*potential_move);
            queue.push_back(new_path);
        }
    }

    None
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let (heightmap, start, end) = parse_map(lines);

    println!(
        "From current position: {}",
        get_shortest_path_length(&heightmap, start, end).unwrap()
    );

    let mut minimum = usize::MAX;
    for row in 0..heightmap.len() {
        for column in 0..heightmap[row].len() {
            if heightmap[row][column] != 0 {
                break;
            }

            if let Some(length) =
                get_shortest_path_length(&heightmap, Location::new(row, column), end)
            {
                minimum = minimum.min(length);
            }
        }
    }
    println!("From best position: {minimum}");
}
