#![warn(clippy::pedantic)]
use std::{
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn visible_from_left(row: usize, column: usize, grid: &[Vec<u8>]) -> bool {
    for c in (0..column).rev() {
        if grid[row][c] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn visible_from_right(row: usize, column: usize, grid: &[Vec<u8>]) -> bool {
    for c in column + 1..grid[row].len() {
        if grid[row][c] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn visible_from_top(row: usize, column: usize, grid: &[Vec<u8>]) -> bool {
    for r in (0..row).rev() {
        if grid[r][column] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn visible_from_bottom(row: usize, column: usize, grid: &[Vec<u8>]) -> bool {
    for r in row + 1..grid.len() {
        if grid[r][column] >= grid[row][column] {
            return false;
        }
    }
    true
}

fn visible(row: usize, column: usize, grid: &[Vec<u8>]) -> bool {
    visible_from_left(row, column, grid)
        || visible_from_right(row, column, grid)
        || visible_from_top(row, column, grid)
        || visible_from_bottom(row, column, grid)
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

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file = std::fs::File::open(&filename)
        .unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);

    let grid: Vec<Vec<_>> = reader
        .lines()
        .map(|line| line.unwrap().bytes().map(|c| c - b'0').collect())
        .collect();

    let mut visible_trees = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if visible(row, column, &grid) {
                visible_trees += 1;
            }
        }
    }

    println!("{visible_trees} visible trees");

    let mut max_scenic_score = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            max_scenic_score = max_scenic_score.max(scenic_score(row, column, &grid));
        }
    }

    println!("Max scenic score {max_scenic_score}");
}
