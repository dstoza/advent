use std::io;

fn checksum<I>(stream: I) -> i32
where
    I: Iterator<Item = i32>,
{
    let min_max = stream.fold((i32::max_value(), i32::min_value()), |acc, x| {
        (std::cmp::min(acc.0, x), std::cmp::max(acc.1, x))
    });
    min_max.1 - min_max.0
}

fn divisible(line: &str) -> i32 {
    let mut seen: Vec<i32> = vec![];
    for text in line.split_whitespace() {
        let number: i32 = text.parse().expect("Expected an integer");
        for s in seen.as_slice() {
            if *s > number && s % number == 0 {
                return s / number;
            } else if number > *s && number % s == 0 {
                return number / s;
            }
        }
        seen.push(number);
    }
    0
}

fn main() {
    let mut input = String::new();
    let mut sum = 0;
    while io::stdin().read_line(&mut input).is_ok() {
        {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                break;
            }

            let stream = trimmed
                .split_whitespace()
                .map(|t| t.parse::<i32>().expect("Expected an integer"));
            sum += checksum(stream);

            // sum += divisible(trimmed);
        }
        input.clear();
    }
    println!("{}", sum);
}
