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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

fn all_shortest(keypad: &[&[u8]], start: Position, end: Position) -> Vec<Vec<Direction>> {
    let mut all_directions = Vec::new();
    let mut shortest = None;
    let mut queue = VecDeque::from([(vec![start], vec![])]);
    while let Some((path, directions)) = queue.pop_front() {
        if let Some(shortest) = shortest {
            if path.len() > shortest {
                continue;
            }
        }

        if *path.last().unwrap() == end {
            shortest =
                shortest.map_or_else(|| Some(path.len()), |current| Some(current.min(path.len())));
            all_directions.push(directions);
            continue;
        }

        for (direction, neighbor) in path
            .last()
            .unwrap()
            .neighbors(keypad[0].len(), keypad.len())
        {
            if path.iter().any(|previous| *previous == neighbor) {
                continue;
            }

            if keypad[neighbor.row][neighbor.column] == b'.' {
                continue;
            }

            let mut next_path = path.clone();
            next_path.push(neighbor);
            let mut next_directions = directions.clone();
            next_directions.push(direction);
            queue.push_back((next_path, next_directions));
        }
    }

    all_directions
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

fn shortest_expanded(
    keypad: &[&[u8]],
    start: Position,
    end: Position,
    directional_paths: &HashMap<(u8, u8), Vec<Direction>>,
) -> usize {
    all_shortest(keypad, start, end)
        .into_iter()
        .map(|path| {
            let mut expanded = direction_string(&path);
            expanded.push('A');
            let expanded = get_directions(expanded, directional_paths);
            let expanded = get_directions(expanded, directional_paths);

            (path.clone(), expanded.len())
        })
        .min_by_key(|(_path, length)| *length)
        .unwrap()
        .1
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

    let mut positions = HashMap::new();
    for (row, line) in NUMERIC.iter().enumerate() {
        for (column, value) in line.iter().enumerate() {
            positions.insert(*value, Position::new(row, column));
        }
    }

    let mut sum = 0;
    for mut code in reader.lines().map(Result::unwrap) {
        let numeric_value = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        let mut length = 0;
        code.insert(0, 'A');
        for pair in code.as_bytes().windows(2) {
            length += shortest_expanded(
                &NUMERIC,
                *positions.get(&pair[0]).unwrap(),
                *positions.get(&pair[1]).unwrap(),
                &directional_paths,
            );
        }

        sum += numeric_value * length;
    }

    println!("{sum}");
}
