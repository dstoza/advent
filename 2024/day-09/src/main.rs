#![warn(clippy::pedantic)]

use std::io::{BufRead, BufReader};

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

#[derive(Debug)]
struct File {
    id: u16,
    start: usize,
    length: usize,
}

impl File {
    fn new(id: u16, start: usize, length: usize) -> Self {
        Self { id, start, length }
    }
}

fn get_files(map: &[u8]) -> Vec<File> {
    let mut id = 0u16;
    let mut start = 0usize;
    let mut mode = Mode::File;
    let mut files = Vec::new();
    for entry in map {
        mode = match mode {
            Mode::File => {
                files.push(File::new(id, start, usize::from(*entry)));
                start += usize::from(*entry);
                id += 1;
                Mode::Space
            }
            Mode::Space => {
                start += usize::from(*entry);
                Mode::File
            }
        };
    }

    files
}

fn defragment_chunks(mut files: Vec<File>) -> usize {
    for id in (0..=files.last().unwrap().id).rev() {
        let position = files.iter().position(|file| file.id == id).unwrap();
        let file_length = files[position].length;
        let mut gap = None;
        for (index, window) in files[..=position].windows(2).enumerate() {
            let gap_length = window[1].start - (window[0].start + window[0].length);
            if gap_length >= file_length {
                gap = Some(index + 1);
                break;
            }
        }

        if let Some(gap) = gap {
            let gap_start = files[gap - 1].start + files[gap - 1].length;
            let mut file = files.remove(position);
            file.start = gap_start;
            files.insert(gap, file);
        }
    }

    files
        .iter()
        .map(|file| {
            (file.start..(file.start + file.length))
                .map(|location| location * usize::from(file.id))
                .sum::<usize>()
        })
        .sum()
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

    let file = std::fs::File::open(args.filename).unwrap();
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

    if args.part == 1 {
        let mut blocks = expand(&map);
        defragment_blocks(&mut blocks);
        println!("{}", checksum(&blocks));
    } else {
        let files = get_files(&map);
        println!("{}", defragment_chunks(files));
    }
}
