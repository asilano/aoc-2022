use std::fs;
use std::collections::HashSet;

use itertools::Itertools;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn part_one(data: &String) -> u32 {
    let wrong_items = data.lines().map(|rucksack| {
        wrong_item(rucksack)
    });

    let priorities = wrong_items.map(|item| { priority(&item) });
    //println!("{:?}", priorities.clone().collect::<Vec<u32>>());
    priorities.sum()
}

fn wrong_item(rucksack: &str) -> char {
    let size = rucksack.len(); // Safe - we know the data is ASCII

    let pocket_one = HashSet::<_>::from_iter(rucksack.clone().chars().take(size / 2));
    let pocket_two = HashSet::<_>::from_iter(rucksack.clone().chars().skip(size / 2));

    let rogue = pocket_one.intersection(&pocket_two).next().unwrap();
    *rogue
}

fn priority(item: &char) -> u32 {
    const LOWER_A: u32 = 'a' as u32;
    const UPPER_A: u32 = 'A' as u32;
    if (*item as u32) < LOWER_A {
        (*item as u32) - UPPER_A + 27
    } else {
        (*item as u32) - LOWER_A + 1
    }
}

fn part_two(data: &String) -> u32 {
    let groups = data.lines().chunks(3);
    let badges = groups.into_iter().map(|group| {
        let elf_items = group.map(|elf| { HashSet::<char>::from_iter(elf.chars()) });
        let common_items = elf_items.reduce(|accum, elf| { 
            let isect = accum.intersection(&elf);
            isect.map(|c| *c).collect()
        });
        *common_items.unwrap().iter().next().unwrap()
    });
    
    let priorities = badges.map(|item| { priority(&item) });
    //println!("{:?}", priorities.clone().collect::<Vec<u32>>());
    priorities.sum()
}

fn main() {
    let data = load_data();
    let score = part_one(&data);
    println!("Part one score: {}", score);
    let score = part_two(&data);
    println!("Part two score: {}", score);
}
