use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    mem::swap,
};

struct Context<I: Iterator<Item = String>> {
    iterator: I,
    pub previous: Option<String>,
    pub current: Option<String>,
    pub next: Option<String>,
}

impl<I: Iterator<Item = String>> Context<I> {
    fn new(mut iterator: I) -> Self {
        let next = iterator.next();

        Self {
            iterator,
            previous: None,
            current: None,
            next,
        }
    }

    fn advance(&mut self) {
        swap(&mut self.previous, &mut self.current);
        swap(&mut self.current, &mut self.next);
        swap(&mut self.next, &mut self.iterator.next());
    }
}

fn get_low_point_risk_level<I: Iterator<Item = String>>(lines: I) -> i32 {
    let mut risk_level = 0;

    let mut context = Context::new(lines);
    context.advance();

    while let Some(current) = &context.current {
        for (position, sample) in current.as_bytes().iter().enumerate() {
            // Check above
            if let Some(previous) = &context.previous {
                if previous.as_bytes()[position] <= *sample {
                    continue;
                }
            }

            // Check left
            if position > 0 && *sample >= current.as_bytes()[position - 1] {
                continue;
            }

            // Check right
            if position < current.len() - 1 && *sample >= current.as_bytes()[position + 1] {
                continue;
            }

            // Check below
            if let Some(next) = &context.next {
                if next.as_bytes()[position] <= *sample {
                    continue;
                }
            }

            risk_level += 1 + (*sample - b'0') as i32;
        }

        context.advance();
    }

    risk_level
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    println!(
        "Risk level: {}",
        get_low_point_risk_level(reader.lines().map(|l| l.unwrap()))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> [String; 5] {
        [
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678"),
        ]
    }

    #[test]
    fn test_risk_level() {
        assert_eq!(get_low_point_risk_level(get_example().into_iter()), 15);
    }
}
