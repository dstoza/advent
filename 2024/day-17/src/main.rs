#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Override initial value of A register
    #[arg(short, long)]
    a: Option<i64>,

    /// File to open
    filename: String,
}

struct RegisterFile<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> RegisterFile<T>
where
    T: Copy + From<u8>,
{
    fn load_combo(&self, operand: u8) -> T {
        match operand {
            0..=3 => T::from(operand),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

impl<T> RegisterFile<T>
where
    T: std::fmt::Debug + FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn parse_register(string: &str) -> T where {
        string.split(": ").nth(1).unwrap().parse().unwrap()
    }

    fn parse(lines: &mut impl Iterator<Item = String>) -> Self {
        Self {
            a: Self::parse_register(&lines.next().unwrap()),
            b: Self::parse_register(&lines.next().unwrap()),
            c: Self::parse_register(&lines.next().unwrap()),
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(Result::unwrap);

    let mut register_file = RegisterFile::<i64>::parse(&mut lines);
    if let Some(a) = args.a {
        register_file.a = a;
    }

    let program = lines
        .nth(1)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|position| position.parse().unwrap())
        .collect::<Vec<u8>>();

    let mut output = Vec::new();

    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            0 => {
                // adv
                register_file.a >>= register_file.load_combo(program[ip + 1]);
                ip += 2;
            }
            1 => {
                // bxl
                register_file.b ^= i64::from(program[ip + 1]);
                ip += 2;
            }
            2 => {
                // bst
                register_file.b = register_file.load_combo(program[ip + 1]) % 8;
                ip += 2;
            }
            3 => {
                // jnz
                if register_file.a == 0 {
                    ip += 2;
                } else {
                    ip = program[ip + 1] as usize;
                }
            }
            4 => {
                // bxc
                register_file.b ^= register_file.c;
                ip += 2;
            }
            5 => {
                // out
                output.push(register_file.load_combo(program[ip + 1]) % 8);
                ip += 2;
            }
            6 => {
                // bdv
                register_file.b = register_file.a >> register_file.load_combo(program[ip + 1]);
                ip += 2;
            }
            7 => {
                // cdv
                register_file.c = register_file.a >> register_file.load_combo(program[ip + 1]);
                ip += 2;
            }
            _ => unreachable!(),
        }
    }

    println!("{output:?}");
}
