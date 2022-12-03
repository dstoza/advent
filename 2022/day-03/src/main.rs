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
        let bytes = line.as_bytes();

        let first_compartment_contents: HashSet<_> =
            bytes.iter().take(line.len() / 2).copied().collect();

        let second_compartment_contents: HashSet<_> =
            bytes.iter().skip(line.len() / 2).copied().collect();

        let common_contents = &first_compartment_contents & &second_compartment_contents;
        assert!(common_contents.len() == 1);
        let common_item = *common_contents.iter().next().unwrap();

        rucksack_sum += get_item_priority(common_item);

        let contents = &first_compartment_contents | &second_compartment_contents;
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
