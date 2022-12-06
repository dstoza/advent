#![warn(clippy::pedantic)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn get_marker_position(buffer: &str, distinct_characters: usize) -> usize {
    buffer
        .as_bytes()
        .windows(distinct_characters)
        .map(|window| {
            let mut elements = HashSet::new();
            window.iter().all(|element| elements.insert(*element))
        })
        .position(|p| p)
        .unwrap()
        + distinct_characters
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);

    for line in reader.lines().map(std::result::Result::unwrap) {
        println!(
            "First packet marker after character {}",
            get_marker_position(&line, 4)
        );
        println!(
            "First message marker after character {}",
            get_marker_position(&line, 14)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_position() {
        assert_eq!(get_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            get_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            get_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }
}
