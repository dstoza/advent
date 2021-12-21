#![feature(test)]
extern crate test;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

fn parse_input<I: Iterator<Item = String>>(
    mut lines: I,
) -> ([u8; 512], HashSet<(i16, i16)>, usize, usize) {
    let algorithm: [u8; 512] = lines.next().unwrap().as_bytes().try_into().unwrap();
    // Skip the blank line
    lines.next();

    let mut width = 0;
    let mut height = 0;

    let mut light_pixels = HashSet::new();
    for (y, line) in lines.enumerate() {
        width = line.as_bytes().len();
        height = y + 1;
        for (x, byte) in line.as_bytes().iter().enumerate() {
            if *byte == b'#' {
                light_pixels.insert((x as i16, y as i16));
            }
        }
    }

    (algorithm, light_pixels, width, height)
}

fn flatten(neighborhood: [u8; 9]) -> u16 {
    let mut flattened = 0u16;
    for pixel in neighborhood {
        flattened <<= 1;
        flattened += (pixel == b'#') as u16;
    }
    flattened
}

fn run_iterations(
    algorithm: &[u8; 512],
    mut light_pixels: HashSet<(i16, i16)>,
    width: usize,
    height: usize,
    iterations: usize,
) -> HashSet<(i16, i16)> {
    const OFFSETS: [(i16, i16); 9] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut new_pixels = HashSet::new();

    let mut padding = 1;
    let mut background = b'.';
    for _ in 0..iterations {
        for y in -padding..(height as i16 + padding) {
            for x in -padding..(width as i16 + padding) {
                let neighborhood: Vec<_> = OFFSETS
                    .iter()
                    .map(|(offset_x, offset_y)| {
                        let x = x + offset_x;
                        let y = y + offset_y;
                        if x <= -padding || y <= -padding || x >= (width as i16) + padding - 1 || y >= (height as i16) + padding - 1 {
                            background
                        } else if light_pixels.contains(&(x, y)) {
                            b'#'
                        } else {
                            b'.'
                        }
                    })
                    .collect();
                let neighborhood: [u8; 9] = neighborhood.try_into().unwrap();
                let flattened = flatten(neighborhood);
                if algorithm[flattened as usize] == b'#' {
                    new_pixels.insert((x, y));
                }
            }
        }

        background = algorithm[flatten([background; 9]) as usize];

        swap(&mut light_pixels, &mut new_pixels);
        new_pixels.clear();

        padding += 1;
    }

    light_pixels
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let (algorithm, light_pixels, width, height) =
        parse_input(reader.lines().map(|line| line.unwrap()));
    println!("Lit pixels: {}", light_pixels.len());
    let light_pixels = run_iterations(&algorithm, light_pixels, width, height, 50);
    println!("Lit pixels: {}", light_pixels.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use test::Bencher;

    fn get_example() -> [String; 7] {
        let mut algorithm = String::from(
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
        );
        algorithm
            .push_str("#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###");
        algorithm
            .push_str(".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.");
        algorithm
            .push_str(".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....");
        algorithm
            .push_str(".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..");
        algorithm
            .push_str("...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....");
        algorithm.push_str("..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#");
        [
            algorithm,
            String::new(),
            String::from("#..#."),
            String::from("#...."),
            String::from("##..#"),
            String::from("..#.."),
            String::from("..###"),
        ]
    }

    #[test]
    fn test_flatten() {
        assert_eq!(flatten("...#...#.".as_bytes().try_into().unwrap()), 34);
    }

    #[test]
    fn test_iterate() {
        let (algorithm, light_pixels, width, height) = parse_input(get_example().into_iter());
        assert_eq!(run_iterations(&algorithm, light_pixels.clone(), width, height, 1).len(), 24);
        assert_eq!(run_iterations(&algorithm, light_pixels.clone(), width, height, 2).len(), 35);
    }
}
