#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

struct Node {
    previous: usize,
    next: usize,
    value: i32,
}

struct Ring {
    nodes: Vec<Node>,
}

impl Ring {
    fn new(values: &[i32]) -> Self {
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

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    #[allow(clippy::comparison_chain)]
    fn move_node(&mut self, index: usize) {
        let value = self.nodes[index].value;
        let current_previous = self.nodes[index].previous;
        let current_next = self.nodes[index].next;

        // print!("{value} ");

        let int_length = self.nodes.len() as i32;
        let value = if value < 0 {
            -((-value) % (int_length - 1))
        } else if value > 0 {
            value % (int_length - 1)
        } else {
            value
        };
        
        // print!("{value} ");

        let value = if value > int_length / 2 {
            value - int_length + 1
        } else if value < -int_length / 2 {
            value + int_length - 1
        } else {
            value
        };

        // println!("{value} ");

        // First find the destination
        let (destination_previous, destination_next) = if value < 0 {
            let distance = ((-value) as usize) % self.nodes.len();
            if distance == 0 {
                return;
            }
            let mut previous_index = current_previous;
            for _ in 0..distance {
                previous_index = self.nodes[previous_index].previous;
            }
            (previous_index, self.nodes[previous_index].next)
        } else if value > 0 {
            let distance = (value as usize) % self.nodes.len();
            if distance == 0 {
                return;
            }
            let mut next_index = current_next;
            for _ in 0..distance {
                next_index = self.nodes[next_index].next;
            }
            (self.nodes[next_index].previous, next_index)
        } else {
            return;
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

    fn get_grove_coordinates(&self) -> i32 {
        let mut index = self.nodes.iter().position(|node| node.value == 0).unwrap();
        let mut sum = 0;
        for iteration in 0..=3000 {
            if iteration > 1 && iteration % 1000 == 0 {
                println!("{}", self.nodes[index].value);
                sum += self.nodes[index].value;
            }
            index = self.nodes[index].next;
        }
        sum
    }

    fn get_contents(&self) -> HashMap<i32, usize> {
        let mut hashmap = HashMap::new();
        let mut index = 0;
        for _ in 0..self.nodes.len() {
            *hashmap.entry(self.nodes[index].value).or_default() += 1;
            index = self.nodes[index].next;
        }
        hashmap
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
    let mut ring = Ring::new(&values);

    let mut previous_contents = ring.get_contents();

    // println!("{ring:?}");

    for index in 0..ring.len() {
        ring.move_node(index);
        // println!("{ring:?}");
        let contents = ring.get_contents();
        if contents != previous_contents {
            println!("Failed at {index}");
            break;
        }
        previous_contents = contents;
    }

    println!("Grove coordinates: {}", ring.get_grove_coordinates());
}
