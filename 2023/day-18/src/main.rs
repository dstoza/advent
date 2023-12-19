#![warn(clippy::pedantic)]

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn get_horizontal_segments(lines: impl Iterator<Item = String>) -> Vec<(i64, RangeInclusive<i64>)> {
    let mut row = 0;
    let mut column = 0;
    let mut segments: Vec<(i64, RangeInclusive<i64>)> = lines
        .filter_map(|line| {
            let mut split = line.split_whitespace();

            #[cfg(not(feature = "hex"))]
            let direction = split.next().unwrap();
            #[cfg(not(feature = "hex"))]
            let distance: i64 = split
                .next()
                .and_then(|distance| distance.parse().ok())
                .unwrap();

            #[cfg(feature = "hex")]
            let hex = split
                .nth(2)
                .unwrap()
                .trim_start_matches("(#")
                .trim_end_matches(')');
            #[cfg(feature = "hex")]
            let distance = i64::from_str_radix(&hex[0..5], 16).unwrap();
            #[cfg(feature = "hex")]
            let direction = match &hex[5..] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => unreachable!(),
            };

            match direction {
                "R" => {
                    let segment = column..=column + distance;
                    column += distance;
                    Some((row, segment))
                }
                "L" => {
                    let segment = column - distance..=column;
                    column -= distance;
                    Some((row, segment))
                }
                "D" => {
                    row += distance;
                    None
                }
                "U" => {
                    row -= distance;
                    None
                }
                _ => None,
            }
        })
        .collect();

    segments.sort_unstable_by(|(left_row, left_range), (right_row, right_range)| {
        match left_row.cmp(right_row) {
            Ordering::Equal => left_range.start().cmp(right_range.start()),
            _ => left_row.cmp(right_row),
        }
    });

    segments
}

fn get_contained_area(segments: &[(i64, RangeInclusive<i64>)]) -> i64 {
    let mut area = 0;

    let mut last_row = None;
    let mut open_segments: Vec<RangeInclusive<i64>> = Vec::new();
    let mut shrinkage = 0;

    let mut iterator = segments.iter().peekable();
    while let Some((row, segment)) = iterator.next() {
        if let Some(last_row) = last_row {
            if *row != last_row {
                shrinkage = 0;
                area += open_segments
                    .iter()
                    .map(|segment| *segment.end() - *segment.start() + 1)
                    .sum::<i64>()
                    * (*row - last_row - 1);
            }
        }

        if let Some(match_position) = open_segments.iter().position(|open| open == segment) {
            let removed = open_segments.remove(match_position);
            shrinkage += removed.end() - removed.start() + 1;
        } else if let Some(extend_left) = open_segments
            .iter_mut()
            .find(|open| open.start() == segment.end())
        {
            *extend_left = *segment.start()..=*extend_left.end();
        } else if let Some(shrink_left) = open_segments
            .iter_mut()
            .find(|open| open.start() == segment.start())
        {
            shrinkage += *segment.end() - *shrink_left.start();
            *shrink_left = *segment.end()..=*shrink_left.end();
        } else if let Some(extend_right) = open_segments
            .iter_mut()
            .find(|open| open.end() == segment.start())
        {
            *extend_right = *extend_right.start()..=*segment.end();
        } else if let Some(shrink_right) = open_segments
            .iter_mut()
            .find(|open| open.end() == segment.end())
        {
            shrinkage += *shrink_right.end() - *segment.start();
            *shrink_right = *shrink_right.start()..=*segment.start();
        } else if let Some(split_position) = open_segments
            .iter()
            .position(|open| *open.start() < *segment.start() && *open.end() > *segment.end())
        {
            shrinkage += segment.end() - segment.start() - 1;
            let split = open_segments.remove(split_position);
            open_segments.push(*split.start()..=*segment.start());
            open_segments.push(*segment.end()..=*split.end());
        } else {
            open_segments.push(segment.clone());
        }

        if !open_segments.is_empty() {
            open_segments.sort_unstable_by_key(|segment| *segment.start());
            let mut merged = Vec::new();
            let mut previous = open_segments[0].clone();
            for segment in &open_segments[1..] {
                if segment.start() == previous.end() {
                    previous = *previous.start()..=*segment.end();
                } else {
                    merged.push(previous);
                    previous = segment.clone();
                }
            }
            merged.push(previous);
            open_segments = merged;
        }

        let last_segment_of_row = match iterator.peek() {
            Some((next_row, _)) => next_row != row,
            None => true,
        };

        if last_segment_of_row {
            area += open_segments
                .iter()
                .map(|segment| *segment.end() - *segment.start() + 1)
                .sum::<i64>()
                + shrinkage;
        }

        last_row = Some(*row);
    }

    area
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let horizontal_segments =
        get_horizontal_segments(reader.lines().map(std::result::Result::unwrap));

    let contained_area = get_contained_area(&horizontal_segments);
    println!("{contained_area}");
}
