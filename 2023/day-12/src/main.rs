#![feature(iter_intersperse)]
#![warn(clippy::pedantic)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use smallvec::SmallVec;

type Key = SmallVec<[u8; 64]>;

#[derive(Default)]
struct Cache {
    data: HashMap<Key, usize>,
    hits: usize,
    misses: usize,
}

impl Cache {
    fn get(&mut self, key: &Key) -> Option<usize> {
        if let Some(value) = self.data.get(key) {
            self.hits += 1;
            Some(*value)
        } else {
            self.misses += 1;
            None
        }
    }

    fn insert(&mut self, key: Key, value: usize) {
        self.data.insert(key, value);
    }
}

fn count_segment_arrangements(segment: &[u8], lengths: &[u8], cache: &mut Cache) -> usize {
    let mut key = SmallVec::from(segment);
    key.push(b' ');
    key.extend_from_slice(lengths);
    if let Some(arrangements) = cache.get(&key) {
        return arrangements;
    }

    if segment.iter().any(|b| *b == b'#') && lengths.is_empty() {
        cache.insert(key, 0);
        return 0;
    }

    if lengths.is_empty() {
        cache.insert(key, 1);
        return 1;
    }

    if usize::from(lengths.iter().copied().sum::<u8>()) + lengths.len() - 1 > segment.len() {
        cache.insert(key, 0);
        return 0;
    }

    let first_length = usize::from(lengths[0]);
    let arrangements = (0..=segment.len() - first_length)
        .filter_map(|start| {
            if start + first_length < segment.len() && segment[start + first_length] == b'#' {
                return None;
            }

            if segment[0..start].iter().any(|b| *b == b'#') {
                return None;
            }

            if start + first_length + 1 < segment.len() {
                let remainder = &segment[start + first_length + 1..];
                Some(count_segment_arrangements(remainder, &lengths[1..], cache))
            } else {
                Some(usize::from(lengths.len() == 1))
            }
        })
        .collect::<Vec<_>>();

    let sum = arrangements.iter().sum();
    cache.insert(key, sum);
    sum
}

fn count_arrangements(segments: &[Vec<u8>], lengths: &[u8], cache: &mut Cache) -> usize {
    let key = segments
        .iter()
        .map(std::vec::Vec::as_slice)
        .collect::<Vec<_>>();
    let key = key.as_slice().join(&b'.');

    let known: u8 = lengths.iter().sum();
    let spaces: usize = segments.iter().map(std::vec::Vec::len).sum();
    if usize::from(known) > spaces {
        return 0;
    }

    let seen = bytecount::count(&key, b'#');
    if seen > usize::from(known) {
        return 0;
    }

    let mut key = SmallVec::from(key);
    key.push(b' ');
    key.extend_from_slice(lengths);
    if let Some(arrangements) = cache.get(&key) {
        return arrangements;
    }

    if segments.len() == 1 {
        let count = count_segment_arrangements(&segments[0], lengths, cache);
        return count;
    }

    let mut count = 0;

    let first_segment = segments[0].as_slice();
    if !first_segment.iter().any(|b| *b == b'#') {
        count += count_arrangements(&segments[1..], lengths, cache);
    }

    for taken_lengths in 1..=lengths.len() {
        let arrangements =
            count_segment_arrangements(first_segment, &lengths[0..taken_lengths], cache);
        count +=
            arrangements * count_arrangements(&segments[1..], &lengths[taken_lengths..], cache);
    }

    cache.insert(key, count);
    count
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cache = Cache::default();

    let sum: usize = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let mut split = line.split_whitespace();

            let segments = split.next().unwrap();
            let segments = (0..5).map(|_| segments).collect::<Vec<_>>();
            let segments = segments.join("?");
            let segments = segments
                .split('.')
                .filter_map(|segment| {
                    if segment.is_empty() {
                        None
                    } else {
                        Some(Vec::from(segment.as_bytes()))
                    }
                })
                .collect::<Vec<_>>();

            let lengths = split
                .next()
                .unwrap()
                .split(',')
                .map(|length| length.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            let lengths = lengths
                .iter()
                .copied()
                .cycle()
                .take(5 * lengths.len())
                .collect::<Vec<_>>();

            count_arrangements(&segments, &lengths, &mut cache)
        })
        .sum();

    println!("{sum}");
    println!("hits {} misses {}", cache.hits, cache.misses);
}
