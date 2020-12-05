use std::{
    cmp::max,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use bit_set::BitSet;

fn parse_row(line: &[u8]) -> usize {
    let mut row = 0;
    let mut factor = 64;
    for c in line {
        if *c == b'B' {
            row += factor;
        }
        factor /= 2;
    }
    row
}

fn parse_column(line: &[u8]) -> usize {
    let mut column = 0;
    let mut factor = 4;
    for c in line {
        if *c == b'R' {
            column += factor;
        }
        factor /= 2;
    }
    column
}

fn parse_seat(line: &str) -> usize {
    let bytes = line.as_bytes();
    parse_row(&bytes[0..7]) * 8 + parse_column(&bytes[7..])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut max_seat = 0;
    let mut occupied = BitSet::new();

    let mut line = String::new();
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let seat = parse_seat(&line.trim());

        max_seat = max(max_seat, seat);
        occupied.insert(seat);

        line.clear();
    }

    println!("Max seat: {}", max_seat);
    for seat in occupied.into_iter() {
        if !occupied.contains(seat + 1) && occupied.contains(seat + 2) {
            println!("My seat: {}", seat + 1);
            break;
        }
    }
}
