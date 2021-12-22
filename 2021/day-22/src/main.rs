#![feature(test)]
extern crate test;

use std::{
    cell::RefCell,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
    rc::Rc, ops::Range,
};

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Off,
    On,
}

#[derive(Debug, Eq, PartialEq)]
struct Step {
    command: Command,
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

fn parse_steps<I: Iterator<Item = String>>(lines: I) -> Vec<Step> {
    lines.map(|line| {
        let mut split = line.split(' ');
        let command = match split.next() {
            Some("off") => Command::Off,
            Some("on") => Command::On,
            _ => unreachable!()
        };

        let ranges = split.next().unwrap().split(',');
        let ranges: Vec<_> = ranges.map(|range| {
            let mut range = range.split('=').nth(1).unwrap().split("..");
            range.next().unwrap().parse().unwrap()..(range.next().unwrap().parse::<i32>().unwrap() + 1)
        }).collect();

        Step {
            command,
            x: ranges[0].clone(),
            y: ranges[1].clone(),
            z: ranges[2].clone(),
        }
    }).collect()
}

struct Node {
    x: i32,
    y: i32,
    z: i32,
    size: u32,
    is_complete: bool,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_basic_example() -> [String; 4] {
        [
            String::from("on x=10..12,y=10..12,z=10..12"),
            String::from("on x=11..13,y=11..13,z=11..13"),
            String::from("off x=9..11,y=9..11,z=9..11"),
            String::from("on x=10..10,y=10..10,z=10..10"),
        ]
    }

    #[test]
    fn test_basic_example() {
        let steps = parse_steps(get_basic_example().into_iter());
        assert_eq!(steps.len(), 4);
        // The ranges look different because the problem specification uses inclusive ranges,
        // but this code assumes exclusive ranges
        assert_eq!(steps[0], Step { command: Command::On, x: 10..13, y: 10..13, z: 10..13});
        assert_eq!(steps[1], Step { command: Command::On, x: 11..14, y: 11..14, z: 11..14});
        assert_eq!(steps[2], Step { command: Command::Off, x: 9..12, y: 9..12, z: 9..12});
        assert_eq!(steps[3], Step { command: Command::On, x: 10..11, y: 10..11, z: 10..11});
    }

    // #[bench]
    // fn bench_input(b: &mut Bencher) {
    //     let file = File::open("input.txt").unwrap();
    //     let reader = BufReader::new(file);
    //     let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    //     b.iter(|| {
    //         let (algorithm, pixels) = parse_input(lines.clone().into_iter());
    //         assert_eq!(run_iterations(&algorithm, pixels, 50), 12333);
    //     });
    // }
}
