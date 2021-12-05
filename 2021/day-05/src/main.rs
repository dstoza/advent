use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{point, Point2, Vector2};

fn get_direction_vector(p0: Point2<i16>, p1: Point2<i16>) -> Vector2<i16> {
    let mut vector = p1 - p0;
    vector.x = vector.x.clamp(-1, 1);
    vector.y = vector.y.clamp(-1, 1);
    vector
}

fn count_overlaps<I: Iterator<Item = String>>(lines: I) -> usize {
    lines
        .map(|line| {
            let mut split = line.split(" -> ").map(|position| {
                let mut split = position
                    .split(',')
                    .map(|coord| coord.parse::<i16>().unwrap());
                point![split.next().unwrap(), split.next().unwrap()]
            });
            (split.next().unwrap(), split.next().unwrap())
        })
        // .filter(|(p0, p1)| p0.x == p1.x || p0.y == p1.y)
        .fold(HashMap::new(), |mut map, line| {
            let (mut start, end) = line;
            let direction_vector = get_direction_vector(start, end);
            while start != end {
                *map.entry(start).or_insert(0usize) += 1;
                start += direction_vector;
            }
            *map.entry(start).or_insert(0usize) += 1;
            map
        })
        .into_values()
        .filter(|value| *value >= 2)
        .count()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    println!(
        "Overlaps: {}",
        count_overlaps(reader.lines().map(|line| line.unwrap()))
    );
}

#[cfg(test)]
mod test {
    use nalgebra::vector;

    use super::*;

    fn get_sample_input() -> [String; 10] {
        [
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2"),
        ]
    }

    #[test]
    fn test_direction_vector() {
        assert_eq!(
            get_direction_vector(point![0, 0], point![-2, 2]),
            vector![-1, 1]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![-2, 0]),
            vector![-1, 0]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![-2, -2]),
            vector![-1, -1]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![0, 2]),
            vector![0, 1]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![0, 0]),
            vector![0, 0]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![0, -2]),
            vector![0, -1]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![2, 2]),
            vector![1, 1]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![2, 0]),
            vector![1, 0]
        );
        assert_eq!(
            get_direction_vector(point![0, 0], point![2, -2]),
            vector![1, -1]
        );
    }

    #[test]
    fn test_rectilinear_vents() {
        assert_eq!(count_overlaps(get_sample_input().into_iter()), 5);
    }
}
