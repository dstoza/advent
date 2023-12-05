#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::Range,
};

fn intersect(a: &Range<i64>, b: Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
    if b.end <= a.start || b.start >= a.end {
        (None, vec![a.clone()])
    } else {
        let intersection = a.start.max(b.start)..a.end.min(b.end);

        let mut remainder = Vec::new();

        let beginning = a.start..b.start;
        if !beginning.is_empty() {
            remainder.push(beginning);
        }

        let end = b.end..a.end;
        if !end.is_empty() {
            remainder.push(end)
        }

        (Some(intersection), remainder)
    }
}

#[derive(Clone, Debug)]
struct Map {
    destination: i64,
    source: i64,
    length: i64,
}

impl Map {
    fn parse(line: &str) -> Self {
        let mut split = line.split_whitespace();
        Self {
            destination: split.next().unwrap().parse().unwrap(),
            source: split.next().unwrap().parse().unwrap(),
            length: split.next().unwrap().parse().unwrap(),
        }
    }

    fn source_range(&self) -> Range<i64> {
        self.source..self.source + self.length
    }

    fn offset(&self) -> i64 {
        self.destination - self.source
    }

    fn map_ranges(&self, ranges: &mut Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut mapped = Vec::new();
        let remainder = ranges
            .iter()
            .flat_map(|range| {
                let (intersection, remainder) = intersect(range, self.source_range());
                match intersection {
                    Some(intersection) => {
                        let start = intersection.start + self.offset();
                        let length = intersection.end - intersection.start;
                        mapped.push(start..start + length);
                        remainder
                    }
                    None => vec![range.clone()],
                }
            })
            .collect::<Vec<_>>();

        // Preserve the remainder in the input for future maps
        *ranges = remainder;

        mapped
    }
}

fn map_ranges(mut ranges: Vec<Range<i64>>, map_sets: &[Map]) -> Vec<Range<i64>> {
    let mut mapped = Vec::new();
    for map_set in map_sets {
        mapped.append(&mut map_set.map_ranges(&mut ranges));
    }
    // Pass through anything that wasn't mapped
    mapped.append(&mut ranges);
    mapped
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut map_sets = Vec::new();
    let mut current_set = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if !current_set.is_empty() {
                map_sets.push(current_set.clone());
                current_set.clear();
            }
            lines.next().unwrap();
            continue;
        }

        current_set.push(Map::parse(&line));
    }
    map_sets.push(current_set);

    let nearest_as_individual = seeds
        .iter()
        .flat_map(|seed| {
            let mut ranges = vec![*seed..*seed + 1];
            for map_set in &map_sets {
                ranges = map_ranges(ranges, map_set.as_slice());
            }
            ranges
        })
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("{nearest_as_individual}");

    let nearest_as_ranges = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .flat_map(|range| {
            let mut ranges = vec![range];
            for map_set in &map_sets {
                ranges = map_ranges(ranges, map_set.as_slice());
            }
            ranges
        })
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("{nearest_as_ranges}");
}
