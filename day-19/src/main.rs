use std::{fs, collections::HashMap, ops::Sub, cmp::{Ordering, max}};

use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct ResourceSet {
    ore: u32,
    clay: u32,
    obsidian: u32
}
impl ResourceSet {
    fn new(cost: &str) -> Self {
        let ore_rex = Regex::new(r"(\d+) ore").unwrap();
        let clay_rex = Regex::new(r"(\d+) clay").unwrap();
        let obsidian_rex = Regex::new(r"(\d+) obsidian").unwrap();

        let ore = if let Some(caps) = ore_rex.captures(cost) {
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
        } else {
            0
        };
        let clay = if let Some(caps) = clay_rex.captures(cost) {
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
        } else {
            0
        };
        let obsidian = if let Some(caps) = obsidian_rex.captures(cost) {
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
        } else {
            0
        };

        Self { 
            ore,
            clay,
            obsidian
        }
    }
}
impl PartialOrd for ResourceSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ore_cmp = self.ore.cmp(&other.ore);
        let clay_cmp = self.clay.cmp(&other.clay);
        let obsidian_cmp = self.obsidian.cmp(&other.obsidian);
        
        if ore_cmp == Ordering::Equal && clay_cmp == Ordering::Equal && obsidian_cmp == Ordering::Equal {
            Some(Ordering::Equal)
        }
        else if ore_cmp != Ordering::Greater && clay_cmp != Ordering::Greater && obsidian_cmp != Ordering::Greater {
            Some(Ordering::Less)
        }
        else if ore_cmp != Ordering::Less && clay_cmp != Ordering::Less && obsidian_cmp != Ordering::Less {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
impl Sub for &ResourceSet {
    type Output = ResourceSet;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}

struct Blueprint {
    index: usize,
    orebot: ResourceSet,
    claybot: ResourceSet,
    obsidbot: ResourceSet,
    geodebot: ResourceSet
}
impl Blueprint {
    fn new(info: &str) -> Self {
        let rex = Regex::new(r"Blueprint (?P<index>\d+): Each ore robot costs (?P<orebotCost>.*). Each clay robot costs (?P<claybotCost>.*). Each obsidian robot costs (?P<obsidbotCost>.*). Each geode robot costs (?P<geodebotCost>.*).").unwrap();
        let caps = rex.captures(info).unwrap();
        let index = caps.name("index").unwrap().as_str().parse::<usize>().unwrap();
        let orebot = ResourceSet::new(caps.name("orebotCost").unwrap().as_str());
        let claybot = ResourceSet::new(caps.name("claybotCost").unwrap().as_str());
        let obsidbot = ResourceSet::new(caps.name("obsidbotCost").unwrap().as_str());
        let geodebot = ResourceSet::new(caps.name("geodebotCost").unwrap().as_str());
        
        Self {
            index,
            orebot,
            claybot,
            obsidbot,
            geodebot
        }
    }

    fn max_needed(&self) -> ResourceSet {
        let ore = max(self.orebot.ore, max(self.claybot.ore, max(self.obsidbot.ore, self.geodebot.ore)));
        let clay = max(self.orebot.clay, max(self.claybot.clay, max(self.obsidbot.clay, self.geodebot.clay)));
        let obsidian = max(self.orebot.obsidian, max(self.claybot.obsidian, max(self.obsidbot.obsidian, self.geodebot.obsidian)));

        ResourceSet { ore, clay, obsidian }
    }
}

type BotCount = (u32, u32, u32, u32);

fn part_one(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.iter().map(|bp| {
        let mut best_geodes = 0u32;
        let resources = ResourceSet { ore: 0, clay: 0, obsidian: 0 };
        let mut seen_state = HashMap::<ResourceSet, (BotCount, u32)>::new();
        let max_needed = bp.max_needed();
        find_most_geodes(&bp, resources, 24, 1, 0, 0, 0, &mut best_geodes, 0, &mut seen_state, &max_needed);
        println!("Blueprint {}, max geodes {}", bp.index, best_geodes);
        best_geodes * bp.index as u32
    }).sum()
}

fn part_two(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.iter().take(3).map(|bp| {
        let mut best_geodes = 0u32;
        let resources = ResourceSet { ore: 0, clay: 0, obsidian: 0 };
        let mut seen_state = HashMap::<ResourceSet, (BotCount, u32)>::new();
        let max_needed = bp.max_needed();
        find_most_geodes(&bp, resources, 32, 1, 0, 0, 0, &mut best_geodes, 0, &mut seen_state, &max_needed);
        println!("Blueprint {}, max geodes {}", bp.index, best_geodes);
        best_geodes
    }).product()
}

fn find_most_geodes(blueprint: &Blueprint, resources: ResourceSet, ticks_left: u32, orebots: u32, claybots: u32, obsidbots: u32, geodebots: u32, best_geodes: &mut u32, geodes_now: u32, seen_state: &mut HashMap<ResourceSet, (BotCount, u32)>, max_needed: &ResourceSet) {
    if geodes_now > *best_geodes { *best_geodes = geodes_now; }
    if ticks_left == 0 { return; }

    // The absolute bound on geodes producible from here is if we start producing only geode bots
    // at one per tick. That will produce tick(tick-1)/2 geodes, plus the tick*geodebots from the
    // ones we already have. If that doesn't beat the best so far, then we may as well stop.
    let upper_bound = geodes_now + geodebots * ticks_left + ticks_left * (ticks_left - 1) / 2;
    if upper_bound <= *best_geodes { return; }

    if let Some(state) = seen_state.get_mut(&resources) {
        // If we've seen this state before, at least as early with at least as many bots, we
        // can skip out. We won't improve.
        let state_bots = state.0;
        if state_bots.0 >= orebots &&
            state_bots.1 >= claybots &&
                state_bots.2 >= obsidbots &&
                state_bots.3 >= geodebots && 
                state.1 >= ticks_left {
            // println!("Skipping due to state");
            return;
        } else {
            state.0 = (orebots, claybots, obsidbots, geodebots);
            state.1 = ticks_left;
        }
    } else {
        seen_state.insert(resources.clone(), ((orebots, claybots, obsidbots, geodebots), ticks_left));
    }

    if resources >= blueprint.orebot && orebots < max_needed.ore {
        let mut new_resources = &resources - &blueprint.orebot;
        new_resources.ore += orebots;
        new_resources.clay += claybots;
        new_resources.obsidian += obsidbots;
        //println!("Tick {}. Build orebot. Now {} ore, {} clay, {} obs, {} geo; {} orebots, {} claybots, {} obsbots, {} geobots",
        //         25-ticks_left, new_resources.ore, new_resources.clay, new_resources.obsidian, geodes_now + geodebots, orebots + 1, claybots, obsidbots, geodebots, seen_state);
        find_most_geodes(blueprint, new_resources, ticks_left - 1, orebots + 1, claybots, obsidbots, geodebots, best_geodes, geodes_now + geodebots, seen_state, max_needed);
    }
    if resources >= blueprint.claybot && claybots < max_needed.clay {
        let mut new_resources = &resources - &blueprint.claybot;
        new_resources.ore += orebots;
        new_resources.clay += claybots;
        new_resources.obsidian += obsidbots;
        //println!("Tick {}. Build claybot. Now {} ore, {} clay, {} obs, {} geo; {} orebots, {} claybots, {} obsbots, {} geobots",
        //         25-ticks_left, new_resources.ore, new_resources.clay, new_resources.obsidian, geodes_now + geodebots, orebots, claybots + 1, obsidbots, geodebots, seen_state);
        find_most_geodes(blueprint, new_resources, ticks_left - 1, orebots, claybots + 1, obsidbots, geodebots, best_geodes, geodes_now + geodebots, seen_state, max_needed);
    }
    if resources >= blueprint.obsidbot && obsidbots < max_needed.obsidian {
        let mut new_resources = &resources - &blueprint.obsidbot;
        new_resources.ore += orebots;
        new_resources.clay += claybots;
        new_resources.obsidian += obsidbots;
        //println!("Tick {}. Build obsidbot. Now {} ore, {} clay, {} obs, {} geo; {} orebots, {} claybots, {} obsbots, {} geobots",
        //         25-ticks_left, new_resources.ore, new_resources.clay, new_resources.obsidian, geodes_now + geodebots, orebots, claybots, obsidbots+1, geodebots, seen_state);
        find_most_geodes(blueprint, new_resources, ticks_left - 1, orebots, claybots, obsidbots + 1, geodebots, best_geodes, geodes_now + geodebots, seen_state, max_needed);
    }
    if resources >= blueprint.geodebot {
        let mut new_resources = &resources - &blueprint.geodebot;
        new_resources.ore += orebots;
        new_resources.clay += claybots;
        new_resources.obsidian += obsidbots;
        //println!("Tick {}. Build geodebot. Now {} ore, {} clay, {} obs, {} geo; {} orebots, {} claybots, {} obsbots, {} geobots",
        //         25-ticks_left, new_resources.ore, new_resources.clay, new_resources.obsidian, geodes_now + geodebots, orebots, claybots, obsidbots, geodebots + 1);
        find_most_geodes(blueprint, new_resources, ticks_left - 1, orebots, claybots, obsidbots, geodebots + 1, best_geodes, geodes_now + geodebots, seen_state, max_needed);
    }
    let mut new_resources = resources.clone();
    new_resources.ore += orebots;
    new_resources.clay += claybots;
    new_resources.obsidian += obsidbots;
        //println!("Tick {}. Build nothing. Now {} ore, {} clay, {} obs, {} geo; {} orebots, {} claybots, {} obsbots, {} geobots",
        //         25-ticks_left, new_resources.ore, new_resources.clay, new_resources.obsidian, geodes_now + geodebots, orebots, claybots, obsidbots, geodebots, seen_state);
    find_most_geodes(blueprint, new_resources, ticks_left - 1, orebots, claybots, obsidbots, geodebots, best_geodes, geodes_now + geodebots, seen_state, max_needed);
}

fn main() {
    let data = load_data();
    let blueprints = data.lines().map(|l| Blueprint::new(l)).collect::<Vec<Blueprint>>();

    let quality_sum = part_one(&blueprints);
    println!("Part one: {}", quality_sum);
    let product = part_two(&blueprints);
    println!("Part two: {}", product);
}
