#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

struct Node {
    previous: usize,
    next: usize,
    value: i64,
}

struct Ring {
    nodes: Vec<Node>,
}

impl Ring {
    fn new(values: &[i64]) -> Self {
        let mut nodes: Vec<_> = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let previous = if index > 0 { index - 1 } else { 0 };
                let next = index + 1;
                Node {
                    previous,
                    next,
                    value: *value,
                }
            })
            .collect();

        nodes.first_mut().unwrap().previous = nodes.len() - 1;
        nodes.last_mut().unwrap().next = 0;

        Self { nodes }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn move_node(&mut self, index: usize) {
        let value = self.nodes[index].value;
        let current_previous = self.nodes[index].previous;
        let current_next = self.nodes[index].next;

        // Wrap modulo the ring length
        let int_length = self.nodes.len() as i64;

        let value = match value.cmp(&0) {
            std::cmp::Ordering::Less => -((-value) % (int_length - 1)),
            std::cmp::Ordering::Greater => value % (int_length - 1),
            std::cmp::Ordering::Equal => value,
        };

        // Transform into [-length / 2, length / 2] to avoid insert/remove issues below
        let value = if value > int_length / 2 {
            value - (int_length - 1)
        } else if value < -int_length / 2 {
            value + (int_length - 1)
        } else {
            value
        };

        // First find the destination
        let (destination_previous, destination_next) = match value.cmp(&0) {
            std::cmp::Ordering::Less => {
                let distance = ((-value) as usize) % self.nodes.len();
                if distance == 0 {
                    return;
                }
                let mut previous_index = current_previous;
                for _ in 0..distance {
                    previous_index = self.nodes[previous_index].previous;
                }
                (previous_index, self.nodes[previous_index].next)
            }
            std::cmp::Ordering::Greater => {
                let distance = (value as usize) % self.nodes.len();
                if distance == 0 {
                    return;
                }
                let mut next_index = current_next;
                for _ in 0..distance {
                    next_index = self.nodes[next_index].next;
                }
                (self.nodes[next_index].previous, next_index)
            }
            std::cmp::Ordering::Equal => return,
        };

        // Remove the node from its current location
        self.nodes[current_previous].next = self.nodes[index].next;
        self.nodes[current_next].previous = self.nodes[index].previous;

        // Insert it into the destination location
        self.nodes[destination_previous].next = index;
        self.nodes[index].previous = destination_previous;
        self.nodes[destination_next].previous = index;
        self.nodes[index].next = destination_next;
    }

    fn get_grove_coordinates(&self) -> i64 {
        let mut index = self.nodes.iter().position(|node| node.value == 0).unwrap();
        let mut sum = 0;
        for iteration in 0..=3000 {
            if iteration > 1 && iteration % 1000 == 0 {
                sum += self.nodes[index].value;
            }
            index = self.nodes[index].next;
        }
        sum
    }
}

impl std::fmt::Debug for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;
        for _ in 0..self.nodes.len() {
            writeln!(f, "{}", self.nodes[index].value)?;
            index = self.nodes[index].next;
        }
        write!(f, "")
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);
    let values: Vec<_> = lines.map(|line| line.parse().unwrap()).collect();

    let mut unencrypted_ring = Ring::new(&values);
    for index in 0..unencrypted_ring.len() {
        unencrypted_ring.move_node(index);
    }

    println!(
        "Unencrypted grove coordinates: {}",
        unencrypted_ring.get_grove_coordinates()
    );

    let encrypted_values: Vec<_> = values.iter().map(|value| value * 811_589_153).collect();
    let mut encrypted_ring = Ring::new(&encrypted_values);
    for _ in 0..10 {
        for index in 0..encrypted_ring.len() {
            encrypted_ring.move_node(index);
        }
    }

    println!(
        "Encrypted grove coordinates: {}",
        encrypted_ring.get_grove_coordinates()
    );
}
