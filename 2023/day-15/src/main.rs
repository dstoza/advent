#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn hash(data: &[u8]) -> usize {
    let mut hash = 0;
    for b in data {
        hash += usize::from(*b);
        hash += hash << 4;
        hash %= 256;
    }

    hash
}

#[derive(Clone, Debug)]
struct Lens {
    label: Vec<u8>,
    focal_length: usize,
}

impl Lens {
    fn new(label: Vec<u8>, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

fn run_step(step: &str, boxes: &mut [Vec<Lens>]) {
    let step = step.as_bytes();
    let command_position = step.iter().position(|b| *b == b'-' || *b == b'=').unwrap();
    let label = &step[0..command_position];
    let box_for_label = &mut boxes[hash(label)];
    let command = step[command_position];
    match command {
        b'-' => {
            if let Some(position) = box_for_label.iter().position(|lens| lens.label == label) {
                box_for_label.remove(position);
            }
        }
        b'=' => {
            let focal_length = std::str::from_utf8(&step[command_position + 1..])
                .map(|focal_length| focal_length.parse().unwrap())
                .unwrap();
            if let Some(entry) = box_for_label.iter_mut().find(|lens| lens.label == label) {
                entry.focal_length = focal_length;
            } else {
                box_for_label.push(Lens::new(Vec::from(label), focal_length));
            }
        }
        _ => unreachable!(),
    }
}

fn focusing_power(boxes: &[Vec<Lens>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, lenses)| {
            lenses.iter().enumerate().map(move |(lens_index, lens)| {
                (box_index + 1) * (lens_index + 1) * lens.focal_length
            })
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let steps = line.split(',').map(String::from).collect::<Vec<_>>();

    let hash_sum: usize = steps.iter().map(|step| hash(step.as_bytes())).sum();
    println!("{hash_sum}");

    let mut boxes = vec![Vec::new(); 256];
    for step in steps {
        run_step(&step, &mut boxes);
    }

    println!("{}", focusing_power(&boxes));
}
