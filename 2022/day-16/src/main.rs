#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

type VertexIndex = usize;

#[derive(Debug)]
struct Graph {
    vertices: Vec<String>,
    edges: HashSet<(VertexIndex, VertexIndex)>,
    shortest_paths: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: HashSet::new(),
            shortest_paths: Vec::new(),
        }
    }

    fn add_edge(&mut self, from: &String, to: &String) {
        if !self.vertices.contains(from) {
            self.vertices.push(from.to_string());
        }
        let from_index = self.vertices.iter().position(|v| *v == *from).unwrap();

        if !self.vertices.contains(to) {
            self.vertices.push(to.to_string());
        }
        let to_index = self.vertices.iter().position(|v| *v == *to).unwrap();

        self.edges
            .insert((from_index.min(to_index), from_index.max(to_index)));
    }

    // Compute shortest paths using Floyd-Warshall
    fn compute_shortest_paths(&mut self) {
        self.shortest_paths = vec![vec![usize::MAX; self.vertices.len()]; self.vertices.len()];
        for index in 0..self.vertices.len() {
            self.shortest_paths[index][index] = 0;
        }

        for (from, to) in &self.edges {
            self.shortest_paths[*from][*to] = 1;
            self.shortest_paths[*to][*from] = 1;
        }

        for middle in 0..self.vertices.len() {
            for from in 0..self.vertices.len() {
                for to in 0..self.vertices.len() {
                    if self.shortest_paths[from][middle] == usize::MAX
                        || self.shortest_paths[middle][to] == usize::MAX
                    {
                        continue;
                    }

                    self.shortest_paths[from][to] = self.shortest_paths[from][to]
                        .min(self.shortest_paths[from][middle] + self.shortest_paths[middle][to]);
                }
            }
        }
    }

    fn get_path_length(&self, from: &String, to: &String) -> usize {
        let from_index = self.vertices.iter().position(|v| v == from).unwrap();
        let to_index = self.vertices.iter().position(|v| v == to).unwrap();
        self.shortest_paths[from_index][to_index]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FlowableValve {
    name: String,
    flow_rate: usize,
}

impl FlowableValve {
    fn new(name: String, flow_rate: usize) -> Self {
        Self { name, flow_rate }
    }
}

// Returns the complete graph plus a list of vertexes with non-0 flow rate
fn parse_graph(lines: impl Iterator<Item = String>) -> (Graph, Vec<FlowableValve>) {
    let mut flowable_valves = Vec::new();
    let mut graph = Graph::new();

    for line in lines {
        let mut split = line.split(';');

        let mut valve = split.next().unwrap().split(" has flow rate=");
        let valve_name = valve
            .next()
            .unwrap()
            .strip_prefix("Valve ")
            .unwrap()
            .to_string();
        let flow_rate = valve.next().unwrap().parse().unwrap();

        let tunnels = split.next().unwrap().split("valve").nth(1).unwrap();

        let tunnels = if tunnels.starts_with('s') {
            tunnels.strip_prefix("s ").unwrap()
        } else {
            tunnels.strip_prefix(' ').unwrap()
        }
        .split(", ");

        for tunnel in tunnels {
            graph.add_edge(&valve_name, &tunnel.to_string());
        }

        if flow_rate > 0 {
            flowable_valves.push(FlowableValve::new(valve_name, flow_rate));
        }
    }

    graph.compute_shortest_paths();

    (graph, flowable_valves)
}

type ElapsedTime = usize;

const DURATION: usize = 26;

#[derive(Clone, Debug, Eq)]
struct State {
    position: String,
    opened: Vec<(FlowableValve, ElapsedTime)>,
    time_elapsed: ElapsedTime,
    remaining: Vec<FlowableValve>,
}

impl State {
    fn new(position: String, mut remaining: Vec<FlowableValve>) -> Self {
        remaining.sort_by_key(|valve| valve.flow_rate);
        remaining.reverse();
        Self {
            position,
            opened: Vec::new(),
            time_elapsed: 0,
            remaining,
        }
    }

    fn get_released(&self) -> usize {
        self.opened
            .iter()
            .map(|(valve, opened_since)| (DURATION - opened_since) * valve.flow_rate)
            .sum()
    }

    fn get_potential(&self) -> usize {
        self.get_released()
            + self
                .remaining
                .iter()
                .take((DURATION - self.time_elapsed) / 2)
                .enumerate()
                .map(|(index, valve)| {
                    (DURATION - self.time_elapsed - 2 * (1 + index)) * valve.flow_rate
                })
                .sum::<usize>()
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.get_potential() == other.get_potential()
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_potential().cmp(&other.get_potential())
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.time_elapsed)?;
        for (valve, when) in &self.opened {
            write!(f, " {} {}", valve.name, when)?;
        }
        write!(f, "  {}", self.get_released())
    }
}

fn compute_maximum_pressure(graph: &Graph, flowable_valves: &[FlowableValve]) -> usize {
    let mut queue = BinaryHeap::new();

    let initial_state = State::new(String::from("AA"), flowable_valves.to_vec());
    let mut best = initial_state.clone();
    queue.push(initial_state);

    while let Some(state) = queue.pop() {
        if state.get_potential() < best.get_released() {
            continue;
        }

        if state.get_released() > best.get_released() {
            best = state.clone();
        }

        for remaining in &state.remaining {
            let distance_to_valve = graph.get_path_length(&state.position, &remaining.name);
            let valve_opened_at = state.time_elapsed + distance_to_valve + 1;

            if valve_opened_at < DURATION {
                let mut opened = state.opened.clone();
                opened.push((remaining.clone(), valve_opened_at));

                let new_state = State {
                    position: remaining.name.clone(),
                    opened,
                    time_elapsed: valve_opened_at,
                    remaining: state
                        .remaining
                        .iter()
                        .filter(|r| r.name != remaining.name)
                        .cloned()
                        .collect(),
                };

                queue.push(new_state);
            }
        }
    }

    best.get_released()
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let (graph, flowable_valves) = parse_graph(lines);

    let maximum_pressure = compute_maximum_pressure(&graph, &flowable_valves);
    println!("{maximum_pressure}");

    let mut elephant_maximum = 0;
    for mine in flowable_valves.iter().powerset() {
        if mine.is_empty() {
            continue;
        }

        if mine.len() > flowable_valves.len() {
            break;
        }

        let mine: Vec<_> = mine.iter().map(|valve| (**valve).clone()).collect();
        let elephants: Vec<_> = flowable_valves
            .iter()
            .filter(|valve| !mine.contains(valve))
            .cloned()
            .collect();

        // println!("Checking {:?} {:?}", mine, elephants);

        elephant_maximum = elephant_maximum.max(
            compute_maximum_pressure(&graph, &mine) + compute_maximum_pressure(&graph, &elephants),
        );
    }

    println!("{elephant_maximum}");
}
