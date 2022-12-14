#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Location {
    x: u16,
    y: u16,
}

impl Location {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn from_str(s: &str) -> Self {
        let mut split = s.split(',');
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
        }
    }

    fn below(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn diagonal_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn diagonal_right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn get_rocks_for_path(from: Location, to: Location) -> Vec<Location> {
    if from.x == to.x {
        (from.y.min(to.y)..=from.y.max(to.y))
            .map(|y| Location::new(from.x, y))
            .collect()
    } else {
        assert!(from.y == to.y);
        (from.x.min(to.x)..=from.x.max(to.x))
            .map(|x| Location::new(x, from.y))
            .collect()
    }
}

fn parse_cave(lines: impl Iterator<Item = String>) -> HashSet<Location> {
    let mut cave = HashSet::new();

    for line in lines {
        let mut split = line.split(" -> ");
        let mut previous = Location::from_str(split.next().unwrap());
        for current in split {
            let current = Location::from_str(current);
            for rock in get_rocks_for_path(previous, current) {
                cave.insert(rock);
            }
            previous = current;
        }
    }

    cave
}

// Returns whether the sand fell
fn falls_down(sand_location: &mut Location, cave: &HashSet<Location>) -> bool {
    if !cave.contains(&sand_location.below()) {
        *sand_location = sand_location.below();
        true
    } else if !cave.contains(&sand_location.diagonal_left()) {
        *sand_location = sand_location.diagonal_left();
        true
    } else if !cave.contains(&sand_location.diagonal_right()) {
        *sand_location = sand_location.diagonal_right();
        true
    } else {
        false
    }
}

fn count_drops(mut cave: HashSet<Location>, void_depth: u16) -> (usize, usize) {
    let mut drops_until_void = None;
    let mut drops = 0;

    while !cave.contains(&Location::new(500, 0)) {
        let mut sand_location = Location::new(500, 0);
        loop {
            if sand_location.y == void_depth - 1 {
                // Sand comes to rest on infinite floor
                if drops_until_void.is_none() {
                    drops_until_void = Some(drops);
                }
                cave.insert(sand_location);
                break;
            }

            if falls_down(&mut sand_location, &cave) {
                continue;
            }

            // Sand comes to rest
            cave.insert(sand_location);
            break;
        }

        drops += 1;
    }

    (drops_until_void.unwrap(), drops)
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let cave = parse_cave(lines);

    let void_depth = cave.iter().map(|rock| rock.y).max().unwrap() + 2;

    let (drops_until_void, drops_until_full) = count_drops(cave, void_depth);
    println!("{drops_until_void} drops until void");
    println!("{drops_until_full} drops until full");
}
