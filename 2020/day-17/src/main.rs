#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct PocketDimension {
    dimensions: u32,
    side_length: usize,
    margin: usize,
    cubes: Vec<bool>,
}

impl PocketDimension {
    fn address_helper(side_length: usize, x: usize, y: usize, z: usize, w: usize) -> usize {
        w * side_length * side_length * side_length
            + z * side_length * side_length
            + y * side_length
            + x
    }

    fn get_address(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        PocketDimension::address_helper(self.side_length, x, y, z, w)
    }

    fn new(dimensions: u32, iterations: usize, initial_state: &[String]) -> Self {
        let mut cubes = Vec::new();
        let margin = iterations + 1;
        let side_length = initial_state.len() + margin * 2;
        cubes.resize(side_length * side_length * side_length * side_length, false);

        for y in 0..initial_state.len() {
            let line = &initial_state[y];
            for x in 0..initial_state.len() {
                let cube = match line.as_bytes()[x] {
                    b'#' => true,
                    b'.' => false,
                    _ => panic!("Unexpected byte {}", line.as_bytes()[x]),
                };
                let w = match dimensions {
                    3 => 0,
                    4 => margin,
                    _ => panic!("Unexpected dimensionality {}", dimensions),
                };
                cubes[PocketDimension::address_helper(side_length, x + margin, y + margin, margin, w)] = cube;
            }
        }

        Self {
            dimensions,
            side_length,
            margin,
            cubes,
        }
    }

    fn count_active_neighbors(
        &self,
        center_x: usize,
        center_y: usize,
        center_z: usize,
        center_w: usize,
    ) -> u32 {
        let mut count = 0;

        let w_range = match self.dimensions {
            3 => center_w..=center_w,
            4 => center_w - 1..=center_w + 1,
            _ => panic!("Unexpected dimensionality {}", self.dimensions),
        };

        for w in w_range {
            for z in center_z - 1..=center_z + 1 {
                for y in center_y - 1..=center_y + 1 {
                    for x in center_x - 1..=center_x + 1 {
                        if x == center_x && y == center_y && z == center_z && w == center_w {
                            continue;
                        }

                        if self.cubes[self.get_address(x, y, z, w)] {
                            count += 1;
                            if count >= 4 {
                                return count;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn simulate(&mut self) {
        let mut changes = Vec::new();

        let range = self.margin - 1..self.side_length - self.margin;
        self.margin -= 1;

        let w_range = match self.dimensions {
            3 => 0..1,
            4 => range.clone(),
            _ => panic!("Unexpected dimensionality {}", self.dimensions),
        };

        for w in w_range {
            for z in range.clone() {
                for y in range.clone() {
                    for x in range.clone() {
                        let address = self.get_address(x, y, z, w);
                        if self.cubes[address] {
                            let active_neighbors = self.count_active_neighbors(x, y, z, w);
                            if !(2..=3).contains(&active_neighbors) {
                                changes.push(address);
                            }
                        } else if self.count_active_neighbors(x, y, z, w) == 3 {
                            changes.push(address);
                        }
                    }
                }
            }
        }

        for change in changes {
            self.cubes[change] ^= true;
        }
    }

    fn get_active_count(&self) -> u32 {
        self.cubes
            .iter()
            .map(|active| if *active { 1 } else { 0 })
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut initial_state = Vec::new();

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        initial_state.push(String::from(line.trim()));

        line.clear();
    }

    let dimensions: u32 = args[2].parse().expect("Failed to parse dimensionality");

    let iterations = 6;
    let mut pocket_dimension = PocketDimension::new(dimensions, iterations, &initial_state);
    for _ in 0..iterations {
        pocket_dimension.simulate();
    }
    println!("Active cubes: {}", pocket_dimension.get_active_count());
}
