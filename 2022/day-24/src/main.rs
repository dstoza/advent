#![warn(clippy::pedantic)]

use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    usize,
};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn distance_squared_from_origin(self) -> usize {
        self.row * self.row + self.column * self.column
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
        let line = line.strip_prefix("#").unwrap().strip_suffix("#").unwrap();
        if line.starts_with("#") {
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

    fn get_vacancies<'a>(&'a mut self, time: usize) -> &'a HashSet<Position> {
        self.resize_vacancies(time);
        &self.vacancies[time]
    }

    fn print_vacancies(&mut self, time: usize) {
        let width = self.width;
        let height = self.height;
        let vacancies = self.get_vacancies(time);
        for row in 0..=height + 1 {
            print!("#");
            for column in 0..width {
                if vacancies.contains(&Position::new(row, column)) {
                    print!(".");
                } else {
                    print!("*");
                }
            }
            println!("#");
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    position: Position,
    time: usize,
}

impl State {
    fn new(position: Position, time: usize) -> Self {
        Self { position, time }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let comparison = self
            .position
            .distance_squared_from_origin()
            .cmp(&other.position.distance_squared_from_origin());
        match comparison {
            // Pick the one farther from the origin
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => comparison,

            // If there is a tie, pick the one with less time elapsed
            std::cmp::Ordering::Equal => other.time.cmp(&self.time),
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
        
    }

    neighbors
}

fn find_shortest_path(vacancy_cache: &mut VacancyCache) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(State::new(Position::new(0, 0), 0));

    while let Some(state) = queue.pop() {

    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let (blizzards, width, height) = parse_blizzards(lines);
    let mut vacancy_cache = VacancyCache::new(blizzards, width, height);

    let shortest_path = find_shortest_path(&mut vacancy_cache);
}
