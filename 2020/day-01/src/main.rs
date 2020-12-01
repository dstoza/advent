use std::{env, fs::File, io::{BufRead, BufReader}};

fn sum_product(array: Vec<i32>) -> i32 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        return;
    }

    let filename = &args[0];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut array = Vec::<i32>::new();
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            println!("bytes 0");
            break;
        }

        println!("Parsing {}", &line);

        if let Ok(integer) = line.as_str().parse::<i32>() {
            array.push(integer);
        }
    }

    for i in &array {
        println!("{}", i);
    }
    println!("Result: {}", sum_product(array));
}
