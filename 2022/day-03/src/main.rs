#![warn(clippy::pedantic)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn get_item_priority(item: u8) -> u32 {
    if item >= 97 {
        u32::from(item - 97 + 1)
    } else {
        u32::from(item - 65 + 27)
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);

    let mut rucksack_sum = 0;
    let mut badge_sum = 0;
    let mut possible_badges = HashSet::new();
    for (elf_id, line) in reader.lines().map(std::result::Result::unwrap).enumerate() {
        let mut first_compartment_contents = HashSet::<u8>::new();
        for item in line.as_bytes().iter().take(line.len() / 2) {
            first_compartment_contents.insert(*item);
        }

        let mut contents = first_compartment_contents.clone();
        let mut common_item = None;
        for item in line.as_bytes().iter().skip(line.len() / 2) {
            contents.insert(*item);
            if first_compartment_contents.contains(item) {
                common_item = Some(*item);
            }
        }
        let common_item = common_item.unwrap();

        rucksack_sum += get_item_priority(common_item);

        if elf_id % 3 == 0 {
            possible_badges = contents;
        } else {
            possible_badges = &possible_badges & &contents;
        }

        if elf_id % 3 == 2 {
            assert!(possible_badges.len() == 1);
            badge_sum += get_item_priority(*possible_badges.iter().next().unwrap());
        }
    }

    println!("Rucksack sum: {rucksack_sum}");
    println!("Badge sum: {badge_sum}");
}
