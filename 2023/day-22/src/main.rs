#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Brick {
    x: RangeInclusive<u16>,
    y: RangeInclusive<u16>,
    z: RangeInclusive<u16>,
}

impl Brick {
    fn parse(string: &str) -> Self {
        let mut parts = string.split('~');
        let mut components = parts
            .next()
            .unwrap()
            .split(',')
            .map(|component| component.parse().unwrap())
            .zip(
                parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|component| component.parse().unwrap()),
            )
            .map(|(start, end)| start..=end);
        Self {
            x: components.next().unwrap(),
            y: components.next().unwrap(),
            z: components.next().unwrap(),
        }
    }

    fn plan_cubes(&self) -> Vec<(u16, u16)> {
        self.x
            .clone()
            .flat_map(|x| self.y.clone().map(move |y| (x, y)))
            .collect()
    }

    fn set_bottom(&mut self, bottom: u16) {
        let height = self.z.end() - self.z.start();
        self.z = bottom..=bottom + height;
    }

    fn top_cubes(&self) -> Vec<(u16, u16, u16)> {
        self.plan_cubes()
            .iter()
            .map(|(x, y)| (*x, *y, *self.z.end()))
            .collect()
    }

    fn bottom_cubes(&self) -> Vec<(u16, u16, u16)> {
        self.plan_cubes()
            .iter()
            .map(|(x, y)| (*x, *y, *self.z.start()))
            .collect()
    }
}

#[derive(Debug)]
struct Tower {
    width: u16,
    top: Vec<(u16, Option<usize>)>,
    supporters: HashMap<usize, HashSet<usize>>,
    essential: HashSet<usize>,
    seen: usize,
}

impl Tower {
    fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            top: vec![(0, None); usize::from(width * height)],
            supporters: HashMap::new(),
            essential: HashSet::new(),
            seen: 0,
        }
    }

    fn cell(&self, x: u16, y: u16) -> &(u16, Option<usize>) {
        &self.top[usize::from(y * self.width) + usize::from(x)]
    }

    fn cell_mut(&mut self, x: u16, y: u16) -> &mut (u16, Option<usize>) {
        &mut self.top[usize::from(y * self.width) + usize::from(x)]
    }

    fn get_bottom(&mut self, brick: &Brick) -> u16 {
        brick
            .plan_cubes()
            .iter()
            .map(|(x, y)| self.cell(*x, *y).0 + 1)
            .max()
            .unwrap()
    }

    fn place(&mut self, brick: &Brick, index: usize) {
        self.seen += 1;

        let mut supporters = HashSet::new();

        for cube in brick.bottom_cubes() {
            let (x, y, z) = cube;
            let (height, below) = *self.cell(x, y);
            if height == z - 1 && below.is_some() {
                supporters.insert(below.unwrap());
                self.supporters
                    .entry(index)
                    .and_modify(|supporters| {
                        supporters.insert(below.unwrap());
                    })
                    .or_insert_with(|| HashSet::from([below.unwrap()]));
            }
        }

        if supporters.len() == 1 {
            self.essential.insert(*supporters.iter().next().unwrap());
        }

        for cube in brick.top_cubes() {
            let (x, y, z) = cube;
            *self.cell_mut(x, y) = (z, Some(index));
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut bricks = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| Brick::parse(&line))
        .collect::<Vec<_>>();

    bricks.sort_unstable_by_key(|brick| *brick.z.start());

    let x_range = bricks.iter().skip(1).fold(bricks[0].x.clone(), |acc, e| {
        (*acc.start()).min(*e.x.start())..=(*acc.end()).max(*e.x.end())
    });

    let y_range = bricks.iter().skip(1).fold(bricks[0].y.clone(), |acc, e| {
        (*acc.start()).min(*e.y.start())..=(*acc.end()).max(*e.y.end())
    });

    let mut tower = Tower::new(
        x_range.end() - x_range.start() + 1,
        y_range.end() - y_range.start() + 1,
    );

    for (index, mut brick) in bricks.into_iter().enumerate() {
        let bottom = tower.get_bottom(&brick);
        brick.set_bottom(bottom);
        tower.place(&brick, index);
    }

    let redundant = tower.seen - tower.essential.len();
    println!("redundant: {redundant}");
}
