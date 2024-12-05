#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// File to open
    filename: String,
}

fn parse_edges(lines: &mut impl Iterator<Item = String>) -> Vec<(u16, u16)> {
    let mut edges = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('|');
        let from = split.next().unwrap().parse::<u16>().unwrap();
        let to = split.next().unwrap().parse::<u16>().unwrap();

        edges.push((from, to));
    }

    edges
}

fn get_topological_sort(edges: &[(u16, u16)], nodes: &[u16]) -> Vec<u16> {
    let mut edges = edges
        .iter()
        .filter(|(from, to)| nodes.iter().any(|n| n == from) && nodes.iter().any(|n| n == to))
        .collect::<Vec<_>>();

    // Implementation of Kahn's algorithm
    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn%27s_algorithm

    let mut no_incoming = Vec::new();
    for node in nodes {
        if edges.iter().all(|(_from, to)| to != node) {
            no_incoming.push(*node);
        }
    }

    let mut sorted = Vec::new();
    while let Some(node) = no_incoming.pop() {
        sorted.push(node);

        let edges_to_remove = edges
            .iter()
            .filter(|(from, _to)| *from == node)
            .copied()
            .collect::<Vec<_>>();

        for edge in edges_to_remove {
            edges.retain(|e| *e != edge);

            let (_from, to) = edge;
            if edges.iter().all(|(_f, t)| t != to) {
                no_incoming.push(*to);
            }
        }
    }

    sorted
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(Result::unwrap);

    let edges = parse_edges(&mut lines);

    let mut correct_sum = 0;
    let mut incorrect_sum = 0;
    for update in lines.map(|line| {
        line.split(',')
            .map(|page| page.parse::<u16>().unwrap())
            .collect::<Vec<_>>()
    }) {
        let sorted = get_topological_sort(&edges, &update);

        if sorted == update {
            correct_sum += update[update.len() / 2];
        } else {
            incorrect_sum += sorted[sorted.len() / 2];
        }
    }

    println!("{correct_sum} {incorrect_sum}");
}
