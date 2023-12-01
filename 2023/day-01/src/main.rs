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

    let mut sum = 0;
    for line in reader.lines().map(std::result::Result::unwrap) {
        let mut first_index = line.len();
        let mut first_value = None;
        for (s, value) in DIGITS {
            let index = line.find(s);
            if let Some(index) = index {
                if index < first_index {
                    first_index = index;
                    first_value = Some(value);
                }
            }
        }

        let mut last_index = 0;
        let mut last_value = None;
        for (s, value) in DIGITS {
            let index = line.rfind(s);
            if let Some(index) = index {
                if index >= last_index {
                    last_index = index;
                    last_value = Some(value);
                }
            }
        }

        let value = first_value.unwrap() * 10 + last_value.unwrap();
        sum += value;
    }

    println!("{sum}");
}
