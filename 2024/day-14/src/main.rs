#![warn(clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    ops::{Add, Mul},
};

use clap::Parser;
use flate2::{bufread::DeflateEncoder, Compression};
use regex::Regex;

#[derive(Parser)]
#[command(disable_help_flag(true))]
struct Args {
    /// File to open
    filename: String,

    #[arg(short, long)]
    width: i32,

    #[arg(short, long)]
    height: i32,

    #[arg(short, long)]
    steps: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn wrap(&mut self, width: i32, height: i32) {
        self.x = (self.x % width + width) % width;
        self.y = (self.y % height + height) % height;
    }

    fn quadrant(self, width: i32, height: i32) -> Option<i32> {
        //  3 | 0
        //  -----
        //  2 | 1

        match (self.x.cmp(&(width / 2)), self.y.cmp(&(height / 2))) {
            (Ordering::Greater, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Greater) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Less, Ordering::Less) => Some(3),
            _ => None,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Clone, Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn new(position: Vector, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    fn position_after_steps(&self, steps: i32, width: i32, height: i32) -> Vector {
        let mut position = self.position + self.velocity * steps;
        position.wrap(width, height);
        position
    }
}

fn parse_robots(lines: impl Iterator<Item = String>) -> Vec<Robot> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    lines
        .map(|line| {
            let [px, py, vx, vy] = regex.captures(&line).unwrap().extract().1;
            let position = Vector::new(px.parse().unwrap(), py.parse().unwrap());
            let velocity = Vector::new(vx.parse().unwrap(), vy.parse().unwrap());
            Robot::new(position, velocity)
        })
        .collect()
}

fn find_first_nonoverlapping(robots: &[Robot], width: i32, height: i32) -> i32 {
    let mut steps = 1;
    'outer: loop {
        let mut positions = HashSet::new();
        for robot in robots {
            let position = robot.position_after_steps(steps, width, height);
            if positions.contains(&position) {
                steps += 1;
                continue 'outer;
            }

            positions.insert(position);
        }

        break;
    }

    steps
}

struct OccupancyGrid {
    width: usize,
    grid: Vec<u8>,
}

impl OccupancyGrid {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width: width as usize,
            grid: vec![u8::from(false); (width * height) as usize],
        }
    }

    fn set(&mut self, position: Vector) {
        self.grid[position.y as usize * self.width + position.x as usize] = u8::from(true);
    }

    fn into_buffer(self) -> Vec<u8> {
        self.grid
    }
}

fn compute_entropy(positions: OccupancyGrid) -> usize {
    let buffer = positions.into_buffer();
    let reader = BufReader::new(buffer.as_slice());
    let mut compressor = DeflateEncoder::new(reader, Compression::fast());
    let mut output = Vec::new();
    compressor.read_to_end(&mut output).unwrap();
    output.len()
}

fn find_minimum_entropy(robots: &[Robot], width: i32, height: i32) -> i32 {
    (0..width * height)
        .map(|steps| {
            let mut positions = OccupancyGrid::new(width, height);
            for robot in robots {
                let position = robot.position_after_steps(steps, width, height);
                positions.set(position);
            }
            (steps, compute_entropy(positions))
        })
        .min_by_key(|(_steps, entropy)| *entropy)
        .unwrap()
        .0
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let robots = parse_robots(reader.lines().map(Result::unwrap));

    let mut quadrants = HashMap::new();
    let mut positions = HashMap::new();
    for robot in robots.clone() {
        let position = robot.position_after_steps(args.steps, args.width, args.height);
        positions
            .entry(position)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
        if let Some(quadrant) = position.quadrant(args.width, args.height) {
            quadrants
                .entry(quadrant)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
        }
    }

    let safety_factor: i32 = quadrants.values().product();
    println!("{safety_factor}");

    let first_nonoverlapping = find_first_nonoverlapping(&robots, args.width, args.height);
    let minimum_entropy = find_minimum_entropy(&robots, args.width, args.height);
    println!("{first_nonoverlapping} {minimum_entropy}");
}
