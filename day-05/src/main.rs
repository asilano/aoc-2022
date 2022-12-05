use std::fs;

use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

type Yard = Vec<Vec<char>>;

fn parse_chests(data: &str) -> Yard {
    let mut lines = data.lines().rev();
    let identifiers = lines.next().unwrap().split_whitespace();
    let pile_count = identifiers.rev().next().unwrap().parse::<usize>().unwrap();

    let mut piles = vec![Vec::<char>::new(); pile_count];

    for level in lines {
        let mut chars = level.chars();
        for pile in 0..pile_count {
            let chest = chars.nth(1).unwrap();
            if chest.is_alphabetic() { piles[pile].push(chest) }
            chars.nth(1);
        }
    }

    piles
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize
}
impl Instruction {
    pub fn execute_9000(&self, piles: &mut Yard) {
        let mut to_pile = Vec::<char>::new();
        {
            let from_pile = &mut piles[self.from];
            for _ in 0..self.count {
               to_pile.push(from_pile.pop().unwrap());
            }
        }
        piles[self.to].append(&mut to_pile);
    }
    pub fn execute_9001(&self, piles: &mut Yard) {
        let mut to_pile = Vec::<char>::new();
        {
            let from_pile = &mut piles[self.from];
            for _ in 0..self.count {
               to_pile.push(from_pile.pop().unwrap());
            }
        }
        let mut revd_pile = to_pile.into_iter().rev().collect();
        piles[self.to].append(&mut revd_pile);
    }
}

fn parse_instructions(data: &str) -> Vec<Instruction> {
    let pattern = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    data.lines().map(|line| {
        let parts = pattern.captures(line).unwrap();
        let count = parts.name("count").unwrap().as_str().parse::<usize>().unwrap();
        let from = parts.name("from").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = parts.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1;
        Instruction { from, to, count }
    }).collect()
}

fn part_one(piles: &mut Yard, routine: &Vec<Instruction>) {
    for instruction in routine {
        instruction.execute_9000(piles);
    }

    for pile in piles {
        println!("{}", pile.pop().unwrap());
    }
}

fn part_two(piles: &mut Yard, routine: &Vec<Instruction>) {
    for instruction in routine {
        instruction.execute_9001(piles);
    }

    for pile in piles {
        println!("{}", pile.pop().unwrap());
    }
}

fn main() {
    let data = load_data();
    let (chests, instructions) = data.split_once("\n\n").unwrap();
    let piles = parse_chests(chests);
    let routine = parse_instructions(instructions);

    let mut p1_work = piles.clone();
    part_one(&mut p1_work, &routine);
    println!("");
    let mut p2_work = piles.clone();
    part_two(&mut p2_work, &routine);
}
