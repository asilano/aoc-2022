use std::{fs, collections::HashMap, time::SystemTime};

use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

type Arena = HashMap<(usize, usize), char>;

fn parse_map(data: &String) -> (Arena, (usize, usize), usize) {
    let mut arena = Arena::new();
    for line in data.lines() {
        let mut endpoints = line.split(" -> ");
        let start_str = endpoints.next().unwrap();
        let (start_x_str, start_y_str) = start_str.split_once(",").unwrap();
        let mut start_x = start_x_str.parse::<usize>().unwrap();
        let mut start_y = start_y_str.parse::<usize>().unwrap();
        for end_str in endpoints {
            let (end_x_str, end_y_str) = end_str.split_once(",").unwrap();
            let end_x = end_x_str.parse::<usize>().unwrap();
            let end_y = end_y_str.parse::<usize>().unwrap();

            if start_x < end_x {
                for x in start_x..=end_x { arena.insert((x, start_y), '#'); }
            } else if start_x > end_x {
                for x in end_x..=start_x { arena.insert((x, start_y), '#'); }
            } else if start_y < end_y {
                for y in start_y..=end_y { arena.insert((start_x, y), '#'); }
            } else if start_y > end_y {
                for y in end_y..=start_y { arena.insert((start_x, y), '#'); }
            }

            start_x = end_x;
            start_y = end_y;
        }
    }

    let min_x = arena.keys().map(|(x, _y)| x).min().unwrap().clone();
    let max_x = arena.keys().map(|(x, _y)| x).max().unwrap().clone();
    let max_y = arena.keys().map(|(_x, y)| y).max().unwrap().clone();

    (arena, (min_x, max_x), max_y)
}

fn part_one(arena: &mut Arena, max_y: usize) -> usize {
    let drop_x = 500;
    let drop_y = 0;
    let mut carry_on = true;
    while carry_on {
        let mut x = drop_x;
        let mut y = drop_y;
        loop {
            if !arena.contains_key(&(x, y+1)) {
                // Drop down
                y += 1;
            } else if !arena.contains_key(&(x-1, y+1)) {
                // Drop left
                x -= 1;
                y += 1;
            } else if !arena.contains_key(&(x+1, y+1)) {
                // Drop right
                x += 1;
                y += 1;
            } else {
                arena.insert((x, y), 'o');
                break;
            }

            if y>max_y {
                carry_on = false;
                break;
            }
        }
    }

    arena.iter().filter(|(_, &c)| c == 'o').count()
}

fn part_two(arena: &mut Arena, max_y: usize) -> usize {
    let drop_x = 500;
    let drop_y = 0;
    let mut carry_on = true;
    while carry_on {
        let mut x = drop_x;
        let mut y = drop_y;
        loop {
            if !arena.contains_key(&(x, y+1)) && y != max_y + 1{
                // Drop down
                y += 1;
            } else if !arena.contains_key(&(x-1, y+1)) && y != max_y + 1{
                // Drop left
                x -= 1;
                y += 1;
            } else if !arena.contains_key(&(x+1, y+1)) && y != max_y + 1{
                // Drop right
                x += 1;
                y += 1;
            } else {
                arena.insert((x, y), 'o');
                if (x,y) == (500, 0) { carry_on = false; }
                break;
            }
        }
    }

    arena.iter().filter(|(_, &c)| c == 'o').count()
}
fn part_two_dynamic(arena: &mut Arena, max_y: usize) -> usize {
    let mut last_path: Vec<(usize, usize)> = vec![(500,0)];
    let mut carry_on = true;
    while carry_on {
        let mut x = last_path.last().unwrap().0;
        let mut y = last_path.last().unwrap().1;
        loop {
            if !arena.contains_key(&(x, y+1)) && y != max_y + 1{
                // Drop down
                y += 1;
            } else if !arena.contains_key(&(x-1, y+1)) && y != max_y + 1{
                // Drop left
                x -= 1;
                y += 1;
            } else if !arena.contains_key(&(x+1, y+1)) && y != max_y + 1{
                // Drop right
                x += 1;
                y += 1;
            } else {
                arena.insert((x, y), 'o');
                last_path.pop();
                if (x,y) == (500, 0) { carry_on = false; assert!(last_path.is_empty()); }
                break;
            }

            last_path.push((x, y));
        }
    }

    arena.iter().filter(|(_, &c)| c == 'o').count()
}
fn timeit<F: Fn() -> T, T>(f: F) -> T {
  let start = SystemTime::now();
  let result = f();
  let end = SystemTime::now();
  let duration = end.duration_since(start).unwrap();
  println!("it took {} milliseconds", duration.as_millis());
  result
}
fn main() {
    let data = load_data();
    let (mut arena, (min_x, max_x), max_y) = parse_map(&data);

    let count = part_one(&mut arena.clone(), max_y);
//    for y in 0..=max_y {
//        for x in min_x..=max_x {
//            if let Some(c) = arena.get(&(x,y)) {
//                print!("{}", c);
//            } else {
//                print!(".");
//            }
//        }
//        println!("");
//    }
    println!("Part one {}", count);

    let count = timeit(|| part_two(&mut arena.clone(), max_y));
    println!("Part two {}", count);
    let count = timeit(|| part_two_dynamic(&mut arena.clone(), max_y));
    println!("Part two dynamic {}", count);
}

