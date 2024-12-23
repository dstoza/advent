#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

fn bron_kerbosch(
    r: &HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    neighbors: &HashMap<String, HashSet<String>>,
    maximal: &mut Vec<String>,
) {
    if p.is_empty() && x.is_empty() {
        let mut m = r.iter().cloned().collect::<Vec<_>>();
        if m.len() > maximal.len() {
            m.sort_unstable();
            *maximal = m;
        }
    }

    let vertices = p.iter().cloned().collect::<Vec<_>>();
    for v in &vertices {
        bron_kerbosch(
            &r.union(&HashSet::from([v.clone()])).cloned().collect(),
            p.intersection(neighbors.get(v).unwrap()).cloned().collect(),
            x.intersection(neighbors.get(v).unwrap()).cloned().collect(),
            neighbors,
            maximal,
        );
        p.remove(v);
        x.insert(v.to_owned());
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut neighbors = HashMap::new();
    for line in reader.lines().map(Result::unwrap) {
        let mut split = line.split('-');
        let a = split.next().unwrap().to_owned();
        let b = split.next().unwrap().to_owned();

        neighbors
            .entry(a.clone())
            .and_modify(|others: &mut HashSet<_>| {
                others.insert(b.clone());
            })
            .or_insert_with(|| HashSet::from([b.clone()]));

        neighbors
            .entry(b)
            .and_modify(|others: &mut HashSet<_>| {
                others.insert(a.clone());
            })
            .or_insert_with(|| HashSet::from([a]));
    }

    let mut trios = HashSet::new();
    for (from, to) in neighbors
        .iter()
        .flat_map(|(from, to)| to.iter().map(|t| (from.to_owned(), t)))
    {
        let from_connections = neighbors.get(&from).unwrap();
        let to_connections = neighbors.get(to).unwrap();
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

    println!("{count}");

    let vertices = neighbors.keys().cloned().collect::<HashSet<_>>();
    let mut maximal = Vec::new();
    bron_kerbosch(
        &HashSet::new(),
        vertices,
        HashSet::new(),
        &neighbors,
        &mut maximal,
    );

    println!("{}", maximal.join(","));
}
