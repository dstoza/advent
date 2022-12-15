#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::RangeInclusive,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn from_str(s: &str) -> Self {
        let mut split = s.split(", ");
        Self {
            x: split
                .next()
                .unwrap()
                .strip_prefix("x=")
                .unwrap()
                .parse()
                .unwrap(),
            y: split
                .next()
                .unwrap()
                .strip_prefix("y=")
                .unwrap()
                .parse()
                .unwrap(),
        }
    }

    fn distance_to(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct RangeSet {
    ranges: Vec<RangeInclusive<i64>>,
}

impl RangeSet {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn len(&self) -> usize {
        self.ranges
            .iter()
            .map(|range| range.end().abs_diff(*range.start()) as usize + 1)
            .sum()
    }

    fn insert(&mut self, range: RangeInclusive<i64>) {
        self.ranges.push(range);
    }

    #[allow(clippy::range_minus_one)]
    fn remove(&mut self, value: i64) {
        let mut split = Vec::new();

        for range in &self.ranges {
            if range.contains(&value) {
                split.push(*range.start()..=value - 1);
                split.push(value + 1..=*range.end());
            } else {
                split.push(range.clone());
            }
        }

        self.ranges = split;
    }

    fn coalesce(&mut self) {
        self.ranges.sort_unstable_by_key(|range| *range.start());

        let mut coalesced = Vec::new();
        let mut ranges = self.ranges.iter();
        let mut previous = ranges.next().unwrap().clone();
        for current in ranges {
            if current.start() <= previous.end() {
                previous = *previous.start()..=*previous.end().max(current.end());
            } else {
                coalesced.push(previous);
                previous = current.clone();
            }
        }
        coalesced.push(previous);

        self.ranges = coalesced;
    }

    fn clamp(&mut self, range: RangeInclusive<i64>) {
        self.ranges = self
            .ranges
            .iter()
            .filter_map(|r| {
                if r.end() < range.start() || r.start() > range.end() {
                    None
                } else {
                    Some(*r.start().max(range.start())..=*r.end().min(range.end()))
                }
            })
            .collect();
    }
}

fn parse_sensor_beacon_pairs(lines: impl Iterator<Item = String>) -> Vec<(Location, Location)> {
    let mut pairs = Vec::new();

    for line in lines {
        let mut split = line.split(':');
        let sensor = Location::from_str(split.next().unwrap().strip_prefix("Sensor at ").unwrap());
        let beacon = Location::from_str(
            split
                .next()
                .unwrap()
                .strip_prefix(" closest beacon is at ")
                .unwrap(),
        );
        pairs.push((sensor, beacon));
    }

    pairs
}

fn get_impossible_positions(
    sensor_beacon_pairs: &[(Location, Location)],
    row: i64,
    clamp: Option<RangeInclusive<i64>>,
) -> RangeSet {
    let mut impossible_positions = RangeSet::new();

    for (sensor, beacon) in sensor_beacon_pairs {
        let distance_to_beacon = sensor.distance_to(*beacon);
        let distance_to_row = (sensor.y - row).abs();
        if distance_to_row > distance_to_beacon {
            continue;
        }

        let spread = distance_to_beacon - distance_to_row;
        impossible_positions.insert((sensor.x - spread)..=(sensor.x + spread));
    }

    impossible_positions.coalesce();

    if let Some(clamp_range) = clamp {
        impossible_positions.clamp(clamp_range);
    } else {
        for (_sensor, beacon) in sensor_beacon_pairs {
            if beacon.y == row {
                impossible_positions.remove(beacon.x);
            }
        }
    }

    impossible_positions
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn find_possible_position(sensor_beacon_pairs: &[(Location, Location)], max_row: i64) -> Location {
    for row in 0..=max_row {
        let impossible_positions =
            get_impossible_positions(sensor_beacon_pairs, row, Some(0..=max_row));
        if impossible_positions.len() != (max_row + 1) as usize {
            return Location::new(impossible_positions.ranges[0].end() + 1, row);
        }
    }

    unreachable!()
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let row = std::env::args()
        .nth(2)
        .expect("Row to check not found")
        .parse()
        .unwrap();
    let clamp = std::env::args()
        .nth(3)
        .expect("Clamp value not found")
        .parse()
        .unwrap();

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let sensor_beacon_pairs = parse_sensor_beacon_pairs(lines);

    let impossible_position_count = get_impossible_positions(&sensor_beacon_pairs, row, None).len();
    println!("Impossible positions in row {row}: {impossible_position_count}");

    let possible_position = find_possible_position(&sensor_beacon_pairs, clamp);
    println!(
        "Tuning frequency: {}",
        possible_position.x * 4_000_000 + possible_position.y
    );
}
