#![warn(clippy::pedantic)]

use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    usize,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn distance_squared_to(self, other: Self) -> usize {
        let row_diff = self.row.abs_diff(other.row);
        let column_diff = self.column.abs_diff(other.column);
        row_diff * row_diff + column_diff * column_diff
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Blizzard = (Position, Direction);
type Width = usize;
type Height = usize;

// Width and height are the width of the blizzard region, exclusive of the walls
fn parse_blizzards(mut lines: impl Iterator<Item = String>) -> (Vec<Blizzard>, Width, Height) {
    let top_wall = lines.next().unwrap();
    let width = top_wall.len() - 2;

    let mut blizzards = Vec::new();
    let mut height = 0;
    for (row, line) in lines.enumerate() {
        let line = line.strip_prefix('#').unwrap().strip_suffix('#').unwrap();
        if line.starts_with('#') {
            break;
        }

        for (column, char) in line.chars().enumerate() {
            match char {
                '^' => blizzards.push((Position::new(row + 1, column), Direction::North)),
                '>' => blizzards.push((Position::new(row + 1, column), Direction::East)),
                'v' => blizzards.push((Position::new(row + 1, column), Direction::South)),
                '<' => blizzards.push((Position::new(row + 1, column), Direction::West)),
                '.' => (),
                _ => unimplemented!(),
            }
        }

        height += 1;
    }

    (blizzards, width, height)
}

struct VacancyCache {
    blizzards: Vec<Blizzard>,
    width: usize,
    height: usize,
    vacancies: Vec<HashSet<Position>>,
}

fn get_vacancies(blizzards: &[Blizzard], width: usize, height: usize) -> HashSet<Position> {
    let mut vacancies = HashSet::new();
    for row in 1..=height {
        for column in 0..width {
            vacancies.insert(Position::new(row, column));
        }
    }

    for (position, _) in blizzards {
        vacancies.remove(position);
    }

    vacancies.insert(Position::new(0, 0));
    vacancies.insert(Position::new(height + 1, width - 1));

    vacancies
}

impl VacancyCache {
    fn new(blizzards: Vec<Blizzard>, width: usize, height: usize) -> Self {
        Self {
            vacancies: vec![get_vacancies(&blizzards, width, height)],
            blizzards,
            width,
            height,
        }
    }

    fn resize_vacancies(&mut self, time: usize) {
        while self.vacancies.len() <= time {
            self.blizzards = self
                .blizzards
                .iter()
                .map(|(position, direction)| match direction {
                    Direction::North => (
                        Position::new(
                            (position.row + self.height - 2) % self.height + 1,
                            position.column,
                        ),
                        *direction,
                    ),
                    Direction::East => (
                        Position::new(position.row, (position.column + 1) % self.width),
                        *direction,
                    ),
                    Direction::South => (
                        Position::new(position.row % self.height + 1, position.column),
                        *direction,
                    ),
                    Direction::West => (
                        Position::new(
                            position.row,
                            (position.column + self.width - 1) % self.width,
                        ),
                        *direction,
                    ),
                })
                .collect();

            self.vacancies
                .push(get_vacancies(&self.blizzards, self.width, self.height));
        }
    }

    fn get_vacancies(&mut self, time: usize) -> &HashSet<Position> {
        self.resize_vacancies(time);
        &self.vacancies[time]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    current: Position,
    destination: Position,
    time: usize,
}

impl State {
    fn new(current: Position, destination: Position, time: usize) -> Self {
        Self {
            current,
            destination,
            time,
        }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let comparison = other.time.cmp(&self.time);
        match comparison {
            // First prefer the shortest time
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => comparison,

            // Otherwise prefer the one that is closer to the destination
            std::cmp::Ordering::Equal => other
                .current
                .distance_squared_to(other.destination)
                .cmp(&self.current.distance_squared_to(self.destination)),
        }
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(position: Position, width: usize, height: usize) -> Vec<Position> {
    let mut neighbors = Vec::new();

    if position.row > 0 {
        neighbors.push(Position::new(position.row - 1, position.column));
    }

    if position.row < height + 1 {
        neighbors.push(Position::new(position.row + 1, position.column));
    }

    if position.column > 0 {
        neighbors.push(Position::new(position.row, position.column - 1));
    }

    if position.column < width - 1 {
        neighbors.push(Position::new(position.row, position.column + 1));
    }

    neighbors
}

fn find_arrival_time(
    vacancy_cache: &mut VacancyCache,
    from: Position,
    to: Position,
    start_time: usize,
) -> usize {
    let width = vacancy_cache.width;
    let height = vacancy_cache.height;

    let mut visited = HashSet::new();

    let mut queue = BinaryHeap::new();
    queue.push(State::new(from, to, start_time));

    while let Some(state) = queue.pop() {
        if visited.contains(&state) {
            continue;
        }

        visited.insert(state);

        let vacancies = vacancy_cache.get_vacancies(state.time + 1);

        // Check neighbors
        let neighbors = get_neighbors(state.current, width, height);
        for neighbor in neighbors {
            if neighbor == state.destination {
                return state.time + 1;
            }

            if vacancies.contains(&neighbor) {
                queue.push(State::new(neighbor, state.destination, state.time + 1));
            }
        }

        // Check staying in place
        if vacancies.contains(&state.current) {
            queue.push(State::new(state.current, state.destination, state.time + 1));
        }
    }

    unreachable!()
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let (blizzards, width, height) = parse_blizzards(lines);
    let mut vacancy_cache = VacancyCache::new(blizzards, width, height);

    let start = Position::new(0, 0);
    let end = Position::new(height + 1, width - 1);

    let initial_time = find_arrival_time(&mut vacancy_cache, start, end, 0);
    println!("Initial time: {initial_time}");

    let return_time = find_arrival_time(&mut vacancy_cache, end, start, initial_time);
    println!("Return time: {return_time}");

    let final_time = find_arrival_time(&mut vacancy_cache, start, end, return_time);
    println!("Final time: {final_time}");
}
