#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn main() {
    // const DIGITS: [(&str, i32); 9] = [
    //     ("1", 1),
    //     ("2", 2),
    //     ("3", 3),
    //     ("4", 4),
    //     ("5", 5),
    //     ("6", 6),
    //     ("7", 7),
    //     ("8", 8),
    //     ("9", 9),
    // ];

    const DIGITS: [(&str, i32); 18] = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let sum: i32 = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let matches = DIGITS
                .iter()
                .flat_map(|(pattern, value)| {
                    line.match_indices(pattern)
                        .map(|(index, _)| (index, *value))
                })
                .collect::<Vec<_>>();

            let first = matches.iter().min_by_key(|(index, _)| *index).unwrap().1;
            let last = matches.iter().max_by_key(|(index, _)| *index).unwrap().1;
            first * 10 + last
        })
        .sum();

    println!("{sum}");
}
