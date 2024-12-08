#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

fn compute_antinodes(
    antennas: &[(i32, i32)],
    width: i32,
    height: i32,
    resonance: bool,
) -> Vec<(i32, i32)> {
    if antennas.len() <= 1 {
        return Vec::new();
    }

    let mut antinodes = Vec::new();
    let source = antennas[0];
    for receiver in &antennas[1..] {
        let row_delta = receiver.0 - source.0;
        let column_delta = receiver.1 - source.1;

        if resonance {
            antinodes.push(source);
            antinodes.push(*receiver);
        }

        let mut antinode = (source.0 - row_delta, source.1 - column_delta);
        while (0..width).contains(&antinode.1) && (0..height).contains(&antinode.0) {
            antinodes.push(antinode);

            if !resonance {
                break;
            }

            antinode = (antinode.0 - row_delta, antinode.1 - column_delta);
        }

        let mut antinode = (receiver.0 + row_delta, receiver.1 + column_delta);
        while (0..width).contains(&antinode.1) && (0..height).contains(&antinode.0) {
            antinodes.push(antinode);

            if !resonance {
                break;
            }

            antinode = (antinode.0 + row_delta, antinode.1 + column_delta);
        }
    }

    antinodes.extend_from_slice(&compute_antinodes(&antennas[1..], width, height, resonance));

    antinodes
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut width = 0;
    let mut height = 0;

    let mut antennas = HashMap::new();
    for (row, line) in reader.lines().map(Result::unwrap).enumerate() {
        width = line.len() as i32;
        height += 1;
        for (column, cell) in line.as_bytes().iter().enumerate() {
            if *cell != b'.' {
                antennas
                    .entry(*cell)
                    .and_modify(|locations: &mut Vec<_>| {
                        locations.push((row as i32, column as i32));
                    })
                    .or_insert_with(|| vec![(row as i32, column as i32)]);
            }
        }
    }

    let mut antinodes = Vec::new();
    for group in antennas.values() {
        antinodes.extend_from_slice(&compute_antinodes(group, width, height, args.part == 2));
    }

    let antinodes_in_map = antinodes.iter().collect::<HashSet<_>>().len();
    println!("{antinodes_in_map}");
}
