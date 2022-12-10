#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

struct Cpu {
    cycle: i32,
    x: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { cycle: 1, x: 1 }
    }

    fn get_pixel(&self, pixel_index: i32) -> char {
        if (self.x - 1..=self.x + 1).contains(&pixel_index) {
            '#'
        } else {
            '.'
        }
    }
}

fn advance_pixel_index(pixel_index: &mut i32) {
    *pixel_index = (*pixel_index + 1) % 40;
}

type SignalStrength = i32;
type Pixels = String;

fn run_program(lines: impl Iterator<Item = String>) -> (SignalStrength, Pixels) {
    let mut cpu = Cpu::new();

    let mut signal_strength = 0;

    let mut pixel_index = 0;
    let mut pixels = String::new();

    for line in lines {
        if line == "noop" {
            if (cpu.cycle - 20) % 40 == 0 {
                signal_strength += cpu.cycle * cpu.x;
            }
            cpu.cycle += 1;

            pixels.push(cpu.get_pixel(pixel_index));
            advance_pixel_index(&mut pixel_index);
        } else {
            for _ in 0..2 {
                if (cpu.cycle - 20) % 40 == 0 {
                    signal_strength += cpu.cycle * cpu.x;
                }
                cpu.cycle += 1;

                pixels.push(cpu.get_pixel(pixel_index));
                advance_pixel_index(&mut pixel_index);
            }

            let value: i32 = line.strip_prefix("addx ").unwrap().parse().unwrap();
            cpu.x += value;
        }
    }

    (signal_strength, pixels)
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let (signal_strength, pixels) = run_program(lines);
    println!("Signal strength: {signal_strength}");

    for (index, char) in pixels.chars().enumerate() {
        print!("{char}");
        if index % 40 == 39 {
            println!();
        }
    }
}
