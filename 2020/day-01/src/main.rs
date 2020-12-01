use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn sum_product2(sorted: &[i32], target: i32) -> i32 {
    let mut candidate_index = sorted.len() - 1;
    for number in sorted {
        while number + sorted[candidate_index] > target {
            candidate_index -= 1;
        }

        if number + sorted[candidate_index] == target {
            return number * sorted[candidate_index];
        }
    }
    -1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut array = Vec::<i32>::new();
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let integer = line
            .trim()
            .parse::<i32>()
            .expect(format!("Failed to parse {}", &line).as_str());
        array.push(integer);

        line.clear();
    }

    array.sort();
    println!("Result: {}", sum_product2(&array, 2020));
}
