#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
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

fn parse_padded_grid(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for line in lines {
        let mut bytes = vec![b'*'];
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'*');

        if grid.is_empty() {
            grid.push(vec![b'*'; bytes.len()]);
        }

        grid.push(bytes);
    }

    grid.push(grid[0].clone());

    grid
}

fn neighbors(row: usize, column: usize) -> [(usize, usize); 4] {
    [
        (row - 1, column),
        (row, column + 1),
        (row + 1, column),
        (row, column - 1),
    ]
}

fn get_region(grid: &[Vec<u8>], start_row: usize, start_column: usize) -> HashSet<(usize, usize)> {
    let name = grid[start_row][start_column];
    let mut plots = HashSet::new();
    let mut stack = vec![(start_row, start_column)];
    while let Some((row, column)) = stack.pop() {
        plots.insert((row, column));
        for (neighbor_row, neighbor_column) in neighbors(row, column) {
            if grid[neighbor_row][neighbor_column] == name
                && !plots.contains(&(neighbor_row, neighbor_column))
            {
                stack.push((neighbor_row, neighbor_column));
            }
        }
    }

    plots
}

fn get_regions(mut grid: Vec<Vec<u8>>) -> Vec<HashSet<(usize, usize)>> {
    let mut regions = Vec::new();
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if grid[row][column] != b'*' {
                let region = get_region(&grid, row, column);
                for (r, c) in &region {
                    grid[*r][*c] = b'*';
                }
                regions.push(region);
            }
        }
    }

    regions
}

fn region_perimeter(region: &HashSet<(usize, usize)>) -> usize {
    region
        .iter()
        .map(|(row, column)| {
            neighbors(*row, *column)
                .iter()
                .filter(|neighbor| !region.contains(*neighbor))
                .count()
        })
        .sum()
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    println!("running part {}", args.part);

    let grid = parse_padded_grid(reader.lines().map(Result::unwrap));
    let regions = get_regions(grid);
    let mut price = 0;
    for region in regions {
        price += region.len() * region_perimeter(&region);
    }

    println!("{price}");
}
