#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct AdapterChainer {
    adapters: Vec<usize>,
}

impl AdapterChainer {
    fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }

    fn add_adapter(&mut self, adapter: usize) {
        self.adapters.push(adapter);
    }

    fn get_difference_product(&mut self) -> i32 {
        self.adapters.sort_unstable();

        // Add the first and last differences
        let mut differences = vec![0, 0, 1];
        differences[self.adapters[0] - 1] += 1;

        for window in self.adapters.windows(2) {
            differences[window[1] - window[0] - 1] += 1;
        }

        differences[0] * differences[2]
    }

    fn get_arrangement_count(&mut self) -> usize {
        let back = self.adapters[self.adapters.len() - 1];
        // Add one value outside the range [1,3] to break out of the inner loop below
        self.adapters.push(back + 4);
        self.adapters.reverse();
        // Add the implicit 0 for the outlet
        self.adapters.push(0);

        let mut arrangements = Vec::new();
        arrangements.resize(self.adapters.len(), 0_usize);
        // This is the final adapter, which always hooks directly to the device
        arrangements[1] = 1;

        for index in 1..self.adapters.len() {
            for offset in 1..=3 {
                if self.adapters[index - offset] - self.adapters[index] < 4 {
                    arrangements[index] += arrangements[index - offset];
                } else {
                    break;
                }
            }
        }

        arrangements[arrangements.len() - 1]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut chainer = AdapterChainer::new();

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        chainer.add_adapter(line.trim().parse().expect("Failed to parse adapter"));

        line.clear();
    }

    println!("Difference product: {}", chainer.get_difference_product());
    println!("Arrangements: {}", chainer.get_arrangement_count())
}
