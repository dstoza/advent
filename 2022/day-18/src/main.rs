#![warn(clippy::pedantic)]

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::RangeInclusive,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn parse(string: &str) -> Self {
        let mut split = string.split(',');
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }

    fn get_neighbors(&self) -> [Self; 6] {
        [
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y, self.z + 1),
            Self::new(self.x, self.y, self.z - 1),
        ]
    }

    fn is_in_bounds(&self, bounds: &RangeInclusive<i32>) -> bool {
        bounds.contains(&self.x) && bounds.contains(&self.y) && bounds.contains(&self.z)
    }
}

fn calculate_surface_area(points: &HashSet<Point>) -> usize {
    points
        .iter()
        .map(|point| {
            point
                .get_neighbors()
                .iter()
                .filter(|neighbor| !points.contains(neighbor))
                .count()
        })
        .sum()
}

fn get_bounds(points: impl Iterator<Item = Point>) -> RangeInclusive<i32> {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for point in points {
        min = min.min(point.x).min(point.y).min(point.z);
        max = max.max(point.x).max(point.y).max(point.z);
    }
    min..=max
}

fn calculate_exterior_surface_area(points: &HashSet<Point>) -> usize {
    let bounds = get_bounds(points.iter().copied());
    let bounds = bounds.start() - 1..=bounds.end() + 1;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let start = Point::new(*bounds.start(), *bounds.start(), *bounds.start());
    visited.insert(start);
    queue.push_back(start);

    let mut surface_area = 0;
    while let Some(point) = queue.pop_front() {
        for neighbor in &point.get_neighbors() {
            if points.contains(neighbor) {
                surface_area += 1;
            } else if neighbor.is_in_bounds(&bounds) && !visited.contains(neighbor) {
                queue.push_back(*neighbor);
                visited.insert(*neighbor);
            }
        }
    }

    surface_area
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let points: HashSet<_> = lines.map(|line| Point::parse(&line)).collect();

    let surface_area: usize = calculate_surface_area(&points);
    println!("Surface area: {surface_area}");

    let external_surface_area = calculate_exterior_surface_area(&points);
    println!("External surface area: {external_surface_area}");
}
