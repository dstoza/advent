#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut row = usize::MAX / 2;
    let mut column = usize::MAX / 2;
    let mut path = vec![(row, column)];
    for line in reader.lines().map(std::result::Result::unwrap) {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let distance: usize = split
            .next()
            .and_then(|distance| distance.parse().ok())
            .unwrap();
        for _ in 0..distance {
            match direction {
                "R" => column += 1,
                "D" => row += 1,
                "L" => column -= 1,
                "U" => row -= 1,
                _ => (),
            }
            path.push((row, column));
        }
    }

    let top = *path.iter().map(|(row, _)| row).min().unwrap();
    let left = *path.iter().map(|(_, column)| column).min().unwrap();
    let path = path
        .into_iter()
        .map(|(row, column)| (row - top + 1, column - left + 1))
        .collect::<Vec<_>>();

    let bottom = *path.iter().map(|(row, _)| row).max().unwrap();
    let right = *path.iter().map(|(_, column)| column).max().unwrap();

    for row in 0..=bottom + 1 {
        for column in 0..=right + 1 {
            if path.iter().any(|(r, c)| *r == row && *c == column) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
