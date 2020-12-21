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
const IMAGE_SIZE: usize = TILE_SIZE - 2;

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
    id: u16,
    image: Vec<Vec<u8>>,
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

        let mut image = vec![vec![b' '; IMAGE_SIZE]; IMAGE_SIZE];

        let mut left = [b'*'; TILE_SIZE];
        let mut right = [b'*'; TILE_SIZE];
        for (row, line) in lines.iter().skip(1).enumerate() {
            let bytes = line.as_bytes();
            left[row] = bytes[0];
            right[row] = bytes[bytes.len() - 1];

            if (1..=IMAGE_SIZE).contains(&row) {
                image[row - 1][..IMAGE_SIZE].clone_from_slice(&bytes[1..=IMAGE_SIZE]);
            }
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
            image,
            sides,
            sides_with_neighbors: Vec::new(),
        }
    }

    #[cfg(test)]
    fn from_sides(sides: [[u8; TILE_SIZE]; 4]) -> Self {
        Self {
            id: 0,
            image: Vec::new(),
            sides,
            sides_with_neighbors: Vec::new(),
        }
    }

    fn get_unique_sides(&self) -> Vec<[u8; TILE_SIZE]> {
        let mut unique_sides = Vec::new();
        for side in &self.sides {
            unique_sides.push(*side);
            unique_sides.push(*side);
            unique_sides.last_mut().unwrap().reverse();
        }
        unique_sides.sort_unstable();
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

        let mut side_bytes = self.sides[side as usize];
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

struct TransformedTile {
    id: u16,
    transform: Transform,
}

impl TransformedTile {
    fn new(id: u16, transform: Transform) -> Self {
        Self { id, transform }
    }
}

fn assemble_tiles(
    top_left_corner_id: u16,
    tiles: &HashMap<u16, Tile>,
    tiles_with_side: &HashMap<[u8; TILE_SIZE], Vec<u16>>,
) -> Vec<Vec<TransformedTile>> {
    let mut rows = Vec::new();

    let mut first_row = Vec::new();
    first_row.push(TransformedTile::new(
        tiles[&top_left_corner_id].id,
        tiles[&top_left_corner_id].get_transform_to_be_top_left(),
    ));

    loop {
        let previous = first_row.last().expect("Failed to find previous tile");
        let previous_tile = &tiles[&previous.id];

        let previous_right_side =
            previous_tile.get_side_after_transform(Side::Right, previous.transform);
        if let Some(current_tile) = tiles_with_side[&previous_right_side].iter().find_map(|id| {
            if *id == previous_tile.id {
                None
            } else {
                Some(&tiles[id])
            }
        }) {
            let current_transform =
                current_tile.get_transform_to_match_side(Side::Left, &previous_right_side);
            first_row.push(TransformedTile::new(current_tile.id, current_transform));
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

            let previous = &last_row[column_index];
            let previous_tile = &tiles[&previous.id];

            let previous_bottom_side =
                previous_tile.get_side_after_transform(Side::Bottom, previous.transform);
            if let Some(current_tile) =
                tiles_with_side[&previous_bottom_side]
                    .iter()
                    .find_map(|id| {
                        if *id == previous_tile.id {
                            None
                        } else {
                            Some(&tiles[id])
                        }
                    })
            {
                let current_transform =
                    current_tile.get_transform_to_match_side(Side::Top, &previous_bottom_side);
                row.push(TransformedTile::new(current_tile.id, current_transform));
            } else {
                break;
            }
        }

        if row.is_empty() {
            break;
        }

        rows.push(row);
    }

    rows
}

fn transform_image(image: &[Vec<u8>], transform: Transform) -> Vec<Vec<u8>> {
    let mut result = vec![vec![b' '; image.len()]; image.len()];

    if transform.contains(Transform::ROTATE_90) {
        for (row_index, row) in result.iter_mut().enumerate() {
            for column_index in 0..image.len() {
                row[column_index] = image[image.len() - 1 - column_index][row_index];
            }
        }
    } else {
        result = Vec::from(image);
    }

    if transform.contains(Transform::FLIP_HORIZONTAL) {
        for row in &mut result {
            row.reverse();
        }
    }

    if transform.contains(Transform::FLIP_VERTICAL) {
        result.reverse();
    }

    result
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
                .or_insert_with(Vec::new)
                .push(tile.id);
        }
        tiles.insert(tile.id, tile);
        tile_lines.clear();
    }

    let mut corner_product = 1;
    let mut corners = Vec::new();

    for tile in tiles.values_mut() {
        let mut sides_with_neighbors = Vec::new();
        for (i, side) in tile.sides.iter().enumerate() {
            if tiles_with_side[side].iter().any(|id| *id != tile.id) {
                sides_with_neighbors.push(Side::from_index(i));
            }
        }

        if sides_with_neighbors.len() == 2 {
            corner_product *= u64::from(tile.id);
            corners.push(tile.id);
        };

        tile.sides_with_neighbors = sides_with_neighbors;
    }

    println!("Corner product: {}", corner_product);

    let rows = assemble_tiles(corners[0], &tiles, &tiles_with_side);

    let mut image = Vec::new();
    for row in &rows {
        let mut lines = vec![Vec::new(); TILE_SIZE - 2];
        for placed_tile in row {
            let tile = &tiles[&placed_tile.id];
            let tile_image = transform_image(&tile.image, placed_tile.transform);
            for line in 0..TILE_SIZE - 2 {
                lines[line].extend_from_slice(&tile_image[line]);
            }
        }
        image.append(&mut lines);
    }

    let pattern = [
        b"                  # ",
        b"#    ##    ##    ###",
        b" #  #  #  #  #  #   ",
    ];

    for transform_bits in 0..8 {
        let transform = Transform::from_bits(transform_bits)
            .expect("Failed to convert transform bits into Transform");

        let mut instance_count = 0;

        let transformed_image = transform_image(&image, transform);

        for origin_row in 0..image.len() - (pattern.len() - 1) {
            for origin_column in 0..image.len() - (pattern[0].len() - 1) {
                let mut all_found = true;
                for row in 0..pattern.len() {
                    for column in 0..pattern[0].len() {
                        if pattern[row][column] == b'#'
                            && transformed_image[origin_row + row][origin_column + column] != b'#'
                        {
                            all_found = false;
                            break;
                        }
                    }

                    if !all_found {
                        break;
                    }
                }

                if all_found {
                    instance_count += 1;
                }
            }
        }

        if instance_count > 0 {
            let pattern_hash_count = pattern
                .iter()
                .flat_map(|row| row.iter())
                .filter(|byte| **byte == b'#')
                .count();

            let image_hash_count = transformed_image
                .iter()
                .flat_map(|row| row.iter())
                .filter(|byte| **byte == b'#')
                .count();

            println!(
                "Water roughness: {}",
                image_hash_count - pattern_hash_count * instance_count
            );
            break;
        }
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
