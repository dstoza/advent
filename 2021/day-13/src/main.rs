use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Eq, PartialEq)]
enum Command {
    FoldAlongX(u16),
    FoldAlongY(u16),
}

fn parse_input<I: Iterator<Item = String>>(mut lines: I) -> (Vec<(u16, u16)>, Vec<Command>) {
    let coordinates = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|coordinate_line| {
            let mut split = coordinate_line.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let commands = lines
        .map(|command_line| {
            let mut split = command_line.split_whitespace().nth(2).unwrap().split('=');
            let direction = split.next().unwrap();
            let value: u16 = split.next().unwrap().parse().unwrap();
            match direction {
                "x" => Command::FoldAlongX(value),
                "y" => Command::FoldAlongY(value),
                _ => unreachable!(),
            }
        })
        .collect();

    (coordinates, commands)
}

fn execute_command(coordinates: &mut [(u16, u16)], command: &Command) {
    match command {
        Command::FoldAlongX(value) => {
            for (x, _y) in coordinates {
                if *x > *value {
                    *x = value * 2 - *x;
                }
            }
        }
        Command::FoldAlongY(value) => {
            for (_x, y) in coordinates {
                if *y > *value {
                    *y = value * 2 - *y;
                }
            }
        }
    }
}

fn count_dots(coordinates: &mut [(u16, u16)], commands: &[Command]) -> usize {
    for command in commands {
        execute_command(coordinates, command)
    }

    let unique_dots: HashSet<_> = coordinates.iter().cloned().collect();
    unique_dots.len()
}

fn print_dots(coordinates: &[(u16, u16)]) {
    let coordinates: HashSet<_> = coordinates.iter().cloned().collect();

    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in &coordinates {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    for row in 0..=max_y {
        for column in 0..=max_x {
            print!(
                "{}",
                if coordinates.contains(&(column, row)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let (mut coordinates, commands) = parse_input(reader.lines().map(|line| line.unwrap()));
    // println!("Dots: {}", count_dots(&mut coordinates, &commands[0..1]));
    for command in commands {
        execute_command(&mut coordinates, &command);
    }
    print_dots(&coordinates);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_simple() -> [String; 21] {
        [
            String::from("6,10"),
            String::from("0,14"),
            String::from("9,10"),
            String::from("0,3"),
            String::from("10,4"),
            String::from("4,11"),
            String::from("6,0"),
            String::from("6,12"),
            String::from("4,1"),
            String::from("0,13"),
            String::from("10,12"),
            String::from("3,4"),
            String::from("3,0"),
            String::from("8,4"),
            String::from("1,10"),
            String::from("2,14"),
            String::from("8,10"),
            String::from("9,0"),
            String::new(),
            String::from("fold along y=7"),
            String::from("fold along x=5"),
        ]
    }

    #[test]
    fn test_parse_input() {
        let (coordinates, commands) = parse_input(get_simple().into_iter());
        assert_eq!(
            coordinates,
            vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0)
            ]
        );
        assert_eq!(
            commands,
            vec![Command::FoldAlongY(7), Command::FoldAlongX(5)]
        )
    }

    #[test]
    fn test_execute_command() {
        let mut coordinates = vec![(5, 5)];
        execute_command(&mut coordinates, &Command::FoldAlongY(4)); // Should now be at (5, 3)
        execute_command(&mut coordinates, &Command::FoldAlongX(3));
        assert_eq!(coordinates[0], (1, 3));
    }

    #[test]
    fn test_count_dots_simple() {
        let (mut coordinates, commands) = parse_input(get_simple().into_iter());
        assert_eq!(count_dots(&mut coordinates, &commands[0..1]), 17)
    }
}
