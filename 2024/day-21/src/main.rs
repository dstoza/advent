#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Number of robots using keypads
    #[arg(short, long)]
    robots: usize,

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

fn get_directions(
    first: u8,
    pairs: HashMap<(u8, u8), usize>,
    paths: &HashMap<(u8, u8), Vec<Direction>>,
) -> (u8, HashMap<(u8, u8), usize>) {
    let mut expanded = HashMap::new();

    let first = {
        let mut directions = paths.get(&(b'A', first)).unwrap().to_owned();
        directions.push(Direction::Activate);
        let directions = direction_string(&directions);
        let first = directions.as_bytes()[0];
        for pair in directions.as_bytes().windows(2) {
            expanded
                .entry((pair[0], pair[1]))
                .and_modify(|count| *count += 1)
                .or_insert(1usize);
        }

        first
    };

    for (pair, count) in pairs {
        let mut directions = paths.get(&pair).unwrap().to_owned();
        directions.push(Direction::Activate);
        let mut directions = direction_string(&directions);
        directions.insert(0, 'A');
        for pair in directions.as_bytes().windows(2) {
            expanded
                .entry((pair[0], pair[1]))
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    (first, expanded)
}

fn shortest_expanded(
    keypad: &[&[u8]],
    start: Position,
    end: Position,
    robots: usize,
    directional_paths: &HashMap<(u8, u8), Vec<Direction>>,
) -> usize {
    all_shortest(keypad, start, end)
        .into_iter()
        .map(|mut path| {
            path.push(Direction::Activate);

            let expanded_str = direction_string(&path);

            let mut expanded = HashMap::new();
            for pair in expanded_str.as_bytes().windows(2) {
                expanded
                    .entry((pair[0], pair[1]))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            let mut first = *expanded_str.as_bytes().first().unwrap();
            for _ in 0..robots {
                (first, expanded) = get_directions(first, expanded, directional_paths);
            }

            expanded.values().sum::<usize>() + 1
        })
        .min()
        .unwrap()
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
        ((b'A', b'v'), vec![Direction::Left, Direction::Down]),
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
                args.robots,
                &directional_paths,
            );
        }

        sum += numeric_value * length;
    }

    println!("{sum}");
}
