#![feature(test)]
extern crate test;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const END: u8 = 12;
const START: u8 = END - 1;

struct TinyMap {
    entries: [(u8, [u8; TinyMap::MAX_LENGTH]); END as usize * 2],
}

impl TinyMap {
    const MAX_LENGTH: usize = 15;

    fn new() -> Self {
        Self {
            entries: [(0, [255; TinyMap::MAX_LENGTH]); END as usize * 2],
        }
    }

    fn push(&mut self, key: u8, value: u8) {
        let (length, values) = &mut self.entries[key as usize];
        values[*length as usize] = value;
        *length += 1;
        assert!((*length as usize) < TinyMap::MAX_LENGTH);
    }

    fn at(&self, key: u8) -> &[u8] {
        let (length, values) = &self.entries[key as usize];
        &values[..*length as usize]
    }
}

fn is_lowercase(s: &str) -> bool {
    s != "end" && s.chars().next().unwrap().is_lowercase()
}

fn get_symbol(
    symbols: &mut HashMap<String, u8>,
    next_lowercase: &mut u8,
    next_uppercase: &mut u8,
    s: &str,
) -> u8 {
    *symbols.entry(String::from(s)).or_insert_with(|| {
        if is_lowercase(s) {
            *next_lowercase += 1;
            assert!(*next_lowercase < START);
            *next_lowercase - 1
        } else {
            *next_uppercase += 1;
            *next_uppercase - 1
        }
    })
}

fn parse_neighbors<I: Iterator<Item = String>>(lines: I) -> TinyMap {
    let mut neighbors = TinyMap::new();

    let mut symbols = HashMap::from([(String::from("start"), START), (String::from("end"), END)]);

    let mut next_lowercase = 0;
    let mut next_uppercase = END + 1;

    for line in lines {
        let mut split = line.split('-');
        let from = split.next().unwrap();
        let from = get_symbol(&mut symbols, &mut next_lowercase, &mut next_uppercase, from);

        let to = split.next().unwrap();
        let to = get_symbol(&mut symbols, &mut next_lowercase, &mut next_uppercase, to);

        if to != START {
            neighbors.push(from, to);
        }
        if from != START {
            neighbors.push(to, from);
        }
    }

    neighbors
}

fn do_count_paths(
    neighbors: &TinyMap,
    allow_duplicates: bool,
    previous_lowercase: &mut Vec<u8>,
    has_duplicate: bool,
    current_cave: u8,
) -> usize {
    if current_cave == END {
        return 1;
    }

    let mut paths = 0;

    for neighbor in neighbors.at(current_cave) {
        let neighbor_is_lowercase = *neighbor < START;
        let has_duplicate = if neighbor_is_lowercase
            && previous_lowercase
                .iter()
                .find(|element| *element == neighbor)
                != None
        {
            if !allow_duplicates || has_duplicate {
                continue;
            }
            true
        } else {
            has_duplicate
        };

        if neighbor_is_lowercase {
            previous_lowercase.push(*neighbor);
        }
        paths += do_count_paths(
            neighbors,
            allow_duplicates,
            previous_lowercase,
            has_duplicate,
            *neighbor,
        );
        if neighbor_is_lowercase {
            previous_lowercase.pop();
        }
    }

    paths
}

fn count_paths(neighbors: &TinyMap, allow_duplicates: bool) -> usize {
    do_count_paths(neighbors, allow_duplicates, &mut Vec::new(), false, START)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let neighbors = parse_neighbors(reader.lines().map(|l| l.unwrap()));
    println!("Paths: {}", count_paths(&neighbors, true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_simple() -> [String; 7] {
        [
            String::from("start-A"),
            String::from("start-b"),
            String::from("A-c"),
            String::from("A-b"),
            String::from("b-d"),
            String::from("A-end"),
            String::from("b-end"),
        ]
    }

    fn get_slightly_larger() -> [String; 10] {
        [
            String::from("dc-end"),
            String::from("HN-start"),
            String::from("start-kj"),
            String::from("dc-start"),
            String::from("dc-HN"),
            String::from("LN-dc"),
            String::from("HN-end"),
            String::from("kj-sa"),
            String::from("kj-HN"),
            String::from("kj-dc"),
        ]
    }

    fn get_even_larger() -> [String; 18] {
        [
            String::from("fs-end"),
            String::from("he-DX"),
            String::from("fs-he"),
            String::from("start-DX"),
            String::from("pj-DX"),
            String::from("end-zg"),
            String::from("zg-sl"),
            String::from("zg-pj"),
            String::from("pj-he"),
            String::from("RW-he"),
            String::from("fs-DX"),
            String::from("pj-RW"),
            String::from("zg-RW"),
            String::from("start-pj"),
            String::from("he-WI"),
            String::from("zg-he"),
            String::from("pj-fs"),
            String::from("start-RW"),
        ]
    }

    #[test]
    fn test_count_paths_simple() {
        let neighbors = parse_neighbors(get_simple().into_iter());
        assert_eq!(count_paths(&neighbors, false), 10);
    }

    #[test]
    fn test_count_paths_simple_with_duplicates() {
        let neighbors = parse_neighbors(get_simple().into_iter());
        assert_eq!(count_paths(&neighbors, true), 36);
    }

    #[test]
    fn test_count_paths_slightly_larger() {
        let neighbors = parse_neighbors(get_slightly_larger().into_iter());
        assert_eq!(count_paths(&neighbors, false), 19);
    }

    #[test]
    fn test_count_paths_slightly_larger_with_duplicates() {
        let neighbors = parse_neighbors(get_slightly_larger().into_iter());
        assert_eq!(count_paths(&neighbors, true), 103);
    }

    #[test]
    fn test_count_paths_even_larger() {
        let neighbors = parse_neighbors(get_even_larger().into_iter());
        assert_eq!(count_paths(&neighbors, false), 226);
    }

    #[test]
    fn test_count_paths_even_larger_with_duplicates() {
        let neighbors = parse_neighbors(get_even_larger().into_iter());
        assert_eq!(count_paths(&neighbors, true), 3509);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        b.iter(|| {
            let neighbors = parse_neighbors(lines.clone().into_iter());
            assert_eq!(count_paths(&neighbors, true), 84271);
        });
    }
}
