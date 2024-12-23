#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    println!("running part {}", args.part);

    let mut connections = HashMap::new();
    for line in reader.lines().map(Result::unwrap) {
        let mut split = line.split('-');
        let a = split.next().unwrap().to_owned();
        let b = split.next().unwrap().to_owned();

        connections
            .entry(a.clone())
            .and_modify(|others: &mut HashSet<_>| {
                others.insert(b.clone());
            })
            .or_insert_with(|| HashSet::from([b.clone()]));

        connections
            .entry(b)
            .and_modify(|others: &mut HashSet<_>| {
                others.insert(a.clone());
            })
            .or_insert_with(|| HashSet::from([a]));
    }

    let mut trios = HashSet::new();
    for (from, to) in connections
        .iter()
        .flat_map(|(from, to)| to.iter().map(|t| (from.to_owned(), t)))
    {
        let from_connections = connections.get(&from).unwrap();
        let to_connections = connections.get(to).unwrap();
        for third in from_connections.intersection(to_connections) {
            let mut trio = [from.clone(), to.to_owned(), third.to_owned()];
            trio.sort_unstable();
            trios.insert(trio);
        }
    }

    let count = trios
        .iter()
        .filter(|trio| trio.iter().any(|computer| computer.starts_with('t')))
        .count();

    // 2398 too high
    println!("{count}");
}
