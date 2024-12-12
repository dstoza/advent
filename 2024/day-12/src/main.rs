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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

fn get_edges(region: &HashSet<(usize, usize)>) -> Vec<(Side, (usize, usize))> {
    let mut edges = Vec::new();
    for (row, column) in region {
        if !region.contains(&(*row - 1, *column)) {
            edges.push((Side::Top, (*row, *column)));
        }
        if !region.contains(&(*row, *column + 1)) {
            edges.push((Side::Right, (*row, *column)));
        }
        if !region.contains(&(*row + 1, *column)) {
            edges.push((Side::Bottom, (*row, *column)));
        }
        if !region.contains(&(*row, *column - 1)) {
            edges.push((Side::Left, (*row, *column)));
        }
    }

    edges
}

fn count_sides(mut edges: Vec<(Side, (usize, usize))>) -> usize {
    let mut sides = 0;
    while let Some((side, (row, column))) = edges.pop() {
        let mut related = match side {
            Side::Top | Side::Bottom => {
                let mut related = vec![column];
                edges.retain(|edge| {
                    let (s, (r, c)) = edge;
                    if *s != side || *r != row {
                        return true;
                    }

                    related.push(*c);
                    false
                });

                related
            }
            Side::Left | Side::Right => {
                let mut related = vec![row];
                edges.retain(|edge| {
                    let (s, (r, c)) = edge;
                    if *s != side || *c != column {
                        return true;
                    }

                    related.push(*r);
                    false
                });

                related
            }
        };

        related.sort_unstable();

        sides += 1;
        for window in related.windows(2) {
            if window[1] > window[0] + 1 {
                sides += 1;
            }
        }
    }

    sides
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let grid = parse_padded_grid(reader.lines().map(Result::unwrap));
    let regions = get_regions(grid);
    let mut price = 0;
    for region in regions {
        let edges = get_edges(&region);
        let sides = if args.part == 1 {
            edges.len()
        } else {
            count_sides(edges)
        };

        price += region.len() * sides;
    }

    println!("{price}");
}
