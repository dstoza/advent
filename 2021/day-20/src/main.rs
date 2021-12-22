#![feature(test)]
extern crate test;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

fn parse_input<I: Iterator<Item = String>>(mut lines: I) -> ([u8; 512], VecDeque<VecDeque<u8>>) {
    let algorithm: [u8; 512] = lines.next().unwrap().as_bytes().try_into().unwrap();
    // Skip the blank line
    lines.next();

    let pixels: VecDeque<_> = lines
        .map(|line| line.as_bytes().iter().cloned().collect())
        .collect();

    (algorithm, pixels)
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
    mut current: VecDeque<VecDeque<u8>>,
    iterations: usize,
) -> usize {
    const OFFSETS: [(isize, isize); 9] = [
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
    let mut background = b'.';

    // Pad input with background
    // We pad it two deep, but we'll strip part of it later to update the background color
    current.push_front(VecDeque::from(vec![background; current[0].len()]));
    current.push_front(VecDeque::from(vec![background; current[0].len()]));
    current.push_back(VecDeque::from(vec![background; current[0].len()]));
    current.push_back(VecDeque::from(vec![background; current[0].len()]));

    for line in &mut current {
        line.push_front(background);
        line.push_front(background);
        line.push_back(background);
        line.push_back(background);
    }

    let mut next = current.clone();
    assert_eq!(next.len(), current.len());
    assert_eq!(next[0].len(), current[0].len());

    for _ in 0..iterations {
        for x in 1..current[0].len() - 1 {
            let mut previous_flattened = None;
            for y in 1..current.len() - 1 {
                let accumulator = if let Some(previous) = previous_flattened {
                    previous & 0x3F
                } else {
                    0
                };

                let start = if previous_flattened.is_some() { 6 } else { 0 };

                let flattened = OFFSETS[start..].iter().fold(
                    accumulator,
                    |accumulator, (offset_x, offset_y)| {
                        let x = (x as isize + offset_x) as usize;
                        let y = (y as isize + offset_y) as usize;
                        (accumulator << 1) + (current[y][x] == b'#') as u16
                    },
                );
                next[y][x] = algorithm[flattened as usize];

                previous_flattened = Some(flattened);
            }
        }

        background = algorithm[flatten([background; 9]) as usize];

        // Strip 1 level of padding
        for line in &mut next {
            line.pop_front();
            line.pop_back();
        }
        next.pop_front();
        next.pop_back();

        // Add 2 levels of padding with the new background
        next.push_front(VecDeque::from(vec![background; next[0].len()]));
        next.push_front(VecDeque::from(vec![background; next[0].len()]));
        next.push_back(VecDeque::from(vec![background; next[0].len()]));
        next.push_back(VecDeque::from(vec![background; next[0].len()]));
        for line in &mut next {
            line.push_front(background);
            line.push_front(background);
            line.push_back(background);
            line.push_back(background);
        }

        swap(&mut current, &mut next);
        next = current.clone();
    }

    // Strip padding
    current.pop_front();
    current.pop_front();
    current.pop_back();
    current.pop_back();
    for line in &mut current {
        line.pop_front();
        line.pop_front();
        line.pop_back();
        line.pop_back();
    }
    current
        .iter()
        .map(|line| line.iter().filter(|byte| **byte == b'#').count())
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let (algorithm, pixels) = parse_input(reader.lines().map(|line| line.unwrap()));
    println!("Lit pixels: {}", run_iterations(&algorithm, pixels, 50));
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let (algorithm, pixels) = parse_input(get_example().into_iter());
        assert_eq!(run_iterations(&algorithm, pixels.clone(), 1), 24);
        assert_eq!(run_iterations(&algorithm, pixels.clone(), 2), 35);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        b.iter(|| {
            let (algorithm, pixels) = parse_input(lines.clone().into_iter());
            assert_eq!(run_iterations(&algorithm, pixels, 50), 12333);
        });
    }
}
