use std::{env, fs::File, io::BufReader};

fn sum_product(array: Vec<i32>) -> i32 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        return;
    }

    let filename = &args[0];
    let mut file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let array = Vec::<i32>::new();
}
