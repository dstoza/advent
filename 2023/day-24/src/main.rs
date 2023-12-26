#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use approx::relative_eq;
use nalgebra::{point, vector, Point2, Point3, Vector3};

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

    fn slope_intercept(&self) -> (f64, f64) {
        let slope = self.direction.y / self.direction.x;
        let intercept = self.origin.y - slope * self.origin.x;
        (slope, intercept)
    }

    fn intersection_2d(&self, other: &Self) -> Option<Point2<f64>> {
        // ax + b == cx + d
        // ax - cx == d - b
        // x == (d - b) / (a - c)
        let (a, b) = self.slope_intercept();
        let (c, d) = other.slope_intercept();

        if relative_eq!(a, c) {
            assert!(!relative_eq!(b, d));
            return None;
        }

        let intersection_x = (d - b) / (a - c);
        let intersection_y = a * intersection_x + b;
        let intersection = point![intersection_x, intersection_y];

        let self_time = (intersection_x - self.origin.x) / self.direction.x;
        let other_time = (intersection_x - other.origin.x) / other.direction.x;

        if self_time < 0.0 || other_time < 0.0 {
            return None;
        }

        Some(intersection)
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

    let intersection_range = 200_000_000_000_000.0..=400_000_000_000_000.0;

    let mut in_range = 0;
    for (index, ray) in rays.iter().enumerate() {
        for other in &rays[index + 1..] {
            if let Some(intersection) = ray.intersection_2d(other) {
                if intersection_range.contains(&intersection.x)
                    && intersection_range.contains(&intersection.y)
                {
                    in_range += 1;
                }
            }
        }
    }

    println!("{in_range}");
}
