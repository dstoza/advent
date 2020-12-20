#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use std::{collections::HashMap, convert::TryInto};

use clap::{crate_name, App, Arg};
use common::LineReader;

const TILE_SIZE: usize = 10;

#[derive(Debug)]
enum Side {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
}

impl Side {
    fn from_index(i: usize) -> Self {
        match i {
            0 => Self::Left,
            1 => Self::Top,
            2 => Self::Right,
            3 => Self::Bottom,
            _ => panic!("Unexpected index {}", i),
        }
    }
}

enum Transformation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipH,
    Rotate90FlipH,
    Rotate180FlipH,
    Rotate270FlipH,
}

#[derive(Debug)]
struct Tile {
    id: i16,
    // Stored LTRB, horizontal L->R, vertical T->B
    sides: [[u8; TILE_SIZE]; 4],
    sides_with_neighbors: Vec<Side>,
}

impl Tile {
    fn new(lines: &[String]) -> Self {
        let id = lines[0]
            .split(' ')
            .nth(1)
            .expect("Failed to find ID in split")
            .trim_end_matches(':')
            .parse()
            .expect("Failed to parse ID as i16");

        let mut left = [b'*'; TILE_SIZE];
        let mut right = [b'*'; TILE_SIZE];
        for (i, line) in lines.iter().skip(1).enumerate() {
            let bytes = line.as_bytes();
            left[i] = bytes[0];
            right[i] = bytes[bytes.len() - 1];
        }

        let top = lines[1]
            .as_bytes()
            .try_into()
            .expect("Failed to pack top row into byte array");
        let bottom = lines[lines.len() - 1]
            .as_bytes()
            .try_into()
            .expect("Failed to pack bottom row into byte array");

        let sides = [left, top, right, bottom];

        Self {
            id,
            sides,
            sides_with_neighbors: Vec::new(),
        }
    }

    fn get_unique_sides(&self) -> Vec<[u8; TILE_SIZE]> {
        let mut unique_sides = Vec::new();
        for side in &self.sides {
            unique_sides.push(side.clone());
            unique_sides.push(side.clone());
            unique_sides.last_mut().unwrap().reverse();
        }
        unique_sides.sort();
        unique_sides.dedup();
        unique_sides
    }

    fn get_side_after_transformation(
        &self,
        transformation: Transformation,
        side: Side,
    ) -> [u8; TILE_SIZE] {
        let (source, reverse) = match side {
            Side::Left => match transformation {
                Transformation::None => (Side::Left, false),
                Transformation::Rotate90 => (Side::Bottom, false),
                Transformation::Rotate180 => (Side::Right, true),
                Transformation::Rotate270 => (Side::Top, true),
                Transformation::FlipH => (Side::Right, false),
                Transformation::Rotate90FlipH => (Side::Top, false),
                Transformation::Rotate180FlipH => (Side::Left, true),
                Transformation::Rotate270FlipH => (Side::Bottom, true),
            },
            Side::Top => match transformation {
                Transformation::None => (Side::Top, false),
                Transformation::Rotate90 => (Side::Left, true),
                Transformation::Rotate180 => (Side::Bottom, true),
                Transformation::Rotate270 => (Side::Right, false),
                Transformation::FlipH => (Side::Top, true),
                Transformation::Rotate90FlipH => (Side::Left, false),
                Transformation::Rotate180FlipH => (Side::Bottom, false),
                Transformation::Rotate270FlipH => (Side::Right, true),
            },
            Side::Right => match transformation {
                Transformation::None => (Side::Right, false),
                Transformation::Rotate90 => (Side::Top, false),
                Transformation::Rotate180 => (Side::Left, true),
                Transformation::Rotate270 => (Side::Bottom, true),
                Transformation::FlipH => (Side::Left, false),
                Transformation::Rotate90FlipH => (Side::Bottom, false),
                Transformation::Rotate180FlipH => (Side::Right, true),
                Transformation::Rotate270FlipH => (Side::Top, true),
            },
            Side::Bottom => match transformation {
                Transformation::None => (Side::Bottom, false),
                Transformation::Rotate90 => (Side::Right, true),
                Transformation::Rotate180 => (Side::Top, true),
                Transformation::Rotate270 => (Side::Left, false),
                Transformation::FlipH => (Side::Bottom, true),
                Transformation::Rotate90FlipH => (Side::Right, false),
                Transformation::Rotate180FlipH => (Side::Top, false),
                Transformation::Rotate270FlipH => (Side::Left, true),
            },
        };

        let mut source_data = self.sides[source as usize].clone();
        if reverse {
            source_data.reverse();
        }
        source_data
    }

    /*
    fn get_transformations_matching_constraints(&self, Vec<>) -> Vec<Transformation> {

    }
    */
}

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<FILE>"))
        .get_matches();

    let mut tiles = HashMap::new();
    let mut tiles_with_side = HashMap::new();

    let mut reader = LineReader::new(args.value_of("FILE").unwrap());

    let mut tile_lines = Vec::new();
    while reader.read_with(|line| tile_lines.push(String::from(line))) {
        let tile = Tile::new(&tile_lines);
        for side in tile.get_unique_sides() {
            tiles_with_side
                .entry(side)
                .or_insert(Vec::new())
                .push(tile.id);
        }
        tiles.insert(tile.id, tile);
        tile_lines.clear();
    }

    let mut corner_product = 1;
    let mut corners = Vec::new();
    let mut edges = Vec::new();
    let mut middles = Vec::new();

    for (_, tile) in &mut tiles {
        let mut sides_with_neighbors = Vec::new();
        for (i, side) in tile.sides.iter().enumerate() {
            if tiles_with_side[side].iter().any(|id| *id != tile.id) {
                sides_with_neighbors.push(Side::from_index(i));
            }
        }

        match sides_with_neighbors.len() {
            2 => {
                corner_product *= tile.id as u64;
                corners.push(tile.id);
            }
            3 => {
                edges.push(tile.id);
            }
            4 => {
                middles.push(tile.id);
            }
            _ => panic!(
                "Unexpected number of sides with neighbors: {}",
                sides_with_neighbors.len()
            ),
        };

        tile.sides_with_neighbors = sides_with_neighbors;
    }

    println!("Corner product: {}", corner_product);
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
