#![deny(clippy::all, clippy::pedantic)]

use std::{
    convert::TryInto,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq)]
enum Command {
    Accumulate,
    Jump,
    None,
}

struct Operation {
    command: Command,
    payload: i32,
}

impl Operation {
    fn from_line(line: &str) -> Operation {
        let mut split = line.split(' ');

        let mnemonic = split.next().expect("Failed to parse mnemonic");
        let command = match mnemonic {
            "acc" => Command::Accumulate,
            "jmp" => Command::Jump,
            "nop" => Command::None,
            _ => panic!("Unexpected mnemonic [{}]", mnemonic),
        };

        let payload = split
            .next()
            .expect("Failed to parse payload")
            .parse()
            .expect("Failed to parse payload as i32");

        Operation { command, payload }
    }

    fn execute(&self, flip_operation: bool, accumulator: &mut i32, pc: &mut usize) {
        let command = if flip_operation {
            match self.command {
                Command::Accumulate => Command::Accumulate,
                Command::Jump => Command::None,
                Command::None => Command::Jump,
            }
        } else {
            self.command
        };

        match command {
            Command::Accumulate => {
                *accumulator += self.payload;
                *pc += 1;
            }
            Command::Jump => {
                let signed_pc: isize = (*pc).try_into().expect("Failed to fit PC in isize");
                *pc = (signed_pc + self.payload as isize)
                    .try_into()
                    .expect("Failed to fit signed PC in usize");
            }
            Command::None => *pc += 1,
        }
    }
}

struct Instruction {
    operation: Operation,
    visited: bool,
}

impl Instruction {
    fn new(operation: Operation) -> Self {
        Self {
            operation,
            visited: false,
        }
    }
}

fn run_program(program: &mut Vec<Instruction>, flip_pc: Option<usize>) -> Result<i32, i32> {
    let mut accumulator = 0;
    let mut pc = 0_usize;
    loop {
        let instruction = &mut program[pc as usize];

        if instruction.visited {
            return Err(accumulator);
        }

        instruction.visited = true;
        instruction.operation.execute(
            flip_pc.map_or(false, |flip_pc| flip_pc == pc),
            &mut accumulator,
            &mut pc,
        );

        if pc == program.len() {
            return Ok(accumulator);
        }

        if pc > program.len() {
            return Err(-1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut program = Vec::new();

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        program.push(Instruction::new(Operation::from_line(&line.trim())));

        line.clear();
    }

    if let Err(accumulator) = run_program(&mut program, None) {
        println!("Infinite loop accumulator {}", accumulator);
    }

    for skip_pc in 0..program.len() {
        if program[skip_pc].operation.command == Command::Accumulate {
            continue;
        }

        // Reset visited bits before running
        for instruction in &mut program {
            instruction.visited = false;
        }

        if let Ok(accumulator) = run_program(&mut program, Some(skip_pc)) {
            println!(
                "Flipping PC {} terminated with accumulator {}",
                skip_pc, accumulator
            );
            break;
        }
    }
}
