use std::fs;
use fancy_regex::Regex;
use itertools::Itertools;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn part_one(data: &String) -> usize {
    let pattern = Regex::new(r"(.)(?!.{,2}\1)(.)(?!.?\2)(.)(?!\3).").unwrap();
    pattern.find(data).unwrap().unwrap().end()
}

fn part_two(data: &String) -> usize {
    data.as_bytes().windows(14).position(|window| {
        window.into_iter().unique().count() == 14
    }).unwrap() + 14
}

fn main() {
    let data = load_data();
    println!("Packet begin: {}", part_one(&data));
    println!("Message begin: {}", part_two(&data));
}
