#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn flood_fill(grid: &mut [Vec<u8>], row: usize, column: usize, value: u8) {
    let mut stack = vec![(row, column)];
    while let Some((row, column)) = stack.pop() {
        #[allow(clippy::cast_sign_loss)]
        for (row_offset, column_offset) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let row = match row_offset {
                0 | 1 => row + row_offset as usize,
                -1 => {
                    if row == 0 {
                        continue;
                    }
                    row - 1
                }
                _ => unreachable!(),
            };

            let column = match column_offset {
                0 | 1 => column + column_offset as usize,
                -1 => {
                    if column == 0 {
                        continue;
                    }
                    column - 1
                }
                _ => unreachable!(),
            };

            if (0..grid.len()).contains(&row)
                && (0..grid[0].len()).contains(&column)
                && grid[row][column] == b'.'
            {
                grid[row][column] = value;
                stack.push((row, column));
            }
        }
    }
}

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

    let mut grid = vec![vec![b'.'; right + 2]; bottom + 2];
    for (r, c) in &path {
        grid[*r][*c] = b'#';
    }

    flood_fill(&mut grid, 1, 1, b'o');

    let (interior_row, interior_column, _) = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(column, value)| (row, column, *value))
        })
        .find(|(_, _, value)| *value == b'.')
        .unwrap();

    flood_fill(&mut grid, interior_row, interior_column, b'#');

    let empty_count: usize = grid.iter().map(|line| bytecount::count(line, b'.')).sum();
    assert_eq!(empty_count, 0);

    let capacity: usize = grid.iter().map(|line| bytecount::count(line, b'#')).sum();
    println!("{capacity}");
}
