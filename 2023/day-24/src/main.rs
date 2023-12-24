#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{point, vector, Point3, Vector3};

#[derive(Debug)]
struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    fn parse(string: &str) -> Self {
        let mut parts = string.split(" @ ");
        let mut origin = parts.next().unwrap().split(',');
        let mut direction = parts.next().unwrap().split(',');

        Self {
            origin: point![
                origin.next().unwrap().trim().parse().unwrap(),
                origin.next().unwrap().trim().parse().unwrap(),
                origin.next().unwrap().trim().parse().unwrap()
            ],
            direction: vector![
                direction.next().unwrap().trim().parse().unwrap(),
                direction.next().unwrap().trim().parse().unwrap(),
                direction.next().unwrap().trim().parse().unwrap(),
            ],
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let rays = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| Ray::parse(&line))
        .collect::<Vec<_>>();
}
