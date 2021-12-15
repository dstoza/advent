#![feature(test)]
extern crate test;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input<I: Iterator<Item = String>>(lines: I) -> Vec<Vec<u8>> {
    lines
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

fn propagate_updates(
    risk_to_enter: &[Vec<u8>],
    lowest_risk: &mut [Vec<u16>],
    mut updates: VecDeque<(usize, usize)>,
) {
    let mut update_count = 0;

    while let Some((row, column)) = updates.pop_front() {
        if row > 0
            && (lowest_risk[row][column] + risk_to_enter[row - 1][column] as u16)
                < lowest_risk[row - 1][column]
        {
            lowest_risk[row - 1][column] =
                lowest_risk[row][column] + risk_to_enter[row - 1][column] as u16;
            update_count += 1;
            updates.push_back((row - 1, column));
        }

        if column > 0
            && (lowest_risk[row][column] + risk_to_enter[row][column - 1] as u16)
                < lowest_risk[row][column - 1]
        {
            lowest_risk[row][column - 1] =
                lowest_risk[row][column] + risk_to_enter[row][column - 1] as u16;
            update_count += 1;
            updates.push_back((row, column - 1));
        }

        if column + 1 < risk_to_enter.len()
            && (lowest_risk[row][column] + risk_to_enter[row][column + 1] as u16)
                < lowest_risk[row][column + 1]
        {
            lowest_risk[row][column + 1] =
                lowest_risk[row][column] + risk_to_enter[row][column + 1] as u16;
            update_count += 1;
            updates.push_back((row, column + 1));
        }

        if row + 1 < risk_to_enter.len()
            && (lowest_risk[row][column] + risk_to_enter[row + 1][column] as u16)
                < lowest_risk[row + 1][column]
        {
            lowest_risk[row + 1][column] =
                lowest_risk[row][column] + risk_to_enter[row + 1][column] as u16;
            update_count += 1;
            updates.push_back((row + 1, column));
        }
    }

    println!("{} Updates", update_count);
}

fn propagate_from_neighbors(risk_to_enter: &[Vec<u8>], lowest_risk: &mut [Vec<u16>]) {
    let mut updates = VecDeque::new();

    for row in 0..risk_to_enter.len() {
        for column in 0..risk_to_enter.len() {
            // Above
            if row > 0
                && (lowest_risk[row - 1][column] + risk_to_enter[row][column] as u16)
                    < lowest_risk[row][column]
            {
                lowest_risk[row][column] =
                    lowest_risk[row - 1][column] + risk_to_enter[row][column] as u16;
                updates.push_back((row, column));
                continue;
            }

            // Left
            if column > 0
                && (lowest_risk[row][column - 1] + risk_to_enter[row][column] as u16)
                    < lowest_risk[row][column]
            {
                lowest_risk[row][column] =
                    lowest_risk[row][column - 1] + risk_to_enter[row][column] as u16;
                updates.push_back((row, column));
                continue;
            }

            // Right
            if column + 1 < risk_to_enter.len()
                && (lowest_risk[row][column + 1] + risk_to_enter[row][column] as u16)
                    < lowest_risk[row][column]
            {
                lowest_risk[row][column] =
                    lowest_risk[row][column + 1] + risk_to_enter[row][column] as u16;
                updates.push_back((row, column));
                continue;
            }

            // Below
            if row + 1 < risk_to_enter.len()
                && (lowest_risk[row + 1][column] + risk_to_enter[row][column] as u16)
                    < lowest_risk[row][column]
            {
                lowest_risk[row][column] =
                    lowest_risk[row + 1][column] + risk_to_enter[row][column] as u16;
                updates.push_back((row, column));
                continue;
            }
        }
    }

    propagate_updates(risk_to_enter, lowest_risk, updates);
}

fn get_lowest_risk(risk_to_enter: &[Vec<u8>]) -> u16 {
    let mut lowest_risk = vec![vec![0; risk_to_enter[0].len()]; risk_to_enter.len()];

    // Do a quick, approximate initial propagation
    for row in 0..risk_to_enter.len() {
        for column in 0..risk_to_enter[0].len() {
            if row == 0 && column == 0 {
                continue;
            }

            let left = if column > 0 {
                lowest_risk[row][column - 1] + risk_to_enter[row][column] as u16
            } else {
                u16::MAX
            };
            let above = if row > 0 {
                lowest_risk[row - 1][column] + risk_to_enter[row][column] as u16
            } else {
                u16::MAX
            };

            lowest_risk[row][column] = left.min(above);
        }
    }

    propagate_from_neighbors(risk_to_enter, &mut lowest_risk);

    lowest_risk[lowest_risk.len() - 1][lowest_risk[0].len() - 1]
}

fn expand_map(risk_to_enter: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let horizontal_expansions: Vec<_> = risk_to_enter
        .iter()
        .map(|line| {
            let mut expanded_line = Vec::new();
            for instance in 0..5 {
                expanded_line.extend(line.iter().map(|value| (*value - 1 + instance) % 9 + 1));
            }
            expanded_line
        })
        .collect();

    (0..5)
        .flat_map(|instance| {
            horizontal_expansions.iter().map(move |expansion| {
                expansion
                    .iter()
                    .map(|value| (*value - 1 + instance) % 9 + 1)
                    .collect()
            })
        })
        .collect()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let risk_to_enter = parse_input(reader.lines().map(|line| line.unwrap()));
    // println!("Lowest risk: {}", get_lowest_risk(&risk_to_enter));
    println!(
        "Lowest risk: {}",
        get_lowest_risk(&expand_map(&risk_to_enter))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_example() -> [String; 10] {
        [
            String::from("1163751742"),
            String::from("1381373672"),
            String::from("2136511328"),
            String::from("3694931569"),
            String::from("7463417111"),
            String::from("1319128137"),
            String::from("1359912421"),
            String::from("3125421639"),
            String::from("1293138521"),
            String::from("2311944581"),
        ]
    }

    #[test]
    fn test_parse_example() {
        let rows = parse_input(get_example().into_iter());
        assert_eq!(rows[0], vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2]);
        assert_eq!(rows[9], vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1]);
    }

    #[test]
    fn test_lowest_cost() {
        let risk_to_enter = parse_input(get_example().into_iter());
        assert_eq!(get_lowest_risk(&risk_to_enter), 40);
    }

    #[test]
    fn test_expand_map() {
        let map = vec![vec![8u8]];
        let expanded = expand_map(&map);
        assert_eq!(expanded.len(), 5);
        assert_eq!(expanded[0], vec![8, 9, 1, 2, 3]);
        assert_eq!(expanded[4], vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_expand_example() {
        let expanded = expand_map(&parse_input(get_example().into_iter()));
        assert_eq!(
            expanded[0],
            vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9,
                6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6
            ]
        );
        assert_eq!(
            expanded[49],
            vec![
                6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2,
                5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8, 1, 2, 9, 9, 8, 3, 3, 4, 7, 9
            ]
        )
    }

    #[test]
    fn test_lowest_expanded_cost() {
        let risk_to_enter = parse_input(get_example().into_iter());
        assert_eq!(get_lowest_risk(&expand_map(&risk_to_enter)), 315);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        b.iter(|| {
            let risk_to_enter = parse_input(lines.clone().into_iter());
            assert_eq!(get_lowest_risk(&expand_map(&risk_to_enter)), 2814);
        })
    }
}
