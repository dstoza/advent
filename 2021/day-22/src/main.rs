#![warn(clippy::pedantic)]
#![feature(test)]
extern crate test;

use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    rc::Rc,
};

trait Intersection {
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl<T: Copy + Ord> Intersection for Range<T> {
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.end <= other.start || self.start >= other.end {
            None
        } else {
            Some(self.start.max(other.start)..self.end.min(other.end))
        }
    }
}

const HALF_PERMUTATIONS: [(bool, bool, bool); 8] = [
    (false, false, false),
    (false, false, true),
    (false, true, false),
    (false, true, true),
    (true, false, false),
    (true, false, true),
    (true, true, false),
    (true, true, true),
];

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    z: i32,
    size: i32, // Unsigned in practice and asserted as such at construction, but stored as i32 for ease of use
    is_cube: bool,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    const MAX_VALUE: i32 = 128 * 1024;

    fn new(x: i32, y: i32, z: i32, size: i32) -> Rc<RefCell<Self>> {
        assert!(size > 0);
        Rc::new(RefCell::new(Self {
            x,
            y,
            z,
            size,
            is_cube: true,
            children: Vec::new(),
        }))
    }

    fn create_root() -> Rc<RefCell<Self>> {
        let root = Self::new(
            -Node::MAX_VALUE,
            -Node::MAX_VALUE,
            -Node::MAX_VALUE,
            2 * Node::MAX_VALUE,
        );
        root.borrow_mut().is_cube = false;
        root
    }

    fn is_empty(&self) -> bool {
        !self.is_cube && self.children.is_empty()
    }

    fn count_cubes(&self) -> usize {
        (self.is_cube as usize)
            + self
                .children
                .iter()
                .map(|child| child.borrow().count_cubes())
                .sum::<usize>()
    }

    fn get_volume(&self) -> i32 {
        let self_volume = if self.is_cube {
            self.size * self.size * self.size
        } else {
            0
        };

        self_volume
            + self
                .children
                .iter()
                .map(|child| child.borrow().get_volume())
                .sum::<i32>()
    }

    fn subdivide_if_necessary(&mut self) {
        if !self.is_cube && !self.children.is_empty() {
            return;
        }

        for (top_half_x, top_half_y, top_half_z) in HALF_PERMUTATIONS {
            let x = if top_half_x {
                self.x + self.size / 2
            } else {
                self.x
            };
            let y = if top_half_y {
                self.y + self.size / 2
            } else {
                self.y
            };
            let z = if top_half_z {
                self.z + self.size / 2
            } else {
                self.z
            };

            self.children.push(Node::new(x, y, z, self.size / 2));
        }

        if !self.is_cube {
            for child in &mut self.children {
                child.borrow_mut().is_cube = false;
            }
        }

        self.is_cube = false;
    }

    fn get_child_index(&self, cube: &Node) -> usize {
        let top_x = cube.x >= self.x + self.size / 2;
        let top_y = cube.y >= self.y + self.size / 2;
        let top_z = cube.z >= self.z + self.size / 2;
        (top_x as usize) << 2 | (top_y as usize) << 1 | top_z as usize
    }

    fn insert_cube(&mut self, cube: &Node) {
        assert!(cube.x >= self.x);
        assert!(cube.y >= self.y);
        assert!(cube.z >= self.z);
        assert!(cube.size < self.size);

        // If we're already in an ancestor that is a full cube, we're done
        if self.is_cube {
            return;
        }

        self.subdivide_if_necessary();
        let child_index = self.get_child_index(cube);

        // Descend to the parent of the cube to be inserted
        if self.size > 2 * cube.size {
            self.children[child_index].borrow_mut().insert_cube(cube);
        } else {
            // Otherwise we are the parent node of the cube to be inserted
            let mut child = self.children[child_index].borrow_mut();
            child.is_cube = true;
            child.children.clear();
        }

        // Fix up completed cubes
        if self.children.iter().all(|child| child.borrow().is_cube) {
            self.is_cube = true;
            self.children.clear();
        }
    }

    fn remove_cube(&mut self, cube: &Node) {
        assert!(cube.x >= self.x);
        assert!(cube.y >= self.y);
        assert!(cube.z >= self.z);
        assert!(cube.size < self.size);

        // If we're already in an empty cube, we're done
        if !self.is_cube && self.children.is_empty() {
            return;
        }

        self.subdivide_if_necessary();
        let child_index = self.get_child_index(cube);

        // Descend to the parent of the cube to be removed
        if self.size > 2 * cube.size {
            self.children[child_index].borrow_mut().remove_cube(cube);
        } else {
            let mut child = self.children[child_index].borrow_mut();
            // Otherwise we are the parent node of the cube to be removed
            child.is_cube = false;
            child.children.clear();
        }

        // Fix up removed cubes
        if self.children.iter().all(|child| child.borrow().is_empty()) {
            self.children.clear();
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.size == other.size
    }
}

impl Eq for Node {}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Off,
    On,
}

#[derive(Debug, Eq, PartialEq)]
struct Step {
    command: Command,
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

fn get_middle(range: &Range<i32>) -> i32 {
    ((range.end - range.start) / 2) + range.start
}

fn get_half_range(range: &Range<i32>, use_top_half: bool) -> Range<i32> {
    let middle = get_middle(range);
    if use_top_half {
        middle..range.end
    } else {
        range.start..middle
    }
}

impl Step {
    fn new(command: Command, x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Self {
        Self { command, x, y, z }
    }

    fn parse_from_lines<I: Iterator<Item = String>>(lines: I) -> Vec<Self> {
        lines
            .map(|line| {
                let mut split = line.split(' ');
                let command = match split.next() {
                    Some("off") => Command::Off,
                    Some("on") => Command::On,
                    _ => unreachable!(),
                };

                let ranges = split.next().unwrap().split(',');
                let ranges: Vec<_> = ranges
                    .map(|range| {
                        let mut range = range.split('=').nth(1).unwrap().split("..");
                        let start = range.next().unwrap().parse().unwrap();
                        let end = range.next().unwrap().parse::<i32>().unwrap() + 1; // Add 1 to convert to exclusive range
                        start..end
                    })
                    .collect();

                Self {
                    command,
                    x: ranges[0].clone(),
                    y: ranges[1].clone(),
                    z: ranges[2].clone(),
                }
            })
            .collect()
    }

    fn get_cubes_from(
        &self,
        x: Range<i32>,
        y: Range<i32>,
        z: Range<i32>,
    ) -> Vec<Rc<RefCell<Node>>> {
        if self.x.intersection(&x).unwrap() == x
            && self.y.intersection(&y).unwrap() == y
            && self.z.intersection(&z).unwrap() == z
        {
            assert_eq!(x.end - x.start, y.end - y.start);
            assert_eq!(x.end - x.start, z.end - z.start);
            let size = x.end - x.start;
            return vec![Node::new(x.start, y.start, z.start, size)];
        }

        let mut nodes = Vec::new();

        for (use_top_x, use_top_y, use_top_z) in HALF_PERMUTATIONS {
            let half_x = get_half_range(&x, use_top_x);
            if half_x.intersection(&self.x).is_none() {
                continue;
            }

            let half_y = get_half_range(&y, use_top_y);
            if half_y.intersection(&self.y).is_none() {
                continue;
            }

            let half_z = get_half_range(&z, use_top_z);
            if half_z.intersection(&self.z).is_none() {
                continue;
            }

            nodes.append(&mut self.get_cubes_from(half_x, half_y, half_z));
        }

        nodes
    }

    fn slice_into_cubes(&self) -> Vec<Rc<RefCell<Node>>> {
        self.get_cubes_from(
            -Node::MAX_VALUE..Node::MAX_VALUE,
            -Node::MAX_VALUE..Node::MAX_VALUE,
            -Node::MAX_VALUE..Node::MAX_VALUE,
        )
    }
}

fn run_steps(steps: &[Step]) -> i32 {
    let root = Node::create_root();

    for step in steps {
        let cubes = step.slice_into_cubes();
        match step.command {
            Command::Off => {
                for cube in cubes {
                    root.borrow_mut().remove_cube(&cube.borrow());
                }
            }
            Command::On => {
                for cube in cubes {
                    root.borrow_mut().insert_cube(&cube.borrow());
                }
            }
        }
    }

    let volume = root.borrow().get_volume();
    volume
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut steps = Step::parse_from_lines(reader.lines().map(std::result::Result::unwrap));
    let steps: Vec<_> = steps
        .drain(..)
        .filter(|step| {
            step.x.start >= -50
                && step.x.end <= 50
                && step.y.start >= -50
                && step.y.end <= 50
                && step.z.start >= -50
                && step.z.end <= 50
        })
        .collect();
    println!("Volume: {}", run_steps(&steps));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_basic_example() -> [String; 4] {
        [
            String::from("on x=10..12,y=10..12,z=10..12"),
            String::from("on x=11..13,y=11..13,z=11..13"),
            String::from("off x=9..11,y=9..11,z=9..11"),
            String::from("on x=10..10,y=10..10,z=10..10"),
        ]
    }

    fn get_larger_example() -> [String; 20] {
        [
            String::from("on x=-20..26,y=-36..17,z=-47..7"),
            String::from("on x=-20..33,y=-21..23,z=-26..28"),
            String::from("on x=-22..28,y=-29..23,z=-38..16"),
            String::from("on x=-46..7,y=-6..46,z=-50..-1"),
            String::from("on x=-49..1,y=-3..46,z=-24..28"),
            String::from("on x=2..47,y=-22..22,z=-23..27"),
            String::from("on x=-27..23,y=-28..26,z=-21..29"),
            String::from("on x=-39..5,y=-6..47,z=-3..44"),
            String::from("on x=-30..21,y=-8..43,z=-13..34"),
            String::from("on x=-22..26,y=-27..20,z=-29..19"),
            String::from("off x=-48..-32,y=26..41,z=-47..-37"),
            String::from("on x=-12..35,y=6..50,z=-50..-2"),
            String::from("off x=-48..-32,y=-32..-16,z=-15..-5"),
            String::from("on x=-18..26,y=-33..15,z=-7..46"),
            String::from("off x=-40..-22,y=-38..-28,z=23..41"),
            String::from("on x=-16..35,y=-41..10,z=-47..6"),
            String::from("off x=-32..-23,y=11..30,z=-14..3"),
            String::from("on x=-49..-5,y=-3..45,z=-29..18"),
            String::from("off x=18..30,y=-20..-8,z=-3..13"),
            String::from("on x=-41..9,y=-7..43,z=-33..15"),
        ]
    }

    #[test]
    fn test_parse_basic_example() {
        let steps = Step::parse_from_lines(get_basic_example().into_iter());
        assert_eq!(steps.len(), 4);
        // The ranges look different because the problem specification uses inclusive ranges,
        // but this code assumes exclusive ranges
        assert_eq!(
            steps[0],
            Step {
                command: Command::On,
                x: 10..13,
                y: 10..13,
                z: 10..13
            }
        );
        assert_eq!(
            steps[1],
            Step {
                command: Command::On,
                x: 11..14,
                y: 11..14,
                z: 11..14
            }
        );
        assert_eq!(
            steps[2],
            Step {
                command: Command::Off,
                x: 9..12,
                y: 9..12,
                z: 9..12
            }
        );
        assert_eq!(
            steps[3],
            Step {
                command: Command::On,
                x: 10..11,
                y: 10..11,
                z: 10..11
            }
        );
    }

    #[test]
    fn test_slice() {
        let cubes = Step::new(Command::On, 0..2, 0..2, 0..2).slice_into_cubes();
        assert_eq!(cubes.len(), 1);
        assert_eq!(cubes[0], Node::new(0, 0, 0, 2));

        let cubes = Step::new(Command::On, -1..1, -1..1, -1..1).slice_into_cubes();
        assert_eq!(cubes.len(), 8);

        let cubes = Step::new(Command::On, -2..0, -2..0, -2..0).slice_into_cubes();
        assert_eq!(cubes.len(), 1);

        let cubes = Step::new(Command::On, 0..4, 0..4, 0..4).slice_into_cubes();
        assert_eq!(cubes.len(), 1);

        let cubes = Step::new(Command::On, -1..3, -1..3, -1..3).slice_into_cubes();
        assert_eq!(cubes.len(), 64 - 8 + 1);

        let cubes = Step::new(Command::On, -2..2, -2..2, -2..2).slice_into_cubes();
        assert_eq!(cubes.len(), 8);

        let cubes = Step::new(Command::On, -16..16, -16..16, -16..16).slice_into_cubes();
        assert_eq!(cubes.len(), 8);
    }

    #[test]
    fn test_insert() {
        let root = Node::create_root();

        root.borrow_mut()
            .insert_cube(&Node::new(0, 0, 0, 1).borrow());
        // assert_eq!(root.borrow().count_nodes(), );
        assert_eq!(root.borrow().count_cubes(), 1);
        assert_eq!(root.borrow().get_volume(), 1);

        root.borrow_mut()
            .insert_cube(&Node::new(0, 0, 0, 2).borrow());
        assert_eq!(root.borrow().count_cubes(), 1);
        assert_eq!(root.borrow().get_volume(), 8);

        root.borrow_mut()
            .insert_cube(&Node::new(-2, -2, -2, 2).borrow());
        assert_eq!(root.borrow().count_cubes(), 2);
        assert_eq!(root.borrow().get_volume(), 16);
    }

    #[test]
    fn test_remove() {
        let root = Node::create_root();

        root.borrow_mut()
            .insert_cube(&Node::new(0, 0, 0, 2).borrow());
        assert_eq!(root.borrow().count_cubes(), 1);
        assert_eq!(root.borrow().get_volume(), 8);

        root.borrow_mut()
            .remove_cube(&Node::new(0, 0, 0, 1).borrow());
        assert_eq!(root.borrow().count_cubes(), 7);
        assert_eq!(root.borrow().get_volume(), 7);

        root.borrow_mut()
            .insert_cube(&Node::new(0, 0, 0, 1).borrow());
        assert_eq!(root.borrow().count_cubes(), 1);
        assert_eq!(root.borrow().get_volume(), 8);
    }

    #[test]
    fn test_basic_example() {
        let steps = Step::parse_from_lines(get_basic_example().into_iter());
        assert_eq!(run_steps(&steps), 39);
    }

    #[test]
    fn test_larger_example() {
        let steps = Step::parse_from_lines(get_larger_example().into_iter());
        assert_eq!(run_steps(&steps), 590784);
    }

    // #[bench]
    // fn bench_input(b: &mut Bencher) {
    //     let file = File::open("input.txt").unwrap();
    //     let reader = BufReader::new(file);
    //     let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    //     b.iter(|| {
    //         let (algorithm, pixels) = parse_input(lines.clone().into_iter());
    //         assert_eq!(run_iterations(&algorithm, pixels, 50), 12333);
    //     });
    // }
}
