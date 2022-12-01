use std::fs;
use itertools::Itertools;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn parse_data(data: &String) -> Vec<i64> {
    let elves = data.split("\n\n");
    elves.map(|rows| 
        rows.lines().map(|numstr|
            numstr.parse::<i64>().unwrap()).
        sum()
    ).collect()
}

fn part_one(elves_calories: &Vec<i64>) -> i64 {
    *elves_calories.iter().max().unwrap()
}

fn part_two(elves_calories: &Vec<i64>) -> i64 {
    elves_calories.into_iter().sorted().rev().take(3).sum()
}

fn main() {
    let data = load_data();
    let elves_calories = parse_data(&data);
    let part_one = part_one(&elves_calories);
    println!("Part one: {}", part_one);
    let part_two = part_two(&elves_calories);
    println!("Part two: {}", part_two);
}
