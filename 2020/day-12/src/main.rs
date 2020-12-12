#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn from_i32(value: i32) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Unexpected value {}", value),
        }
    }
}

enum Rotation {
    Right,
    Left,
}

enum Mode {
    Ship,
    Waypoint,
}

struct Navigator {
    mode: Mode,
    x: i32,
    y: i32,
    direction: Direction,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Navigator {
    fn new(mode: Mode) -> Self {
        Self {
            mode,
            x: 0,
            y: 0,
            direction: Direction::East,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn translate(&mut self, direction: Direction, amount: i32) {
        let (x, y) = match self.mode {
            Mode::Ship => (&mut self.x, &mut self.y),
            Mode::Waypoint => (&mut self.waypoint_x, &mut self.waypoint_y),
        };

        match direction {
            Direction::North => {
                *y += amount;
            }
            Direction::East => {
                *x += amount;
            }
            Direction::South => {
                *y -= amount;
            }
            Direction::West => {
                *x -= amount;
            }
        };
    }

    fn rotate_waypoint_clockwise(&mut self) {
        let (x, y) = (self.waypoint_y, -self.waypoint_x);
        self.waypoint_x = x;
        self.waypoint_y = y;
    }

    fn turn(&mut self, rotation: &Rotation, amount: i32) {
        let clockwise_amount = match rotation {
            Rotation::Right => amount,
            Rotation::Left => 360 - amount,
        };
        let direction = self.direction as i32 + clockwise_amount / 90;
        for _ in 0..(clockwise_amount / 90) {
            self.rotate_waypoint_clockwise();
        }
        self.direction = Direction::from_i32(direction % 4);
    }

    fn move_forward(&mut self, amount: i32) {
        match self.mode {
            Mode::Ship => self.translate(self.direction, amount),
            Mode::Waypoint => {
                self.x += self.waypoint_x * amount;
                self.y += self.waypoint_y * amount;
            }
        }
    }

    fn parse_line(&mut self, line: &str) {
        let amount = line[1..].parse().expect("Failed to parse amount as i32");
        match line.as_bytes()[0] {
            b'N' => self.translate(Direction::North, amount),
            b'E' => self.translate(Direction::East, amount),
            b'S' => self.translate(Direction::South, amount),
            b'W' => self.translate(Direction::West, amount),
            b'L' => self.turn(&Rotation::Left, amount),
            b'R' => self.turn(&Rotation::Right, amount),
            b'F' => self.move_forward(amount),
            _ => panic!("Unexpected prefix {}", line.as_bytes()[0]),
        }
    }

    fn get_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        return;
    }

    let mode = match args[2].as_str() {
        "ship" => Mode::Ship,
        "waypoint" => Mode::Waypoint,
        _ => panic!("Unexpected mode {}", args[2].as_str()),
    };

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut navigator = Navigator::new(mode);

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        navigator.parse_line(line.trim());

        line.clear();
    }

    println!("Distance: {}", navigator.get_distance());
}
