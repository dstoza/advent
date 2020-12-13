#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read line"));
    let earliest_timestamp: i32 = line
        .trim()
        .parse()
        .expect("Failed to read earliest timestamp");
    line.clear();

    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read line"));
    let (route, next_arrival) = line
        .trim()
        .split(',')
        .filter(|route| *route != "x")
        .map(|route_str| {
            route_str
                .parse::<i32>()
                .expect("Failed to parse route as i32")
        })
        .map(|route| {
            (
                route,
                (earliest_timestamp / route + 1) * route - earliest_timestamp,
            )
        })
        .min_by_key(|(_route, next_arrival)| *next_arrival)
        .expect("Failed to find next arrival");
    println!("{}", route * next_arrival);
}
