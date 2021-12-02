use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn naive_position(
    (distance, depth, _aim): (i32, i32, i32),
    (direction, value): (u8, i32),
) -> (i32, i32, i32) {
    match direction {
        b'd' => (distance, depth + value, 0), // down
        b'u' => (distance, depth - value, 0), // up
        _ => (distance + value, depth, 0),    // forward
    }
}

fn position_with_aim(
    (distance, depth, aim): (i32, i32, i32),
    (direction, value): (u8, i32),
) -> (i32, i32, i32) {
    match direction {
        b'd' => (distance, depth, aim + value),            // down
        b'u' => (distance, depth, aim - value),            // up
        _ => (distance + value, depth + value * aim, aim), // forward
    }
}

fn compute_position<I: Iterator<Item = String>>(commands: I, use_aim: bool) -> i32 {
    let (distance, depth, _aim) = commands
        .map(|command| {
            let mut split = command.split(' ');
            (
                split.next().unwrap().as_bytes()[0],
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold(
            (0, 0, 0),
            if use_aim {
                position_with_aim
            } else {
                naive_position
            },
        );
    distance * depth
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    println!(
        "Position: {}",
        compute_position(reader.lines().map(|line| line.unwrap()), true)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_commands() {
        let commands = [
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];
        assert_eq!(compute_position(commands.clone().into_iter(), false), 150);
        assert_eq!(compute_position(commands.into_iter(), true), 900);
    }
}
