#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

#[macro_use]
extern crate bitflags;
extern crate test;

use std::{collections::HashMap, convert::TryInto};

use clap::{crate_name, App, Arg};
use common::LineReader;

bitflags! {
    struct Transform: u8 {
        const ROTATE_90 = 1 << 0;
        const FLIP_HORIZONTAL = 1 << 1;
        const FLIP_VERTICAL = 1 << 2;
    }
}

const TILE_SIZE: usize = 10;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

#[derive(Debug)]
struct Tile {
    id: i16,
    // Stored LTRB, horizontal L->R, vertical T->B
    sides: [[u8; TILE_SIZE]; 4],
    sides_with_neighbors: Vec<Side>,
}

impl Tile {
    fn from_lines(lines: &[String]) -> Self {
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

    #[cfg(test)]
    fn from_sides(sides: [[u8; TILE_SIZE]; 4]) -> Self {
        Self {
            id: 0,
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

    fn get_transform_to_be_top_left(&self) -> Transform {
        let mut sorted_sides = self.sides_with_neighbors.clone();
        sorted_sides.sort();
        match sorted_sides.as_slice() {
            [Side::Left, Side::Top] => Transform::FLIP_HORIZONTAL | Transform::FLIP_VERTICAL,
            [Side::Top, Side::Right] => Transform::FLIP_VERTICAL,
            [Side::Right, Side::Bottom] => Transform::empty(),
            [Side::Left, Side::Bottom] => Transform::FLIP_HORIZONTAL,
            _ => panic!("Unexpected sides {:?}", sorted_sides),
        }
    }

    fn get_side_after_transform(&self, mut side: Side, transform: Transform) -> [u8; TILE_SIZE] {
        let mut reverse = false;

        if transform.contains(Transform::FLIP_VERTICAL) {
            reverse = match side {
                Side::Left | Side::Right => !reverse,
                _ => reverse,
            };
            side = match side {
                Side::Top => Side::Bottom,
                Side::Bottom => Side::Top,
                _ => side,
            };
        }

        if transform.contains(Transform::FLIP_HORIZONTAL) {
            reverse = match side {
                Side::Top | Side::Bottom => !reverse,
                _ => reverse,
            };
            side = match side {
                Side::Left => Side::Right,
                Side::Right => Side::Left,
                _ => side,
            };
        }

        if transform.contains(Transform::ROTATE_90) {
            reverse = match side {
                Side::Top | Side::Bottom => !reverse,
                _ => reverse,
            };
            side = Side::from_index((((side as u8) + 3) % 4) as usize);
        }

        let mut side_bytes = self.sides[side as usize].clone();
        if reverse {
            side_bytes.reverse();
        }
        side_bytes
    }

    fn get_transform_to_match_side(&self, side: Side, bytes: &[u8; TILE_SIZE]) -> Transform {
        for transform_bits in 0..8 {
            let transform =
                Transform::from_bits(transform_bits).expect("Failed to parse bits as Transform");
            if self.get_side_after_transform(side, transform) == *bytes {
                return transform;
            }
        }
        panic!("Failed to find a transform to match the given side");
    }
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
        let tile = Tile::from_lines(&tile_lines);
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

    for (_, tile) in &mut tiles {
        let mut sides_with_neighbors = Vec::new();
        for (i, side) in tile.sides.iter().enumerate() {
            if tiles_with_side[side].iter().any(|id| *id != tile.id) {
                sides_with_neighbors.push(Side::from_index(i));
            }
        }

        if sides_with_neighbors.len() == 2 {
            corner_product *= tile.id as u64;
            corners.push(tile.id);
        };

        tile.sides_with_neighbors = sides_with_neighbors;
    }

    println!("Corner product: {}", corner_product);

    let top_left_corner_id = corners[0];
    let top_left_tile = &tiles[&top_left_corner_id];
    let transform_to_be_top_left = top_left_tile.get_transform_to_be_top_left();

    let mut rows = Vec::new();
    let mut first_row = Vec::new();
    first_row.push((top_left_tile.id, transform_to_be_top_left));
    loop {
        let (previous_tile_id, previous_tile_transform) =
            first_row.last().expect("Failed to find previous tile");
        let previous_tile = &tiles[previous_tile_id];

        let previous_right_side =
            previous_tile.get_side_after_transform(Side::Right, *previous_tile_transform);
        if let Some(current_tile_id) = tiles_with_side[&previous_right_side]
            .iter()
            .filter(|id| **id != *previous_tile_id)
            .next()
        {
            let current_tile = &tiles[current_tile_id];
            let current_tile_transform =
                current_tile.get_transform_to_match_side(Side::Left, &previous_right_side);
            first_row.push((*current_tile_id, current_tile_transform));
        } else {
            break;
        }
    }
    rows.push(first_row);

    loop {
        let mut row = Vec::new();

        loop {
            let column_index = row.len();
            let last_row = rows.last().expect("Failed to find previous row");
            if column_index >= last_row.len() {
                break;
            }

            let (previous_tile_id, previous_tile_transform) = &last_row[column_index];
            let previous_tile = &tiles[previous_tile_id];

            let previous_bottom_side =
                previous_tile.get_side_after_transform(Side::Bottom, *previous_tile_transform);
            if let Some(current_tile_id) = tiles_with_side[&previous_bottom_side]
                .iter()
                .filter(|id| **id != *previous_tile_id)
                .next()
            {
                let current_tile = &tiles[current_tile_id];
                let current_tile_transform =
                    current_tile.get_transform_to_match_side(Side::Top, &previous_bottom_side);
                row.push((*current_tile_id, current_tile_transform));
            } else {
                break;
            }
        }

        if row.is_empty() {
            break;
        }

        rows.push(row);
    }
}

#[cfg(test)]
mod tests {
    use super::{Side, Tile, Transform, TILE_SIZE};
    // use test::Bencher;

    fn get_test_sides() -> [[u8; TILE_SIZE]; 4] {
        // 10 ... 19
        // ...   ...
        // 37 ... 28

        let left = [10, 45, 44, 43, 42, 41, 40, 39, 38, 37];
        let top = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
        let right = [19, 20, 21, 22, 23, 24, 25, 26, 27, 28];
        let bottom = [37, 36, 35, 34, 33, 32, 31, 30, 29, 28];
        [left, top, right, bottom]
    }

    #[test]
    fn side_after_no_transform() {
        let tile = Tile::from_sides(get_test_sides());

        assert_eq!(
            tile.get_side_after_transform(Side::Left, Transform::empty()),
            get_test_sides()[Side::Left as usize]
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Top, Transform::empty()),
            get_test_sides()[Side::Top as usize]
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Right, Transform::empty()),
            get_test_sides()[Side::Right as usize]
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Bottom, Transform::empty()),
            get_test_sides()[Side::Bottom as usize]
        );
    }

    #[test]
    fn side_after_rotate_90() {
        // 10 ... 19       37 ... 10
        // ...   ...  -->  ...   ...
        // 37 ... 28       28 ... 19

        let tile = Tile::from_sides(get_test_sides());

        assert_eq!(
            tile.get_side_after_transform(Side::Left, Transform::ROTATE_90),
            get_test_sides()[Side::Bottom as usize]
        );
        let mut reversed_left = get_test_sides()[Side::Left as usize];
        reversed_left.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Top, Transform::ROTATE_90),
            reversed_left
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Right, Transform::ROTATE_90),
            get_test_sides()[Side::Top as usize]
        );
        let mut reversed_right = get_test_sides()[Side::Right as usize];
        reversed_right.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Bottom, Transform::ROTATE_90),
            reversed_right
        );
    }

    #[test]
    fn side_after_flip_horizontal() {
        // 10 ... 19       19 ... 10
        // ...   ...  -->  ...   ...
        // 37 ... 28       28 ... 37

        let tile = Tile::from_sides(get_test_sides());

        assert_eq!(
            tile.get_side_after_transform(Side::Left, Transform::FLIP_HORIZONTAL),
            get_test_sides()[Side::Right as usize]
        );
        let mut reversed_top = get_test_sides()[Side::Top as usize];
        reversed_top.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Top, Transform::FLIP_HORIZONTAL),
            reversed_top
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Right, Transform::FLIP_HORIZONTAL),
            get_test_sides()[Side::Left as usize]
        );
        let mut reversed_bottom = get_test_sides()[Side::Bottom as usize];
        reversed_bottom.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Bottom, Transform::FLIP_HORIZONTAL),
            reversed_bottom
        );
    }

    #[test]
    fn side_after_flip_vertical() {
        // 10 ... 19       37 ... 28
        // ...   ...  -->  ...   ...
        // 37 ... 28       10 ... 19

        let tile = Tile::from_sides(get_test_sides());

        let mut reversed_left = get_test_sides()[Side::Left as usize];
        reversed_left.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Left, Transform::FLIP_VERTICAL),
            reversed_left
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Top, Transform::FLIP_VERTICAL),
            get_test_sides()[Side::Bottom as usize]
        );
        let mut reversed_right = get_test_sides()[Side::Right as usize];
        reversed_right.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Right, Transform::FLIP_VERTICAL),
            reversed_right
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Bottom, Transform::FLIP_VERTICAL),
            get_test_sides()[Side::Top as usize]
        );
    }

    #[test]
    fn side_after_rotate_270() {
        // 10 ... 19       19 ... 28
        // ...   ...  -->  ...   ...
        // 37 ... 28       10 ... 37

        let rotate_270 =
            Transform::ROTATE_90 | Transform::FLIP_HORIZONTAL | Transform::FLIP_VERTICAL;

        let tile = Tile::from_sides(get_test_sides());

        let mut reversed_top = get_test_sides()[Side::Top as usize];
        reversed_top.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Left, rotate_270),
            reversed_top
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Top, rotate_270),
            get_test_sides()[Side::Right as usize]
        );
        let mut reversed_bottom = get_test_sides()[Side::Bottom as usize];
        reversed_bottom.reverse();
        assert_eq!(
            tile.get_side_after_transform(Side::Right, rotate_270),
            reversed_bottom
        );
        assert_eq!(
            tile.get_side_after_transform(Side::Bottom, rotate_270),
            get_test_sides()[Side::Left as usize]
        );
    }

    #[test]
    fn get_transform_to_match_side() {
        // 10 ... 19
        // ...   ...
        // 37 ... 28

        let tile = Tile::from_sides(get_test_sides());
        let mut sides = get_test_sides();

        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Top as usize]),
            Transform::empty()
        );
        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Right as usize]),
            Transform::ROTATE_90 | Transform::FLIP_HORIZONTAL | Transform::FLIP_VERTICAL
        );
        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Bottom as usize]),
            Transform::FLIP_VERTICAL
        );
        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Left as usize]),
            Transform::ROTATE_90 | Transform::FLIP_HORIZONTAL
        );

        for side_index in 0..4 {
            sides[side_index].reverse();
        }

        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Top as usize]),
            Transform::FLIP_HORIZONTAL
        );
        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Right as usize]),
            Transform::ROTATE_90 | Transform::FLIP_VERTICAL
        );
        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Bottom as usize]),
            Transform::FLIP_HORIZONTAL | Transform::FLIP_VERTICAL
        );
        assert_eq!(
            tile.get_transform_to_match_side(Side::Top, &sides[Side::Left as usize]),
            Transform::ROTATE_90
        );
    }
}
