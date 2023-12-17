#![warn(clippy::pedantic)]

use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(self) -> Self {
        ((self as u8 + 3) % 4).into()
    }

    fn right(self) -> Self {
        ((self as u8 + 1) % 4).into()
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Location {
    row: usize,
    column: usize,
    direction: Direction,
    straight_remaining: usize,
}

impl Location {
    fn new(row: usize, column: usize, direction: Direction, straight_remaining: usize) -> Self {
        Self {
            row,
            column,
            direction,
            straight_remaining,
        }
    }

    fn get_value(&self, grid: &[Vec<u16>]) -> usize {
        usize::from(grid[self.row][self.column])
    }

    fn step(&mut self) {
        match self.direction {
            Direction::North => self.row -= 1,
            Direction::East => self.column += 1,
            Direction::South => self.row += 1,
            Direction::West => self.column -= 1,
        }
    }

    fn turn_left(self) -> Self {
        let mut left = self;
        left.direction = left.direction.left();
        left.step();
        left.straight_remaining = 2;
        left
    }

    fn turn_right(self) -> Self {
        let mut right = self;
        right.direction = right.direction.right();
        right.step();
        right.straight_remaining = 2;
        right
    }

    fn go_straight(mut self) -> Option<Self> {
        if self.straight_remaining > 0 {
            self.step();
            self.straight_remaining -= 1;
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq)]
struct SearchNode {
    location: Location,
    current_loss: usize,
    estimated_loss: usize,
}

impl SearchNode {
    fn new(location: Location, current_loss: usize, estimated_loss: usize) -> Self {
        Self {
            location,
            current_loss,
            estimated_loss,
        }
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_loss.cmp(&self.estimated_loss)
    }
}

fn find_least_loss(losses: &[Vec<u16>]) -> usize {
    let mut heuristic = vec![vec![0u16; losses[0].len()]; losses.len()];
    for row in (1..heuristic.len() - 1).rev() {
        for column in (1..heuristic[0].len() - 1).rev() {
            if heuristic[row + 1][column] == 0 {
                heuristic[row][column] += heuristic[row][column + 1];
            } else if heuristic[row][column + 1] == 0 {
                heuristic[row][column] += heuristic[row + 1][column];
            } else {
                heuristic[row][column] +=
                    heuristic[row][column + 1].min(heuristic[row + 1][column]);
            }
        }
    }

    let mut queue = BinaryHeap::from([
        SearchNode::new(
            Location::new(1, 2, Direction::East, 2),
            usize::from(losses[1][2]),
            usize::from(losses[1][2] + heuristic[1][2]),
        ),
        SearchNode::new(
            Location::new(2, 1, Direction::South, 2),
            usize::from(losses[2][1]),
            usize::from(losses[2][1] + heuristic[2][1]),
        ),
    ]);

    let mut best = HashMap::new();

    while let Some(node) = queue.pop() {
        let location = node.location;
        if location.row == losses.len() - 2 && location.column == losses[0].len() - 2 {
            return node.current_loss;
        }

        let best_loss = best
            .get(&location)
            .copied()
            .unwrap_or(node.current_loss + 1);
        if best_loss <= node.current_loss {
            continue;
        }
        best.insert(location, node.current_loss);

        let loss = location.get_value(losses);
        if loss == 0 {
            continue;
        }

        let left = location.turn_left();
        let left_loss = left.get_value(losses);
        if left_loss != 0 {
            let current_loss = node.current_loss + left_loss;
            let estimated_loss = current_loss + left.get_value(&heuristic);
            queue.push(SearchNode::new(left, current_loss, estimated_loss));
        }

        let right = location.turn_right();
        let right_loss = right.get_value(losses);
        if right_loss != 0 {
            let current_loss = node.current_loss + right_loss;
            let estimated_loss = current_loss + right.get_value(&heuristic);
            queue.push(SearchNode::new(right, current_loss, estimated_loss));
        }

        let Some(straight) = location.go_straight() else {
            continue;
        };
        let straight_loss = straight.get_value(losses);
        if straight_loss != 0 {
            let current_loss = node.current_loss + straight_loss;
            let estimated_loss = current_loss + straight.get_value(&heuristic);
            queue.push(SearchNode::new(straight, current_loss, estimated_loss));
        }
    }

    0
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut padding = None;
    let mut losses = reader
        .lines()
        .map(std::result::Result::unwrap)
        .flat_map(|line| {
            let mut lines = vec![[0]
                .iter()
                .copied()
                .chain(line.as_bytes().iter().map(|b| u16::from(*b - b'0')))
                .chain([0].iter().copied())
                .collect::<Vec<_>>()];

            if padding.is_none() {
                let pad = vec![0; line.len() + 2];
                lines.insert(0, pad.clone());
                padding = Some(pad);
            }

            lines
        })
        .collect::<Vec<_>>();
    losses.push(padding.unwrap());

    let least_loss = find_least_loss(&losses);
    println!("{least_loss}");
}
