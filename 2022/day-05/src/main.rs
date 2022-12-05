#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

type StackSlice = Vec<Option<char>>;

fn parse_stack_slices(lines: &mut impl Iterator<Item = String>) -> Vec<StackSlice> {
    let mut slices = Vec::new();
    for line in lines {
        let mut slice = Vec::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    slice.push(Some(chars.next().unwrap()));
                    chars.next();
                    chars.next();
                }
                ' ' => {
                    if let Some('1') = chars.next() {
                        return slices;
                    }

                    slice.push(None);
                    chars.next();
                    chars.next();
                }
                _ => unreachable!(),
            }
        }
        slices.push(slice);
    }
    unreachable!()
}

type Stack = Vec<char>;

fn slices_to_stacks(mut slices: Vec<StackSlice>) -> Vec<Stack> {
    let mut stacks = vec![Vec::new(); slices[0].len()];

    slices.reverse();
    for slice in slices {
        for (index, element) in slice.iter().enumerate() {
            if let Some(c) = *element {
                stacks[index].push(c);
            }
        }
    }

    stacks
}

fn execute_command(command: &str, stacks: &mut [Stack], retain_order: bool) {
    let mut split = command.split(' ');
    let quantity = split.nth(1).unwrap().parse::<usize>().unwrap();
    let source = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    let destination = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

    let mut moved: Vec<_> = stacks[source]
        .drain(stacks[source].len() - quantity..)
        .collect();
    if !retain_order {
        moved.reverse();
    }
    stacks[destination].append(&mut moved);
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let retain_order = if let Some(retain) = std::env::args().nth(2) {
        retain.as_str() == "retain"
    } else {
        false
    };

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let slices = parse_stack_slices(&mut lines);
    let mut stacks = slices_to_stacks(slices);

    lines.next(); // Skip blank line before commands
    for line in lines {
        execute_command(line.as_str(), &mut stacks, retain_order);
    }

    let message: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    println!("{message}");
}
