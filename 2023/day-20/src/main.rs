#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

trait Module: Debug {
    fn handle_inputs(&mut self, _inputs: Vec<String>) {}
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(outputs: Vec<String>) -> Self {
        Self { on: false, outputs }
    }
}

impl Module for FlipFlop {}

#[derive(Debug)]
struct Conjunction {
    last_pulse: HashMap<String, bool>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(outputs: Vec<String>) -> Self {
        Self {
            last_pulse: HashMap::new(),
            outputs,
        }
    }
}

impl Module for Conjunction {
    fn handle_inputs(&mut self, inputs: Vec<String>) {
        println!("handle_inputs {self:?} {inputs:?}");
        for input in inputs {
            self.last_pulse.insert(input, false);
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new(outputs: Vec<String>) -> Self {
        Self { outputs }
    }
}

impl Module for Broadcaster {}

fn collect_outputs(
    name: &str,
    outputs: &str,
    inputs: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    let outputs: Vec<String> = outputs.split(", ").map(std::string::String::from).collect();
    for output in &outputs {
        inputs
            .entry(output.clone())
            .and_modify(|list| list.push(String::from(name)))
            .or_insert_with(|| vec![String::from(name)]);
    }
    outputs
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines().map(std::result::Result::unwrap) {
        let mut parts = line.split(" -> ");
        let module = parts.next().unwrap();
        if module.starts_with('%') {
            let name = String::from(module.trim_start_matches('%'));
            let outputs = collect_outputs(&name, parts.next().unwrap(), &mut inputs);
            let flip_flop = FlipFlop::new(outputs);
            modules.insert(name, Box::new(flip_flop));
        } else if module.starts_with('&') {
            let name = String::from(module.trim_start_matches('&'));
            let outputs = collect_outputs(&name, parts.next().unwrap(), &mut inputs);
            let conjunction = Conjunction::new(outputs);
            modules.insert(name, Box::new(conjunction));
        } else {
            assert_eq!(module, "broadcaster");
            let outputs = collect_outputs(module, parts.next().unwrap(), &mut inputs);
            let broadcaster = Broadcaster::new(outputs);
            modules.insert(String::from(module), Box::new(broadcaster));
        }
    }

    for (name, module) in &mut modules {
        if let Some(inputs) = inputs.remove(name) {
            module.handle_inputs(inputs);
        }
    }
}
