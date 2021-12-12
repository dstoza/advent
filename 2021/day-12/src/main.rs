use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_neighbors<I: Iterator<Item = String>>(lines: I) -> HashMap<String, Vec<String>> {
    let mut neighbors = HashMap::new();

    for line in lines {
        let mut split = line.split('-');
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        if to != "start" {
            neighbors
                .entry(String::from(from))
                .or_insert_with(Vec::new)
                .push(String::from(to));
        }
        if from != "start" {
            neighbors
                .entry(String::from(to))
                .or_insert_with(Vec::new)
                .push(String::from(from));
        }
    }

    neighbors
}

fn do_count_paths<'a>(
    neighbors: &'a HashMap<String, Vec<String>>,
    allow_duplicates: bool,
    current_path: &mut Vec<&'a str>,
    has_duplicate: bool,
) -> usize {
    let current_cave = current_path.last().unwrap();
    if *current_cave == "end" {
        return 1;
    }

    let mut paths = 0;

    for neighbor in &neighbors[*current_cave] {
        let has_duplicate = if neighbor != "end"
            && neighbor.chars().next().unwrap().is_lowercase()
            && current_path.iter().rfind(|element| *element == neighbor) != None
        {
            if !allow_duplicates || has_duplicate {
                continue;
            }
            true
        } else {
            has_duplicate
        };

        current_path.push(neighbor);
        paths += do_count_paths(neighbors, allow_duplicates, current_path, has_duplicate);
        current_path.pop();
    }

    paths
}

fn count_paths(neighbors: &HashMap<String, Vec<String>>, allow_duplicates: bool) -> usize {
    do_count_paths(neighbors, allow_duplicates, &mut vec!["start"], false)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let neighbors = parse_neighbors(reader.lines().map(|l| l.unwrap()));
    println!("Paths: {}", count_paths(&neighbors, true))
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_simple() -> [String; 7] {
        [
            String::from("start-A"),
            String::from("start-b"),
            String::from("A-c"),
            String::from("A-b"),
            String::from("b-d"),
            String::from("A-end"),
            String::from("b-end"),
        ]
    }

    fn get_slightly_larger() -> [String; 10] {
        [
            String::from("dc-end"),
            String::from("HN-start"),
            String::from("start-kj"),
            String::from("dc-start"),
            String::from("dc-HN"),
            String::from("LN-dc"),
            String::from("HN-end"),
            String::from("kj-sa"),
            String::from("kj-HN"),
            String::from("kj-dc"),
        ]
    }

    fn get_even_larger() -> [String; 18] {
        [
            String::from("fs-end"),
            String::from("he-DX"),
            String::from("fs-he"),
            String::from("start-DX"),
            String::from("pj-DX"),
            String::from("end-zg"),
            String::from("zg-sl"),
            String::from("zg-pj"),
            String::from("pj-he"),
            String::from("RW-he"),
            String::from("fs-DX"),
            String::from("pj-RW"),
            String::from("zg-RW"),
            String::from("start-pj"),
            String::from("he-WI"),
            String::from("zg-he"),
            String::from("pj-fs"),
            String::from("start-RW"),
        ]
    }

    #[test]
    fn test_count_paths_simple() {
        let neighbors = parse_neighbors(get_simple().into_iter());
        assert_eq!(count_paths(&neighbors, false), 10);
    }

    #[test]
    fn test_count_paths_simple_with_duplicates() {
        let neighbors = parse_neighbors(get_simple().into_iter());
        assert_eq!(count_paths(&neighbors, true), 36);
    }

    #[test]
    fn test_count_paths_slightly_larger() {
        let neighbors = parse_neighbors(get_slightly_larger().into_iter());
        assert_eq!(count_paths(&neighbors, false), 19);
    }

    #[test]
    fn test_count_paths_slightly_larger_with_duplicates() {
        let neighbors = parse_neighbors(get_slightly_larger().into_iter());
        assert_eq!(count_paths(&neighbors, true), 103);
    }

    #[test]
    fn test_count_paths_even_larger() {
        let neighbors = parse_neighbors(get_even_larger().into_iter());
        assert_eq!(count_paths(&neighbors, false), 226);
    }

    #[test]
    fn test_count_paths_even_larger_with_duplicates() {
        let neighbors = parse_neighbors(get_even_larger().into_iter());
        assert_eq!(count_paths(&neighbors, true), 3509);
    }
}
