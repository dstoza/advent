#![deny(clippy::all, clippy::pedantic)]

use std::{
    convert::TryInto,
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
        .filter_map(|route| {
            if route == "x" {
                return None;
            }

            let route = route.parse::<i32>().expect("Failed to parse route as i32");
            let next_arrival = (earliest_timestamp / route + 1) * route - earliest_timestamp;

            Some((route, next_arrival))
        })
        .min_by_key(|(_route, next_arrival)| *next_arrival)
        .expect("Failed to find next arrival");

    println!(
        "Next arrival {} in {} minutes (product {})",
        route,
        next_arrival,
        route * next_arrival
    );

    let mut timestamp = 0;
    let mut skip = 1;
    for (id, modulo) in line
        .trim()
        .split(',')
        .enumerate()
        .filter_map(|(index, id)| {
            if id == "x" {
                return None;
            }

            let id = id.parse::<i64>().expect("Failed to parse route as i64");
            let index: i64 = index.try_into().expect("Failed to fit index into i64");

            let mut modulo = -index;
            while modulo < 0 {
                modulo += id;
            }

            Some((id, modulo))
        })
    {
        while timestamp % id != modulo {
            timestamp += skip;
        }

        skip *= id
    }

    println!("First subsequent timestamp: {}", timestamp);
}
