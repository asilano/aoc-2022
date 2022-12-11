use std::fs;
use std::{thread, time};
use std::io::{stdout, Write};
// use itertools::Itertools;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

enum Operation {
    Noop,
    Addx(i32)
}

fn parse_data(data: String) -> Vec<Operation> {
    data.lines().map(|line| {
        match &line[0..4] {
            "noop" => Operation::Noop,
            "addx" => Operation::Addx(line[5..].parse::<i32>().unwrap()),
            _ => unreachable!()
        }
    }).collect()
}

fn parts_one_and_two(operations: &Vec<Operation>) -> i32 {
    let mut register_x: i32 = 1;
    let mut strength: i32 = 0;
    let mut cycle: i32 = 0;
    let pause = time::Duration::from_millis(10);

    for op in operations {
        let mut crd = false;
        if (register_x - cycle % 40).abs() <= 1 {
            print!("#");
        } else {
            print!(" ");
        }
        std::io::stdout().flush().unwrap();
        thread::sleep(pause);
        cycle += 1;
        if (cycle % 40) == 20 {
            strength += cycle * register_x;
        }
        if cycle % 40 == 0 {
            println!("");
            crd = true;
        }
        match op {
            Operation::Noop => {},
            Operation::Addx(val) => {
                if (register_x - cycle % 40).abs() <= 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
                std::io::stdout().flush().unwrap();
                thread::sleep(pause);
                cycle += 1;
                if (cycle % 40) == 20 {
                    strength += cycle * register_x;
                }
                register_x += val;
            }
        }
        if !crd && cycle % 40 == 0 {
            println!("");
        }
    }

    strength
}

fn main() {
    let data = load_data();
    let operations = parse_data(data);
    let strength = parts_one_and_two(&operations);
    println!("Part one: {}", strength);
}

