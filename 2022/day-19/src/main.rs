#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn contains(&self, other: Resources) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
}

impl std::ops::Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl std::ops::Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    #[allow(clippy::needless_pass_by_value)] // Needless, but makes the parsing cleaner
    fn parse(line: String) -> Self {
        let mut split = line
            .split(' ')
            .filter_map(|token| token.trim_end_matches(':').parse().ok());
        Self {
            id: split.next().unwrap(),
            ore_robot_cost: Resources::new(split.next().unwrap(), 0, 0, 0),
            clay_robot_cost: Resources::new(split.next().unwrap(), 0, 0, 0),
            obsidian_robot_cost: Resources::new(split.next().unwrap(), split.next().unwrap(), 0, 0),
            geode_robot_cost: Resources::new(split.next().unwrap(), 0, split.next().unwrap(), 0),
        }
    }

    fn get_max_ore_cost(&self) -> usize {
        self.ore_robot_cost
            .ore
            .max(self.clay_robot_cost.ore)
            .max(self.obsidian_robot_cost.ore)
            .max(self.geode_robot_cost.ore)
    }
}

type Inventory = Resources;
type Production = Resources;
type TimeRemaining = usize;
type CacheKey = (Inventory, Production, TimeRemaining);

struct Cache {
    geodes: HashMap<CacheKey, usize>,
    hits: usize,
    misses: usize,
}

impl Cache {
    fn new() -> Self {
        Self {
            geodes: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn get(
        &mut self,
        inventory: Inventory,
        production: Production,
        time_remaining: TimeRemaining,
    ) -> Option<usize> {
        let geodes = self
            .geodes
            .get(&(inventory, production, time_remaining))
            .copied();

        if geodes.is_some() {
            self.hits += 1;
        } else {
            self.misses += 1;
            if self.misses % 1000000 == 0 {
                self.print_stats();
            }
        }

        geodes
    }

    fn insert(
        &mut self,
        inventory: Inventory,
        production: Production,
        time_remaining: TimeRemaining,
        geodes: usize,
    ) {
        self.geodes
            .insert((inventory, production, time_remaining), geodes);
    }

    fn print_stats(&self) {
        println!(
            "Hits: {} ({:.2}%) Misses: {}",
            self.hits,
            100f32 * self.hits as f32 / (self.hits as f32 + self.misses as f32),
            self.misses,
        );
    }
}

fn count_geodes(
    cache: &mut Cache,
    blueprint: &Blueprint,
    mut inventory: Resources,
    production: Resources,
    time_remaining: usize,
) -> usize {
    if time_remaining == 1 {
        return (inventory + production).geode;
    }

    inventory.ore = inventory
        .ore
        .min(time_remaining * blueprint.get_max_ore_cost());
    inventory.clay = inventory
        .clay
        .min(time_remaining * blueprint.obsidian_robot_cost.clay);

    if let Some(geodes) = cache.get(inventory, production, time_remaining) {
        return geodes;
    }

    // Default case of building no new factories
    let mut geodes = count_geodes(
        cache,
        blueprint,
        inventory + production,
        production,
        time_remaining - 1,
    );

    if inventory.contains(blueprint.ore_robot_cost) && production.ore < blueprint.get_max_ore_cost()
    {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.ore_robot_cost,
            production + Resources::new(1, 0, 0, 0),
            time_remaining - 1,
        ));
    }

    if inventory.contains(blueprint.clay_robot_cost)
        && production.clay < blueprint.obsidian_robot_cost.clay
    {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.clay_robot_cost,
            production + Resources::new(0, 1, 0, 0),
            time_remaining - 1,
        ));
    }

    if inventory.contains(blueprint.obsidian_robot_cost) {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.obsidian_robot_cost,
            production + Resources::new(0, 0, 1, 0),
            time_remaining - 1,
        ));
    }

    if inventory.contains(blueprint.geode_robot_cost) {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.geode_robot_cost,
            production + Resources::new(0, 0, 0, 1),
            time_remaining - 1,
        ));
    }

    cache.insert(inventory, production, time_remaining, geodes);

    geodes
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    let blueprints: Vec<_> = lines.map(Blueprint::parse).collect();

    let quality_sum: usize = blueprints
        .iter()
        .map(|blueprint| {
            println!("Processing {}", blueprint.id);
            let mut cache = Cache::new();
            let quality_level = blueprint.id
                * count_geodes(
                    &mut cache,
                    blueprint,
                    Resources::new(0, 0, 0, 0),
                    Resources::new(1, 0, 0, 0),
                    24,
                );
            cache.print_stats();
            quality_level
        })
        .sum();
    println!("Quality sum: {quality_sum}");

    let mut cache = Cache::new();
    let geodes = count_geodes(
        &mut cache,
        &blueprints[2],
        Resources::new(0, 0, 0, 0),
        Resources::new(1, 0, 0, 0),
        32,
    );
    println!("Geodes: {geodes}");
    cache.print_stats();
}
