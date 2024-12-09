#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Part of the problem to run
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// File to open
    filename: String,
}

#[derive(Clone, Copy)]
enum Mode {
    File,
    Space,
}

fn expand(map: &[u8]) -> Vec<u16> {
    let mut blocks = Vec::new();

    let mut id = 1u16;
    let mut mode = Mode::File;
    for entry in map {
        mode = match mode {
            Mode::File => {
                blocks.extend_from_slice(&vec![id; *entry as usize]);
                if *entry == 0 {
                    println!("0-length file");
                }
                id += 1;
                Mode::Space
            }
            Mode::Space => {
                blocks.extend_from_slice(&vec![0; *entry as usize]);
                Mode::File
            }
        };
    }

    blocks
}

fn defragment_blocks(blocks: &mut [u16]) {
    let mut free = blocks.iter().position(|b| *b == 0).unwrap();
    let mut end = blocks.len() - blocks.iter().rev().position(|b| *b != 0).unwrap() - 1;
    while free < end {
        blocks.swap(free, end);

        while blocks[free] != 0 && free < blocks.len() {
            free += 1;
        }
        if free == blocks.len() {
            break;
        }

        while blocks[end] == 0 && end > 0 {
            end -= 1;
        }
        if end == 0 {
            break;
        }
    }
}

fn defragment_chunks(blocks: &mut [u16]) {
    let mut id = *blocks.iter().rev().find(|b| **b != 0).unwrap();
    while id != 0 {
        let chunk_start = blocks.iter().position(|b| *b == id).unwrap();
        let chunk_size = blocks[chunk_start..]
            .iter()
            .take_while(|b| **b == id)
            .count();

        let mut destination = blocks.iter().position(|b| *b == 0);
        while let Some(start) = destination {
            let size = blocks[start..].iter().take_while(|b| **b == 0).count();

            if size >= chunk_size {
                break;
            }

            destination = blocks
                .iter()
                .skip(start + size)
                .position(|b| *b == 0)
                .map(|index| index + start + size);
        }

        if let Some(start) = destination {
            if start < chunk_start {
                let mut temp = vec![0u16; chunk_size];
                temp.swap_with_slice(&mut blocks[chunk_start..(chunk_start + chunk_size)]);
                temp.swap_with_slice(&mut blocks[start..(start + chunk_size)]);
                temp.swap_with_slice(&mut blocks[chunk_start..(chunk_start + chunk_size)]);
            }
        }

        id -= 1;
    }
}

fn checksum(blocks: &[u16]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(index, id)| {
            if *id == 0 {
                None
            } else {
                Some(index * (*id - 1) as usize)
            }
        })
        .sum()
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.filename).unwrap();
    let reader = BufReader::new(file);

    let map = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|b| *b - b'0')
        .collect::<Vec<_>>();

    let mut blocks = expand(&map);
    if args.part == 1 {
        defragment_blocks(&mut blocks);
    } else {
        defragment_chunks(&mut blocks);
    }

    println!("{}", checksum(&blocks));
}
