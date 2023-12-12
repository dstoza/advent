#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_segment_arrangements(segment: &[u8], lengths: &[usize], depth: usize) -> usize {
    // println!(
    //     "{depth} count {} {lengths:?}",
    //     std::str::from_utf8(segment).unwrap()
    // );

    if segment.iter().any(|b| *b == b'#') && lengths.is_empty() {
        // println!("{depth} a returning 0");
        return 0;
    }

    if lengths.is_empty() {
        // println!("{depth} b returning 1");
        return 1;
    }

    if lengths.iter().copied().sum::<usize>() + lengths.len() - 1 > segment.len() {
        // println!("{depth} c returning 0");
        return 0;
    }

    let first_length = lengths[0];
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

            Some(count_segment_arrangements(
                remainder,
                &lengths[1..],
                depth + 1,
            ))
        })
        .collect::<Vec<_>>();

    // println!("{arrangements:?}");

    let sum = arrangements.iter().sum();

    // println!("{depth} d returning {sum}");
    sum
}

fn count_arrangements(segments: &[Vec<u8>], lengths: &[usize]) -> usize {
    // println!(
    //     "count_arrangements {:?} {lengths:?}",
    //     segments
    //         .iter()
    //         .map(|segment| String::from_utf8(segment.clone()).unwrap())
    //         .collect::<Vec<_>>()
    // );

    if segments.len() == 1 {
        let count = count_segment_arrangements(&segments[0], lengths, 1);
        // println!("returning {count}");
        return count;
    }

    let mut count = 0;

    let first_segment = segments[0].as_slice();
    if !first_segment.iter().any(|b| *b == b'#') {
        count += count_arrangements(&segments[1..], lengths);
    }

    for taken_lengths in 1..=lengths.len() {
        // println!("taken {taken_lengths}");
        let arrangements = count_segment_arrangements(first_segment, &lengths[0..taken_lengths], 1);
        count += arrangements * count_arrangements(&segments[1..], &lengths[taken_lengths..]);
    }

    count
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

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
                .map(|length| length.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let lengths = lengths
                .iter()
                .copied()
                .cycle()
                .take(5 * lengths.len())
                .collect::<Vec<_>>();

            println!(
                "{:?} {lengths:?}",
                segments
                    .iter()
                    .map(|segment| String::from_utf8(segment.clone()).unwrap())
                    .collect::<Vec<_>>()
            );

            let arrangements = count_arrangements(&segments, &lengths);
            // println!("{arrangements}");
            arrangements
        })
        .sum();

    println!("{sum}");
}
