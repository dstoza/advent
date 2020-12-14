#![deny(clippy::all, clippy::pedantic)]

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

enum Mode {
    Address,
    Value,
}

struct ProgramLoader {
    mode: Mode,
    set_mask: u64,
    clear_mask: u64,
    floating_bits: Vec<u8>,
    memory: HashMap<u64, u64>,
}

impl ProgramLoader {
    fn new(mode: Mode) -> Self {
        let mut memory = HashMap::new();
        memory.reserve(100_000);
        Self {
            mode,
            set_mask: 0,
            clear_mask: 0,
            floating_bits: Vec::new(),
            memory,
        }
    }

    fn update_masks(&mut self, mask: &str) {
        self.set_mask = 0;
        self.clear_mask = 0;
        self.floating_bits.clear();
        let mut index = 36_u8;
        for byte in mask.as_bytes() {
            self.set_mask <<= 1;
            self.clear_mask <<= 1;
            index -= 1;
            match *byte {
                b'X' => self.floating_bits.push(index),
                b'0' => self.clear_mask |= 1,
                b'1' => self.set_mask |= 1,
                _ => panic!("Unexpected mask byte {}", *byte),
            }
        }
    }

    fn write_value(memory: &mut HashMap<u64, u64>, address: u64, floating_bits: &[u8], value: u64) {
        if floating_bits.is_empty() {
            memory.insert(address, value);
            return;
        }

        ProgramLoader::write_value(
            memory,
            address | 1_u64 << floating_bits[0],
            &floating_bits[1..],
            value,
        );
        ProgramLoader::write_value(
            memory,
            address & !(1_u64 << floating_bits[0]),
            &floating_bits[1..],
            value,
        );
    }

    fn write_memory(&mut self, line: &str) {
        let mut split = line.split('=');

        let address = split.next().expect("Failed to get address split").trim();
        let address: u64 = address[1..address.len() - 1]
            .parse()
            .expect("Failed to parse address as u64");

        let value: u64 = split
            .next()
            .expect("Failed to get value split")
            .trim()
            .parse()
            .expect("Failed to parse value as u64");

        match self.mode {
            Mode::Address => {
                ProgramLoader::write_value(
                    &mut self.memory,
                    address | self.set_mask,
                    &self.floating_bits,
                    value,
                );
                None
            }
            Mode::Value => self
                .memory
                .insert(address, (value | self.set_mask) & !self.clear_mask),
        };
    }

    fn parse_line(&mut self, line: &str) {
        match &line[0..3] {
            "mas" => self.update_masks(&line[7..]),
            "mem" => self.write_memory(&line[3..]),
            _ => panic!("Unexpected line [{}]", line),
        }
    }

    fn get_memory_sum(&self) -> u64 {
        self.memory.iter().map(|(_, value)| *value).sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        return;
    }

    let mode = match args[2].as_ref() {
        "address" => Mode::Address,
        "value" => Mode::Value,
        _ => panic!("Unexpected mode {}", args[2]),
    };

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut loader = ProgramLoader::new(mode);

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        loader.parse_line(line.trim());

        line.clear();
    }

    println!("Sum: {}", loader.get_memory_sum());
}
