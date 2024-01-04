#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use approx::relative_eq;
use nalgebra::{point, vector, Point2, Point3, Vector3};

#[derive(Clone, Debug)]
struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

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

    fn time_offset(&self, other: &Self) -> Option<f64> {
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
        let self_time = (intersection_x - self.origin.x) / self.direction.x;
        let other_time = (intersection_x - other.origin.x) / other.direction.x;

        Some(other_time - self_time)
    }

    fn distance(&self, other: &Self) -> Option<f64> {
        let n = self.direction.cross(&other.direction);
        let n_1 = self.direction.cross(&n);
        let n_2 = other.direction.cross(&n);
        let self_nearest = self.origin
            + (other.origin - self.origin).dot(&n_2) / self.direction.dot(&n_2) * self.direction;
        let other_time = (self.origin - other.origin).dot(&n_1) / other.direction.dot(&n_1);
        let other_nearest = other.origin + other_time * other.direction;
        if other_time >= 0.0 {
            Some((other_nearest - self_nearest).magnitude())
        } else {
            None
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

    let a = rays[0].clone();
    let b = rays[1].clone();

    // Magic values obtained from manually iterating using the loop below
    let t_a = 353_090_968_659f64;
    let t_b = 870_093_641_616f64;

    // for t_b in (870093640616i64..).take(1_000_000) {
    //     let a_at_t = a.origin + t_a as f64 * a.direction;
    //     let b_at_t = b.origin + t_b as f64 * b.direction;
    //     let projected = Ray::new(a_at_t, b_at_t - a_at_t);
    //     let distance: f64 = rays[3..]
    //         .iter()
    //         .filter_map(|ray| projected.distance(ray))
    //         .sum();

    //     println!("{t_a} {t_b} {distance:?}");
    // }

    // Given the magic values, figure out the initial position
    let a_at_t = a.origin + t_a * a.direction;
    let b_at_t = b.origin + t_b * b.direction;
    let projected = Ray::new(a_at_t, (b_at_t - a_at_t) / (t_b - t_a));
    let time_offset = projected.time_offset(&a).unwrap();
    let projected = Ray::new(
        projected.origin - projected.direction * time_offset,
        projected.direction,
    );

    #[allow(clippy::cast_possible_truncation)]
    let coordinate_sum =
        (projected.origin.x + projected.origin.y + projected.origin.z).round() as i64;
    println!("{coordinate_sum}");
}
