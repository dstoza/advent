use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn sum_product2(sorted: &[i32], target: i32) -> Option<i32> {
    let mut candidate_index = sorted.len() - 1;
    for number in sorted {
        while number + sorted[candidate_index] > target {
            if candidate_index == 0 {
                return None;
            }

            candidate_index -= 1;
        }

        if number + sorted[candidate_index] == target {
            return Some(number * sorted[candidate_index]);
        }
    }

    None
}

fn sum_product3(sorted: &[i32], target: i32) -> Option<i32> {
    let mut end = sorted.len() - 1;
    for number in sorted {
        while number + sorted[end] > target {
            end -= 1;
        }

        if let Some(product2) = sum_product2(&sorted[0..end], target - number) {
            return Some(product2 * number);
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
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

    let mode = &args[2];
    array.sort();
    let result = match mode.as_str() {
        "2" => sum_product2(&array, 2020),
        "3" => sum_product3(&array, 2020),
        _ => {
            println!("Expected mode '2' or '3'");
            None
        }
    };
    
    println!("Result: {}", result.expect("Failed to find sum product"));
}
