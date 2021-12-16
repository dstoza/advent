#![feature(test)]
extern crate test;

use std::{
    collections::{BinaryHeap, VecDeque},
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
    while let Some((row, column)) = updates.pop_front() {
        if row > 0
            && (lowest_risk[row][column] + risk_to_enter[row - 1][column] as u16)
                < lowest_risk[row - 1][column]
        {
            lowest_risk[row - 1][column] =
                lowest_risk[row][column] + risk_to_enter[row - 1][column] as u16;
            updates.push_back((row - 1, column));
        }

        if column > 0
            && (lowest_risk[row][column] + risk_to_enter[row][column - 1] as u16)
                < lowest_risk[row][column - 1]
        {
            lowest_risk[row][column - 1] =
                lowest_risk[row][column] + risk_to_enter[row][column - 1] as u16;
            updates.push_back((row, column - 1));
        }

        if column + 1 < risk_to_enter.len()
            && (lowest_risk[row][column] + risk_to_enter[row][column + 1] as u16)
                < lowest_risk[row][column + 1]
        {
            lowest_risk[row][column + 1] =
                lowest_risk[row][column] + risk_to_enter[row][column + 1] as u16;
            updates.push_back((row, column + 1));
        }

        if row + 1 < risk_to_enter.len()
            && (lowest_risk[row][column] + risk_to_enter[row + 1][column] as u16)
                < lowest_risk[row + 1][column]
        {
            lowest_risk[row + 1][column] =
                lowest_risk[row][column] + risk_to_enter[row + 1][column] as u16;
            updates.push_back((row + 1, column));
        }
    }
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

#[derive(Debug, Eq)]
struct Node {
    previous: Vec<(u16, u16)>,
    total_estimated_risk: u16,
    risk_to_node: u16,
    row: u16,
    column: u16,
}

impl Node {
    fn has_previous(&self, row: usize, column: usize) -> bool {
        self.previous.contains(&(row as u16, column as u16))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_estimated_risk.cmp(&self.total_estimated_risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.total_estimated_risk == other.total_estimated_risk
    }
}

fn get_estimated_risk(from_row: usize, from_column: usize, to_index: usize) -> u16 {
    (to_index - from_row + to_index - from_column) as u16
}

fn push_neighbor(
    risk_to_enter: &[Vec<u8>],
    lowest_risk: &mut [Vec<u16>],
    queue: &mut BinaryHeap<Node>,
    node: &Node,
    row: usize,
    column: usize,
) {
    let risk_to_neighbor = node.risk_to_node + risk_to_enter[row][column] as u16;
    if risk_to_neighbor < lowest_risk[row][column] && !node.has_previous(row, column) {
        lowest_risk[row][column] = risk_to_neighbor;
        let previous = {
            let mut previous = node.previous.clone();
            previous.push((node.row, node.column));
            previous
        };
        queue.push(Node {
            previous,
            total_estimated_risk: risk_to_neighbor
                + get_estimated_risk(row, column, risk_to_enter.len() - 1),
            risk_to_node: risk_to_neighbor,
            row: row as u16,
            column: column as u16,
        });
    }
}

fn get_lowest_risk_a_star(risk_to_enter: &[Vec<u8>]) -> u16 {
    let to_index = risk_to_enter.len() - 1;

    let mut lowest_risk = vec![vec![u16::MAX; risk_to_enter.len()]; risk_to_enter.len()];

    let mut queue = BinaryHeap::new();
    queue.push(Node {
        previous: Vec::new(),
        total_estimated_risk: get_estimated_risk(0, 0, to_index),
        risk_to_node: 0,
        row: 0,
        column: 0,
    });

    while let Some(node) = queue.pop() {
        if node.row as usize == to_index && node.column as usize == to_index {
            return node.risk_to_node;
        }

        if node.row > 0 {
            push_neighbor(
                risk_to_enter,
                &mut lowest_risk,
                &mut queue,
                &node,
                (node.row - 1) as usize,
                node.column as usize,
            );
        }

        if node.column > 0 {
            push_neighbor(
                risk_to_enter,
                &mut lowest_risk,
                &mut queue,
                &node,
                node.row as usize,
                (node.column - 1) as usize,
            );
        }

        if (node.column as usize) < to_index {
            push_neighbor(
                risk_to_enter,
                &mut lowest_risk,
                &mut queue,
                &node,
                node.row as usize,
                (node.column + 1) as usize,
            );
        }

        if (node.row as usize) < to_index {
            push_neighbor(
                risk_to_enter,
                &mut lowest_risk,
                &mut queue,
                &node,
                (node.row + 1) as usize,
                node.column as usize,
            );
        }
    }

    unreachable!();
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let risk_to_enter = parse_input(reader.lines().map(|line| line.unwrap()));
    // println!("Lowest risk: {}", get_lowest_risk(&risk_to_enter));
    println!(
        "Lowest risk: {}",
        get_lowest_risk(&(expand_map(&risk_to_enter)))
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
    fn test_lowest_risk() {
        let risk_to_enter = parse_input(get_example().into_iter());
        assert_eq!(get_lowest_risk(&risk_to_enter), 40);
    }

    #[test]
    fn test_lowest_risk_a_star() {
        let risk_to_enter = parse_input(get_example().into_iter());
        assert_eq!(get_lowest_risk_a_star(&risk_to_enter), 40);
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
    fn test_lowest_risk_expanded() {
        let risk_to_enter = parse_input(get_example().into_iter());
        assert_eq!(get_lowest_risk(&expand_map(&risk_to_enter)), 315);
    }

    #[test]
    fn test_lowest_risk_expanded_a_star() {
        let risk_to_enter = parse_input(get_example().into_iter());
        assert_eq!(get_lowest_risk_a_star(&expand_map(&risk_to_enter)), 315);
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

    #[bench]
    fn bench_input_a_star(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        b.iter(|| {
            let risk_to_enter = parse_input(lines.clone().into_iter());
            assert_eq!(get_lowest_risk_a_star(&expand_map(&risk_to_enter)), 2814);
        })
    }
}
