#![warn(clippy::pedantic)]
#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn step_east(grid: &mut [Vec<u8>]) -> bool {
    let mut changed = false;

    for row in grid {
        let last_index = row.len() - 1;
        let last_column_had_mover = row[last_index] == b'>';
        let first_column_was_empty = row[0] == b'.';

        let mut column_index = 0usize;
        while column_index < row.len() - 1 {
            if row[column_index] == b'>' && row[column_index + 1] == b'.' {
                row.swap(column_index, column_index + 1);
                column_index += 2;
                changed = true;
            } else {
                column_index += 1;
            }
        }

        if last_column_had_mover && first_column_was_empty {
            row.swap(0, last_index);
            changed = true;
        }
    }

    changed
}

fn step_south(grid: &mut [Vec<u8>]) -> bool {
    let mut changed = false;

    for column_index in 0..grid[0].len() {
        let last_index = grid.len() - 1;
        let first_row_was_empty = grid[0][column_index] == b'.';
        let last_row_had_mover = grid[last_index][column_index] == b'v';

        let mut row_index = 0usize;
        while row_index < grid.len() - 1 {
            if grid[row_index][column_index] == b'v' && grid[row_index + 1][column_index] == b'.' {
                grid[row_index][column_index] = b'.';
                grid[row_index + 1][column_index] = b'v';
                row_index += 2;
                changed = true;
            } else {
                row_index += 1;
            }
        }

        if last_row_had_mover && first_row_was_empty {
            grid[last_index][column_index] = b'.';
            grid[0][column_index] = b'v';
            changed = true;
        }
    }

    changed
}

fn step(grid: &mut [Vec<u8>]) -> bool {
    let mut changed = step_east(grid);
    changed |= step_south(grid);
    changed
}

fn count_until_stop(grid: &mut [Vec<u8>]) -> usize {
    let mut count = 0;
    while step(grid) {
        count += 1;
    }
    count + 1
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    println!(
        "Steps: {}",
        count_until_stop(
            &mut reader
                .lines()
                .map(|line| line.unwrap().into_bytes())
                .collect::<Vec<_>>(),
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    use test::Bencher;

    #[test]
    fn test_step_east() {
        let mut grid = vec![vec![b'>', b'>', b'.', b'.', b'>']];
        step_east(&mut grid);
        assert_eq!(grid[0], vec![b'>', b'.', b'>', b'.', b'>']);
        step_east(&mut grid);
        assert_eq!(grid[0], vec![b'.', b'>', b'.', b'>', b'>']);
        step_east(&mut grid);
        assert_eq!(grid[0], vec![b'>', b'.', b'>', b'>', b'.']);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let grid = &mut reader
            .lines()
            .map(|line| line.unwrap().into_bytes())
            .collect::<Vec<_>>();

        b.iter(|| assert_eq!(count_until_stop(&mut grid.clone()), 321))
    }
}
