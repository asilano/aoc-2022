use std::{fs, fmt};
use std::collections::VecDeque;
use itertools::Itertools;
use regex::Regex;
use num::integer::{gcd, lcm};

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64, bool, u64) -> u64>,
    action: Box<dyn Fn(u64) -> usize>,
    inspected_count: usize,
    modulus: u64
}
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
         .field("Modulus", &self.modulus)
         .finish()
    }
}
impl Monkey {
    fn from_input(input: &str, id: usize) -> Self {
        let mut lines = input.lines().skip(1);
        
        let items_rex = Regex::new(r"^\s*Starting items: ([\d, ]*)$").unwrap();
        let item_parts = items_rex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().split(", ");
        let items = item_parts.map(|item| item.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();

        let op_rex = Regex::new(r"\s*Operation: new = old ([+*]) (\d+|old)$").unwrap();
        let caps = op_rex.captures(lines.next().unwrap()).unwrap();
        let oper = caps.get(1).unwrap().as_str().to_string();
        let opand = caps.get(2).unwrap().as_str().to_string();
        let operation = Box::new(move |old: u64, part_one: bool, modulus: u64| -> u64 {
            let opand_val = match opand.as_str() {
               "old" => old,
               digits => digits.parse::<u64>().unwrap(),
            };

            let pre_bored = match oper.as_str() {
                "+" => old + opand_val,
                "*" => old * opand_val,
                _ => unreachable!()
            };

            if part_one {
                pre_bored / 3
            } else {
                pre_bored % modulus
            }
        });

        let test_rex = Regex::new(r"\s*Test: divisible by (\d*)$").unwrap();
        let divisible = test_rex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap();
        let true_monkey = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap() as usize;
        let false_monkey = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap() as usize;
        let action = Box::new(move |worry: u64| -> usize {
            if worry % divisible == 0 { true_monkey } else { false_monkey }
        });

        Self {
            id,
            items,
            operation,
            action,
            inspected_count: 0,
            modulus: divisible
        }
    }

    fn inspect_item(&mut self, worry: u64, part_one: bool, modulus: u64) -> (u64, usize) {
        self.inspected_count += 1;
        let new_worry = (self.operation)(worry, part_one, modulus);
        let next_monkey = (self.action)(new_worry);
        (new_worry, next_monkey)
    }
}

fn part_one(monkeys: &mut Vec<Monkey>) -> usize {
    let monkey_count = monkeys.len();
    for _ in 0..20 {
        for ix in 0..monkey_count {
            while !monkeys.get(ix).unwrap().items.is_empty() {
                let item = monkeys.get_mut(ix).unwrap().items.pop_front().unwrap();
                let (new_item, next_monkey) = monkeys.get_mut(ix).unwrap().inspect_item(item, true, 1);
                monkeys.get_mut(next_monkey).unwrap().items.push_back(new_item);
            }
        }
    }
    let max_monkey = monkeys.iter().max_by_key(|m| m.inspected_count).unwrap();
    let max_monkey2 = monkeys.iter().max_by_key(|m| if m.id == max_monkey.id { 0 } else { m.inspected_count }).unwrap();
    max_monkey.inspected_count * max_monkey2.inspected_count
}

fn part_two(monkeys: &mut Vec<Monkey>, modulus: u64) -> usize {
    let monkey_count = monkeys.len();
    for _ in 0..10000 {
        for ix in 0..monkey_count {
            while !monkeys.get(ix).unwrap().items.is_empty() {
                let item = monkeys.get_mut(ix).unwrap().items.pop_front().unwrap();
                let (new_item, next_monkey) = monkeys.get_mut(ix).unwrap().inspect_item(item, false, modulus);
                monkeys.get_mut(next_monkey).unwrap().items.push_back(new_item);
            }
        }
    }
    let max_monkey = monkeys.iter().max_by_key(|m| m.inspected_count).unwrap();
    let max_monkey2 = monkeys.iter().max_by_key(|m| if m.id == max_monkey.id { 0 } else { m.inspected_count }).unwrap();
    max_monkey.inspected_count * max_monkey2.inspected_count
}

fn main() {
    let data = load_data();
    let mut monkeys: Vec<Monkey> = data.split("\n\n").enumerate().map(|(ix, chunk)| Monkey::from_input(chunk, ix)).collect();
    let monkeyness = part_one(&mut monkeys);
    println!("Part one: {}", monkeyness);
    let mut monkeys: Vec<Monkey> = data.split("\n\n").enumerate().map(|(ix, chunk)| Monkey::from_input(chunk, ix)).collect();
    let modulus = monkeys.iter().map(|m| m.modulus).
        reduce(|acc, modls| lcm(acc,modls)).unwrap();
    println!("{:?}", monkeys);
    let monkeyness = part_two(&mut monkeys, modulus);
    println!("Part two: {}", monkeyness);
}

