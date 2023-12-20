#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

trait Module: Debug {
    fn handle_inputs(&mut self, _inputs: Vec<String>) {}
    fn send_pulse(&mut self, high: bool, from: &str) -> Vec<(String, bool)>;
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

impl Module for FlipFlop {
    fn send_pulse(&mut self, high: bool, _from: &str) -> Vec<(String, bool)> {
        if high {
            return Vec::new();
        }

        self.on = !self.on;
        self.outputs
            .iter()
            .map(|output| (output.clone(), self.on))
            .collect()
    }
}

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
        for input in inputs {
            self.last_pulse.insert(input, false);
        }
    }

    fn send_pulse(&mut self, high: bool, from: &str) -> Vec<(String, bool)> {
        *self.last_pulse.get_mut(from).unwrap() = high;
        let all_high = self.last_pulse.values().all(|last| *last);
        self.outputs
            .iter()
            .map(|output| (output.clone(), !all_high))
            .collect()
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

impl Module for Broadcaster {
    fn send_pulse(&mut self, high: bool, _from: &str) -> Vec<(String, bool)> {
        self.outputs
            .iter()
            .map(|output| (output.clone(), high))
            .collect()
    }
}

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

fn count_pulses(
    modules: &mut HashMap<String, Box<dyn Module>>,
    iterations: usize,
) -> (usize, usize) {
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for _ in 0..iterations {
        let mut queue =
            VecDeque::from([(String::from("broadcaster"), false, String::from("button"))]);
        while let Some((name, high, from)) = queue.pop_front() {
            if high {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            if let Some(module) = modules.get_mut(&name) {
                let propagated = module.send_pulse(high, &from);
                for (to, high) in propagated {
                    queue.push_back((to, high, name.clone()));
                }
            }
        }
    }

    (high_pulses, low_pulses)
}

fn presses_to_rx(modules: &mut HashMap<String, Box<dyn Module>>) -> usize {
    let mut last_message = String::new();
    let mut presses = 0;
    loop {
        presses += 1;
        let mut queue =
            VecDeque::from([(String::from("broadcaster"), false, String::from("button"))]);
        while let Some((name, high, from)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&name) {
                if name == "zp" {
                    let message = format!("{module:?}");
                    if message != last_message {
                        // println!("{presses} {message}");
                        last_message = message;
                    }
                }
                let propagated = module.send_pulse(high, &from);
                for (to, high) in propagated {
                    // if to == "ds" {
                    //     println!("ds {high} {presses}");
                    // }
                    // if to == "nd" {
                    //     println!("nd {high} {presses}");
                    // }
                    // if to == "sb" {
                    //     println!("sb {high} {presses}");
                    // }
                    // if to == "hf" {
                    //     println!("hf {high} {presses}");
                    // }
                    queue.push_back((to, high, name.clone()));
                }
            }
        }
    }
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

    for module in &modules {
        println!("{module:?}");
    }

    // let (high_pulses, low_pulses) = count_pulses(&mut modules, 1000);
    // println!("{high_pulses} {low_pulses} {}", high_pulses * low_pulses);

    println!("{}", presses_to_rx(&mut modules));
}
