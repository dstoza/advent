#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Resources {
    amounts: [usize; 4],
}

impl Resources {
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Resources {
            amounts: [ore, clay, obsidian, geode],
        }
    }

    fn one(resource: Resource) -> Self {
        let mut one = Self::default();
        one.set_amount(resource, 1);
        one
    }

    fn contains(&self, other: Resources) -> bool {
        self.amounts
            .iter()
            .enumerate()
            .all(|(index, amount)| *amount >= other.amounts[index])
    }

    fn get_amount(&self, resource: Resource) -> usize {
        self.amounts[resource as usize]
    }

    fn set_amount(&mut self, resource: Resource, amount: usize) {
        self.amounts[resource as usize] = amount;
    }
}

impl std::default::Default for Resources {
    fn default() -> Self {
        Self { amounts: [0; 4] }
    }
}

impl std::ops::Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut sum = self;
        for (index, amount) in sum.amounts.iter_mut().enumerate() {
            *amount += rhs.amounts[index];
        }
        sum
    }
}

impl std::ops::Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut difference = self;
        for (index, amount) in difference.amounts.iter_mut().enumerate() {
            *amount -= rhs.amounts[index];
        }
        difference
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robot_costs: [Resources; 4],
}

impl Blueprint {
    #[allow(clippy::needless_pass_by_value)] // Needless, but makes the parsing cleaner
    fn parse(line: String) -> Self {
        let mut split = line
            .split(' ')
            .filter_map(|token| token.trim_end_matches(':').parse().ok());
        Self {
            id: split.next().unwrap(),
            robot_costs: [
                Resources::new(split.next().unwrap(), 0, 0, 0),
                Resources::new(split.next().unwrap(), 0, 0, 0),
                Resources::new(split.next().unwrap(), split.next().unwrap(), 0, 0),
                Resources::new(split.next().unwrap(), 0, split.next().unwrap(), 0),
            ],
        }
    }

    fn get_robot_cost(&self, resource: Resource) -> Resources {
        self.robot_costs[resource as usize]
    }

    fn get_max_ore_cost(&self) -> usize {
        self.robot_costs
            .iter()
            .map(|cost| cost.get_amount(Resource::Ore))
            .max()
            .unwrap()
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
            if self.misses % 100_000 == 0 {
                println!("{inventory:?} {production:?} {time_remaining}");
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

fn count_geodes_with_factory(
    cache: &mut Cache,
    blueprint: &Blueprint,
    inventory: Resources,
    production: Resources,
    time_remaining: usize,
    factory_resource: Resource,
) -> usize {
    count_geodes(
        cache,
        blueprint,
        inventory + production - blueprint.get_robot_cost(factory_resource),
        production + Resources::one(factory_resource),
        time_remaining - 1,
    )
}

fn count_geodes(
    cache: &mut Cache,
    blueprint: &Blueprint,
    mut inventory: Resources,
    production: Resources,
    time_remaining: usize,
) -> usize {
    if time_remaining == 1 {
        return (inventory + production).get_amount(Resource::Geode);
    }

    inventory.set_amount(
        Resource::Ore,
        inventory
            .get_amount(Resource::Ore)
            .min(time_remaining * blueprint.get_max_ore_cost()),
    );
    inventory.set_amount(
        Resource::Clay,
        inventory.get_amount(Resource::Clay).min(
            time_remaining
                * blueprint
                    .get_robot_cost(Resource::Obsidian)
                    .get_amount(Resource::Clay),
        ),
    );

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

    if inventory.contains(blueprint.get_robot_cost(Resource::Ore))
        && production.get_amount(Resource::Ore) < blueprint.get_max_ore_cost()
    {
        let geodes_with_ore_factory = count_geodes_with_factory(
            cache,
            blueprint,
            inventory,
            production,
            time_remaining,
            Resource::Ore,
        );
        geodes = geodes.max(geodes_with_ore_factory);
    }

    if inventory.contains(blueprint.get_robot_cost(Resource::Clay))
        && production.get_amount(Resource::Clay)
            < blueprint
                .get_robot_cost(Resource::Obsidian)
                .get_amount(Resource::Clay)
    {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.get_robot_cost(Resource::Clay),
            production + Resources::one(Resource::Clay),
            time_remaining - 1,
        ));
    }

    if inventory.contains(blueprint.get_robot_cost(Resource::Obsidian)) {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.get_robot_cost(Resource::Obsidian),
            production + Resources::one(Resource::Obsidian),
            time_remaining - 1,
        ));
    }

    if inventory.contains(blueprint.get_robot_cost(Resource::Geode)) {
        geodes = geodes.max(count_geodes(
            cache,
            blueprint,
            inventory + production - blueprint.get_robot_cost(Resource::Geode),
            production + Resources::one(Resource::Geode),
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
                    Resources::default(),
                    Resources::one(Resource::Ore),
                    24,
                );
            cache.print_stats();
            quality_level
        })
        .sum();
    println!("Quality sum: {quality_sum}");

    let geode_product: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            println!("Processing {}", blueprint.id);
            let mut cache = Cache::new();
            let geodes = count_geodes(
                &mut cache,
                blueprint,
                Resources::default(),
                Resources::one(Resource::Ore),
                32,
            );
            cache.print_stats();
            geodes
        })
        .product();
    println!("Geodes: {geode_product}");
}
