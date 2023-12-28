#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use rand::{seq::SliceRandom, thread_rng};

type Connections = HashMap<String, HashSet<String>>;

fn shortest_path(from: &str, to: &str, connections: &Connections) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut previous = HashMap::new();

    let mut queue: VecDeque<String> = VecDeque::from([String::from(from)]);
    'search: while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node.clone());

        if let Some(connections) = connections.get(&node) {
            for connection in connections {
                if visited.contains(connection) {
                    continue;
                }

                if !previous.contains_key(connection) {
                    previous.insert(connection.clone(), node.clone());
                }

                if connection == to {
                    break 'search;
                }
                queue.push_back(connection.clone());
            }
        }
    }

    let mut path = vec![String::from(to)];
    while let Some(previous) = previous.get(path.last().unwrap()) {
        path.push(previous.clone());
    }
    path.reverse();

    path
}

fn count_reachable(connections: &Connections) -> usize {
    let mut visited = HashSet::new();

    let mut queue: VecDeque<String> =
        VecDeque::from([String::from(connections.keys().next().unwrap())]);
    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node.clone());

        if let Some(connections) = connections.get(&node) {
            for connection in connections {
                if visited.contains(connection) {
                    continue;
                }

                queue.push_back(connection.clone());
            }
        }
    }

    visited.len()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut connections: Connections = HashMap::new();
    for line in reader.lines().map(std::result::Result::unwrap) {
        let mut parts = line.split(": ");
        let from = parts.next().unwrap();
        for to in parts.next().unwrap().split_whitespace() {
            connections
                .entry(String::from(from))
                .and_modify(|connections| {
                    connections.insert(String::from(to));
                })
                .or_insert_with(|| HashSet::from([String::from(to)]));

            connections
                .entry(String::from(to))
                .and_modify(|connections| {
                    connections.insert(String::from(from));
                })
                .or_insert_with(|| HashSet::from([String::from(from)]));
        }
    }

    let mut rng = thread_rng();

    loop {
        let reachable = count_reachable(&connections);
        if reachable < connections.len() {
            println!("{}", reachable * (connections.len() - reachable));
            break;
        }

        let nodes = connections.keys().cloned().collect::<Vec<_>>();

        let mut pairs: HashMap<String, usize> = HashMap::new();

        for _ in 0..(connections.len() / 100).max(50) {
            let from = nodes.choose(&mut rng).unwrap();
            let to = nodes.choose(&mut rng).unwrap();
            if from == to {
                continue;
            }

            let path = shortest_path(from, to, &connections);
            for window in path.windows(2) {
                let min = window[0].clone().min(window[1].clone());
                let max = window[0].clone().max(window[1].clone());
                let key = min + &max;
                pairs
                    .entry(key)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let mut pairs = pairs.iter().collect::<Vec<_>>();
        pairs.sort_unstable_by_key(|(_, count)| **count);
        let (max_key, _) = pairs.last().unwrap();
        let max_a = &max_key[..3];
        let max_b = &max_key[3..];

        if let Some(connections) = connections.get_mut(max_a) {
            connections.remove(max_b);
        }

        if let Some(connections) = connections.get_mut(max_b) {
            connections.remove(max_a);
        }
    }
}
