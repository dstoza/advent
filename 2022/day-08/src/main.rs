#![warn(clippy::pedantic)]
#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn parse_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    lines
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn visible(row: usize, column: usize, grid: &[Vec<u8>]) -> bool {
    (0..column).rev().all(|c| grid[row][c] < grid[row][column])
        || (column + 1..grid[row].len()).all(|c| grid[row][c] < grid[row][column])
        || (0..row).rev().all(|r| grid[r][column] < grid[row][column])
        || (row + 1..grid.len()).all(|r| grid[r][column] < grid[row][column])
}

fn count_visible_trees(grid: &[Vec<u8>]) -> usize {
    let mut visible_trees = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if visible(row, column, grid) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn viewing_distance_left(row: usize, column: usize, grid: &[Vec<u8>]) -> usize {
    let mut distance = 0;
    for c in (0..column).rev() {
        distance += 1;
        if grid[row][c] >= grid[row][column] {
            break;
        }
    }
    distance
}

fn viewing_distance_right(row: usize, column: usize, grid: &[Vec<u8>]) -> usize {
    let mut distance = 0;
    for c in column + 1..grid[row].len() {
        distance += 1;
        if grid[row][c] >= grid[row][column] {
            break;
        }
    }
    distance
}

fn viewing_distance_up(row: usize, column: usize, grid: &[Vec<u8>]) -> usize {
    let mut distance = 0;
    for r in (0..row).rev() {
        distance += 1;
        if grid[r][column] >= grid[row][column] {
            break;
        }
    }
    distance
}

fn viewing_distance_down(row: usize, column: usize, grid: &[Vec<u8>]) -> usize {
    let mut distance = 0;
    for r in row + 1..grid.len() {
        distance += 1;
        if grid[r][column] >= grid[row][column] {
            break;
        }
    }
    distance
}

fn scenic_score(row: usize, column: usize, grid: &[Vec<u8>]) -> usize {
    viewing_distance_left(row, column, grid)
        * viewing_distance_right(row, column, grid)
        * viewing_distance_up(row, column, grid)
        * viewing_distance_down(row, column, grid)
}

fn max_scenic_score(grid: &[Vec<u8>]) -> usize {
    let mut max_scenic_score = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            max_scenic_score = max_scenic_score.max(scenic_score(row, column, grid));
        }
    }
    max_scenic_score
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);

    let grid = parse_grid(reader.lines().map(std::result::Result::unwrap));

    let visible_trees = count_visible_trees(&grid);
    println!("{visible_trees} visible trees");

    let scenic_score = max_scenic_score(&grid);
    println!("Max scenic score {scenic_score}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_parse_grid(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().map(std::result::Result::unwrap).collect();

        b.iter(|| {
            let grid = parse_grid(lines.clone().into_iter());
            assert_eq!(grid.len(), 99);
        });
    }

    #[bench]
    fn bench_count_visible_trees(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);

        let grid = parse_grid(reader.lines().map(std::result::Result::unwrap));

        b.iter(|| {
            let visible_trees = count_visible_trees(&grid);
            assert_eq!(visible_trees, 1823);
        });
    }

    #[bench]
    fn bench_max_scenic_score(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);

        let grid = parse_grid(reader.lines().map(std::result::Result::unwrap));

        b.iter(|| {
            let scenic_score = max_scenic_score(&grid);
            assert_eq!(scenic_score, 211_680);
        });
    }
}
