#![warn(clippy::pedantic)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use smallvec::SmallVec;

type SegmentsKey = (Vec<Vec<u8>>, SmallVec<[u8; 32]>);
type SegmentKey = (SmallVec<[u8; 32]>, SmallVec<[u8; 32]>);

#[derive(Default)]
struct Cache {
    segments_data: HashMap<SegmentsKey, usize>,
    segment_data: HashMap<SegmentKey, usize>,
    hits: usize,
    misses: usize,
    count_arrangement_calls: usize,
}

impl Cache {
    fn get_segments(&mut self, key: &SegmentsKey) -> Option<usize> {
        if let Some(value) = self.segments_data.get(key) {
            self.hits += 1;
            Some(*value)
        } else {
            self.misses += 1;
            None
        }
    }

    fn insert_segments(&mut self, key: SegmentsKey, value: usize) {
        self.segments_data.insert(key, value);
    }

    fn get_segment(&mut self, key: &SegmentKey) -> Option<usize> {
        if let Some(value) = self.segment_data.get(key) {
            self.hits += 1;
            Some(*value)
        } else {
            self.misses += 1;
            None
        }
    }

    fn insert_segment(&mut self, key: SegmentKey, value: usize) {
        self.segment_data.insert(key, value);
    }
}

fn count_segment_arrangements(segment: &[u8], lengths: &[u8], cache: &mut Cache) -> usize {
    let key = (SmallVec::from(segment), SmallVec::from(lengths));
    if let Some(arrangements) = cache.get_segment(&key) {
        return arrangements;
    }

    if segment.iter().any(|b| *b == b'#') && lengths.is_empty() {
        cache.insert_segment(key, 0);
        return 0;
    }

    if lengths.is_empty() {
        cache.insert_segment(key, 1);
        return 1;
    }

    if usize::from(lengths.iter().copied().sum::<u8>()) + lengths.len() - 1 > segment.len() {
        cache.insert_segment(key, 0);
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

            let remainder = if start + first_length + 1 < segment.len() {
                &segment[start + first_length + 1..]
            } else {
                &[]
            };

            Some(count_segment_arrangements(remainder, &lengths[1..], cache))
        })
        .collect::<Vec<_>>();

    let sum = arrangements.iter().sum();
    cache.insert_segment(key, sum);
    sum
}

fn count_arrangements(segments: &[Vec<u8>], lengths: &[u8], cache: &mut Cache) -> usize {
    let key = (segments.to_vec(), SmallVec::from(lengths));
    if let Some(arrangements) = cache.get_segments(&key) {
        return arrangements;
    }

    if segments.len() == 1 {
        let count = count_segment_arrangements(&segments[0], lengths, cache);
        // println!("returning {count}");
        cache.insert_segments(key, count);
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

    cache.insert_segments(key, count);
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
}
