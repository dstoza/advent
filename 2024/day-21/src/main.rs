#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn neighbors(self, width: usize, height: usize) -> Vec<(Direction, Self)> {
        let mut neighbors = Vec::new();

        if self.row > 0 {
            neighbors.push((Direction::Up, Position::new(self.row - 1, self.column)));
        }
        if self.row < height - 1 {
            neighbors.push((Direction::Down, Position::new(self.row + 1, self.column)));
        }
        if self.column > 0 {
            neighbors.push((Direction::Left, Position::new(self.row, self.column - 1)));
        }
        if self.column < width - 1 {
            neighbors.push((Direction::Right, Position::new(self.row, self.column + 1)));
        }

        neighbors
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Activate,
}

fn direction_string(directions: &[Direction]) -> String {
    directions
        .iter()
        .map(|direction| match direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Activate => 'A',
        })
        .collect::<String>()
}

fn get_directions(mut string: String, paths: &HashMap<(u8, u8), Vec<Direction>>) -> String {
    string.insert(0, 'A');
    let mut directions = Vec::new();
    for pair in string.as_bytes().windows(2) {
        directions.extend_from_slice(paths.get(&(pair[0], pair[1])).unwrap());
        directions.push(Direction::Activate);
    }

    direction_string(&directions)
}

fn path_cost(path: &[Direction], directional_paths: &HashMap<(u8, u8), Vec<Direction>>) -> usize {
    let string = direction_string(path);
    let mut string = get_directions(string, directional_paths);
    string.insert(0, 'A');
    string
        .as_bytes()
        .windows(2)
        .map(|window| {
            directional_paths
                .get(&(window[0], window[1]))
                .unwrap()
                .len()
        })
        .sum()
}

fn paths(
    keypad: &[&[u8]],
    start_row: usize,
    start_column: usize,
    directional_paths: &HashMap<(u8, u8), Vec<Direction>>,
) -> HashMap<u8, Vec<Direction>> {
    let mut paths: HashMap<u8, Vec<Direction>> = HashMap::new();

    let mut queue = VecDeque::from([(Position::new(start_row, start_column), Vec::new())]);
    while let Some((position, path)) = queue.pop_front() {
        let value = keypad[position.row][position.column];
        let cost = path_cost(&path, directional_paths);
        if let Some(path) = paths.get(&value) {
            if path_cost(path, directional_paths) <= cost {
                continue;
            }
        }

        paths.insert(value, path.clone());

        for (direction, neighbor) in position.neighbors(keypad[0].len(), keypad.len()) {
            if keypad[neighbor.row][neighbor.column] == b'.' {
                continue;
            }

            let mut path_to_neighbor = path.clone();
            path_to_neighbor.push(direction);
            queue.push_back((neighbor, path_to_neighbor));
        }
    }

    paths
}

fn all_paths(
    keypad: &[&[u8]],
    directional_paths: &HashMap<(u8, u8), Vec<Direction>>,
) -> HashMap<(u8, u8), Vec<Direction>> {
    let mut all_paths = HashMap::new();

    for (row, line) in keypad.iter().enumerate() {
        for (column, value) in line.iter().enumerate() {
            if *value == b'.' {
                continue;
            }

            let paths_for_value = paths(keypad, row, column, directional_paths);
            for (destination, path) in paths_for_value {
                all_paths.insert((*value, destination), path);
            }
        }
    }

    all_paths
}

fn main() {
    const NUMERIC: [&[u8]; 4] = [
        b"789".as_slice(),
        b"456".as_slice(),
        b"123".as_slice(),
        b".0A".as_slice(),
    ];

    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let directional_paths = HashMap::from([
        ((b'^', b'^'), vec![]),
        ((b'^', b'A'), vec![Direction::Right]),
        ((b'^', b'<'), vec![Direction::Down, Direction::Left]),
        ((b'^', b'v'), vec![Direction::Down]),
        ((b'^', b'>'), vec![Direction::Down, Direction::Right]),
        ((b'A', b'^'), vec![Direction::Left]),
        ((b'A', b'A'), vec![]),
        (
            (b'A', b'<'),
            vec![Direction::Down, Direction::Left, Direction::Left],
        ),
        ((b'A', b'v'), vec![Direction::Down, Direction::Left]),
        ((b'A', b'>'), vec![Direction::Down]),
        ((b'<', b'^'), vec![Direction::Right, Direction::Up]),
        (
            (b'<', b'A'),
            vec![Direction::Right, Direction::Right, Direction::Up],
        ),
        ((b'<', b'<'), vec![]),
        ((b'<', b'v'), vec![Direction::Right]),
        ((b'<', b'>'), vec![Direction::Right, Direction::Right]),
        ((b'v', b'^'), vec![Direction::Up]),
        ((b'v', b'A'), vec![Direction::Up, Direction::Right]),
        ((b'v', b'<'), vec![Direction::Left]),
        ((b'v', b'v'), vec![]),
        ((b'v', b'>'), vec![Direction::Right]),
        ((b'>', b'^'), vec![Direction::Left, Direction::Up]),
        ((b'>', b'A'), vec![Direction::Up]),
        ((b'>', b'<'), vec![Direction::Left, Direction::Left]),
        ((b'>', b'v'), vec![Direction::Left]),
        ((b'>', b'>'), vec![]),
    ]);

    let numeric_paths = all_paths(&NUMERIC, &directional_paths);

    for ((from, to), directions) in &directional_paths {
        println!("{} {} {directions:?}", *from as char, *to as char);
    }

    let mut sum = 0;
    for mut code in reader.lines().map(Result::unwrap) {
        let numeric_value = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        println!("{code} {numeric_value}");
        let directions = get_directions(code, &numeric_paths);
        println!("{directions}");
        let directions = get_directions(directions, &directional_paths);
        println!("{directions}");
        let directions = get_directions(directions, &directional_paths);
        println!("{directions}");
        let complexity = numeric_value * directions.len();
        println!("{} {complexity}", directions.len());
        sum += complexity;
    }

    // 237348 too high

    println!("{sum}");
}
