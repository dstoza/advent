#![feature(test)]
extern crate test;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{point, Point3};

#[derive(Clone, Copy)]
enum Orientation {
    PositiveZUpPositiveY,
    PositiveZUpPositiveX,
    PositiveZUpNegativeY,
    PositiveZUpNegativeX,
    NegativeZUpPositiveY,
    NegativeZUpNegativeX,
    NegativeZUpNegativeY,
    NegativeZUpPositiveX,
    PositiveXUpPositiveY,
    PositiveXUpNegativeZ,
    PositiveXUpNegativeY,
    PositiveXUpPositiveZ,
    NegativeXUpPositiveY,
    NegativeXUpPositiveZ,
    NegativeXUpNegativeY,
    NegativeXUpNegativeZ,
    PositiveYUpNegativeZ,
    PositiveYUpPositiveX,
    PositiveYUpPositiveZ,
    PositiveYUpNegativeX,
    NegativeYUpNegativeZ,
    NegativeYUpNegativeX,
    NegativeYUpPositiveZ,
    NegativeYUpPositiveX,
}

const ORIENTATIONS: [Orientation; 24] = [
    Orientation::PositiveZUpPositiveY,
    Orientation::PositiveZUpPositiveX,
    Orientation::PositiveZUpNegativeY,
    Orientation::PositiveZUpNegativeX,
    Orientation::NegativeZUpPositiveY,
    Orientation::NegativeZUpNegativeX,
    Orientation::NegativeZUpNegativeY,
    Orientation::NegativeZUpPositiveX,
    Orientation::PositiveXUpPositiveY,
    Orientation::PositiveXUpNegativeZ,
    Orientation::PositiveXUpNegativeY,
    Orientation::PositiveXUpPositiveZ,
    Orientation::NegativeXUpPositiveY,
    Orientation::NegativeXUpPositiveZ,
    Orientation::NegativeXUpNegativeY,
    Orientation::NegativeXUpNegativeZ,
    Orientation::PositiveYUpNegativeZ,
    Orientation::PositiveYUpPositiveX,
    Orientation::PositiveYUpPositiveZ,
    Orientation::PositiveYUpNegativeX,
    Orientation::NegativeYUpNegativeZ,
    Orientation::NegativeYUpNegativeX,
    Orientation::NegativeYUpPositiveZ,
    Orientation::NegativeYUpPositiveX,
];

fn orient_coordinates(coordinates: &Point3<i32>, orientation: Orientation) -> Point3<i32> {
    match orientation {
        Orientation::PositiveZUpPositiveY => *coordinates,
        Orientation::PositiveZUpPositiveX => {
            point![-coordinates.y, coordinates.x, coordinates.z]
        }
        Orientation::PositiveZUpNegativeY => {
            point![-coordinates.x, -coordinates.y, coordinates.z]
        }
        Orientation::PositiveZUpNegativeX => {
            point![coordinates.y, -coordinates.x, coordinates.z]
        }
        Orientation::NegativeZUpPositiveY => {
            point![-coordinates.x, coordinates.y, -coordinates.z]
        }
        Orientation::NegativeZUpNegativeX => {
            point![-coordinates.y, -coordinates.x, -coordinates.z]
        }
        Orientation::NegativeZUpNegativeY => {
            point![coordinates.x, -coordinates.y, -coordinates.z]
        }
        Orientation::NegativeZUpPositiveX => {
            point![coordinates.y, coordinates.x, -coordinates.z]
        }
        Orientation::PositiveXUpPositiveY => {
            point![-coordinates.z, coordinates.y, coordinates.x]
        }
        Orientation::PositiveXUpNegativeZ => {
            point![-coordinates.y, -coordinates.z, coordinates.x]
        }
        Orientation::PositiveXUpNegativeY => {
            point![coordinates.z, -coordinates.y, coordinates.x]
        }
        Orientation::PositiveXUpPositiveZ => {
            point![coordinates.y, coordinates.z, coordinates.x]
        }
        Orientation::NegativeXUpPositiveY => {
            point![coordinates.z, coordinates.y, -coordinates.x]
        }
        Orientation::NegativeXUpPositiveZ => {
            point![-coordinates.y, coordinates.z, -coordinates.x]
        }
        Orientation::NegativeXUpNegativeY => {
            point![-coordinates.z, -coordinates.y, -coordinates.x]
        }
        Orientation::NegativeXUpNegativeZ => {
            point![coordinates.y, -coordinates.z, -coordinates.x]
        }
        Orientation::PositiveYUpNegativeZ => {
            point![coordinates.x, -coordinates.z, coordinates.y]
        }
        Orientation::PositiveYUpPositiveX => {
            point![coordinates.z, coordinates.x, coordinates.y]
        }
        Orientation::PositiveYUpPositiveZ => {
            point![-coordinates.x, coordinates.z, coordinates.y]
        }
        Orientation::PositiveYUpNegativeX => {
            point![-coordinates.z, -coordinates.x, coordinates.y]
        }
        Orientation::NegativeYUpNegativeZ => {
            point![-coordinates.x, -coordinates.z, -coordinates.y]
        }
        Orientation::NegativeYUpNegativeX => {
            point![coordinates.z, -coordinates.x, -coordinates.y]
        }
        Orientation::NegativeYUpPositiveZ => {
            point![coordinates.x, coordinates.z, -coordinates.y]
        }
        Orientation::NegativeYUpPositiveX => {
            point![-coordinates.z, coordinates.x, -coordinates.y]
        }
    }
}

struct Scanner {
    relative_beacons: Vec<Point3<i32>>,
    absolute_position: Point3<i32>,
    absolute_beacons: Vec<Point3<i32>>,
    anchor_relative_beacons: Vec<HashSet<Point3<i32>>>,
}

impl Scanner {
    fn from_lines<I: Iterator<Item = String>>(lines: &mut I) -> Self {
        Self {
            relative_beacons: lines
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut split = line.split(',');
                    point![
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap()
                    ]
                })
                .collect(),
            absolute_position: point![0, 0, 0],
            absolute_beacons: Vec::new(),
            anchor_relative_beacons: Vec::new(),
        }
    }

    fn is_resolved(&self) -> bool {
        !self.absolute_beacons.is_empty()
    }

    fn resolve(&mut self, absolute_beacons: Vec<Point3<i32>>) {
        self.absolute_beacons = absolute_beacons;
        for beacon in &self.absolute_beacons[..self.absolute_beacons.len() - 11] {
            self.anchor_relative_beacons.push(
                self.absolute_beacons
                    .iter()
                    .map(|other_beacon| Point3::origin() + (other_beacon - beacon))
                    .collect(),
            );
        }
    }

    fn get_oriented_relative_beacons(&self, orientation: Orientation) -> Vec<Point3<i32>> {
        self.relative_beacons
            .iter()
            .map(|coordinates| orient_coordinates(coordinates, orientation))
            .collect()
    }

    fn resolve_against(
        &mut self,
        orientation: Orientation,
        anchor_beacon: Point3<i32>,
        common_beacon: Point3<i32>,
    ) {
        let oriented_relative_beacons = self.get_oriented_relative_beacons(orientation);
        let absolute_beacons = oriented_relative_beacons
            .iter()
            .map(|beacon| beacon + (anchor_beacon - common_beacon))
            .collect();
        self.resolve(absolute_beacons);
        self.absolute_position =
            Point3::origin() + (self.absolute_beacons[0] - oriented_relative_beacons[0]);
    }

    fn try_resolve_against(&mut self, anchor: &Self) -> bool {
        for orientation in ORIENTATIONS {
            let oriented_relative_beacons = self.get_oriented_relative_beacons(orientation);

            for common_beacon in oriented_relative_beacons.iter() {
                let common_relative_beacons: HashSet<_> = oriented_relative_beacons
                    .iter()
                    .map(|beacon| Point3::origin() + (beacon - common_beacon))
                    .collect();

                for (anchor_index, anchor_beacons) in
                    anchor.anchor_relative_beacons.iter().enumerate()
                {
                    if common_relative_beacons.intersection(anchor_beacons).count() >= 12 {
                        // Found a match between anchor.absolute_beacons[anchor_index]
                        // and oriented_relative_beacons[common_index]
                        self.resolve_against(
                            orientation,
                            anchor.absolute_beacons[anchor_index],
                            *common_beacon,
                        );
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn parse_scanners<I: Iterator<Item = String>>(mut lines: I) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    while let Some(_) = lines.next() {
        scanners.push(Scanner::from_lines(&mut lines));
    }
    scanners
}

fn resolve_scanners(scanners: &mut Vec<Scanner>) {
    let last = scanners.len() - 1;
    scanners.swap(0, last);
    let mut resolved = scanners.pop().unwrap();
    let absolute_beacons = resolved.relative_beacons.clone();
    resolved.resolve(absolute_beacons);

    let mut complete = Vec::new();
    let mut anchors = vec![resolved];
    let mut unresolved: Vec<_> = scanners.drain(..).collect();
    while !unresolved.is_empty() {
        for scanner in &mut unresolved {
            for anchor in &anchors {
                if scanner.try_resolve_against(anchor) {
                    break;
                }
            }
        }

        complete.append(&mut anchors);

        // Basically drain_filter, but it isn't stable yet
        let mut index = 0;
        while index < unresolved.len() {
            if unresolved[index].is_resolved() {
                anchors.push(unresolved.remove(index));
            } else {
                index += 1;
            }
        }
    }

    complete.append(&mut anchors);
    std::mem::swap(scanners, &mut complete);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut scanners = parse_scanners(reader.lines().map(|line| line.unwrap()));
    println!("Found {} scanners", scanners.len());
    resolve_scanners(&mut scanners);
    // let unique: HashSet<_> = scanners
    //     .iter()
    //     .flat_map(|scanner| scanner.absolute_beacons.iter())
    //     .collect();
    // println!("Found {} unique points", unique.len());
    let mut max_distance = 0;
    for i in 0..scanners.len() {
        let left_position = scanners[i].absolute_position;
        for scanner in scanners.iter().skip(i + 1) {
            let right_position = scanner.absolute_position;
            max_distance = max_distance.max(
                (left_position.x - right_position.x).abs()
                    + (left_position.y - right_position.y).abs()
                    + (left_position.z - right_position.z).abs(),
            )
        }
    }
    println!("Max distance: {}", max_distance);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use test::Bencher;

    fn get_example_input() -> [String; 54] {
        [
            String::from("--- scanner 0 ---"),
            String::from("404,-588,-901"),
            String::from("528,-643,409"),
            String::from("-838,591,734"),
            String::from("390,-675,-793"),
            String::from("-537,-823,-458"),
            String::from("-485,-357,347"),
            String::from("-345,-311,381"),
            String::from("-661,-816,-575"),
            String::from("-876,649,763"),
            String::from("-618,-824,-621"),
            String::from("553,345,-567"),
            String::from("474,580,667"),
            String::from("-447,-329,318"),
            String::from("-584,868,-557"),
            String::from("544,-627,-890"),
            String::from("564,392,-477"),
            String::from("455,729,728"),
            String::from("-892,524,684"),
            String::from("-689,845,-530"),
            String::from("423,-701,434"),
            String::from("7,-33,-71"),
            String::from("630,319,-379"),
            String::from("443,580,662"),
            String::from("-789,900,-551"),
            String::from("459,-707,401"),
            String::from(""),
            String::from("--- scanner 1 ---"),
            String::from("686,422,578"),
            String::from("605,423,415"),
            String::from("515,917,-361"),
            String::from("-336,658,858"),
            String::from("95,138,22"),
            String::from("-476,619,847"),
            String::from("-340,-569,-846"),
            String::from("567,-361,727"),
            String::from("-460,603,-452"),
            String::from("669,-402,600"),
            String::from("729,430,532"),
            String::from("-500,-761,534"),
            String::from("-322,571,750"),
            String::from("-466,-666,-811"),
            String::from("-429,-592,574"),
            String::from("-355,545,-477"),
            String::from("703,-491,-529"),
            String::from("-328,-685,520"),
            String::from("413,935,-424"),
            String::from("-391,539,-444"),
            String::from("586,-435,557"),
            String::from("-364,-763,-893"),
            String::from("807,-499,-711"),
            String::from("755,-354,-619"),
            String::from("553,889,-390"),
            String::from(""),
        ]
    }

    #[test]
    fn test_orientations_are_unique() {
        let mut coordinates = HashSet::new();
        for orientation in ORIENTATIONS {
            let transformed = orient_coordinates(&point![1, 2, 3], orientation);
            coordinates.insert(transformed);
        }
        assert_eq!(coordinates.len(), ORIENTATIONS.len());
    }

    #[test]
    fn test_try_resolve_against() {
        let mut scanners = parse_scanners(get_example_input().into_iter());
        assert_eq!(scanners.len(), 2);
        let absolute_beacons = scanners[0].relative_beacons.clone();
        scanners[0].resolve(absolute_beacons);
        scanners.swap(0, 1);
        let resolved = scanners.pop().unwrap();

        assert_eq!(scanners[0].try_resolve_against(&resolved), true);
        println!("Resolved position {:?}", scanners[0].absolute_position);
    }

    // #[bench]
    // fn bench_input(b: &mut Bencher) {
    //     let file = File::open("input.txt").unwrap();
    //     let reader = BufReader::new(file);
    //     let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    //     b.iter(|| {
    //         assert_eq!(get_maximum_magnitude(&lines), 4638);
    //     })
    // }
}
