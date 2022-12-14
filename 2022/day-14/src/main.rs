#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
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

fn count_drops_until_void(mut cave: HashSet<Location>, void_depth: i32) -> usize {
    let mut drops = 0;
    loop {
        let mut sand_location = Location::new(500, 0);
        loop {
            if sand_location.y == void_depth {
                // Sand has fallen into the void
                break;
            }

            if falls_down(&mut sand_location, &cave) {
                continue;
            }

            // Sand comes to rest
            cave.insert(sand_location);
            break;
        }

        if sand_location.y == void_depth {
            break;
        }

        drops += 1;
    }

    drops
}

fn count_drops_until_full(mut cave: HashSet<Location>, void_depth: i32) -> usize {
    let mut drops = 0;
    while !cave.contains(&Location::new(500, 0)) {
        let mut sand_location = Location::new(500, 0);
        loop {
            if sand_location.y == void_depth - 1 {
                // Sand comes to rest on infinite floor
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

    drops
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let cave = parse_cave(lines);

    let void_depth = cave.iter().map(|rock| rock.y).max().unwrap() + 2;

    let drops_until_void = count_drops_until_void(cave.clone(), void_depth);
    println!("{drops_until_void} drops until void");

    let drops_until_full = count_drops_until_full(cave, void_depth);
    println!("{drops_until_full} drops until full");
}
