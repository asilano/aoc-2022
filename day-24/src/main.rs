use std::{fs, collections::{HashMap, HashSet}};
use num::integer::lcm;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;
const WALL: u8 = 16;
type Timeslice = HashMap<(usize, usize), u8>;

fn parse_data(data: &String) -> (Timeslice, usize, usize) {
    let mut first = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            if space != '.' {
                first.insert((x, y), match space {
                    '#' => WALL,
                    '^' => UP,
                    '>' => RIGHT,
                    'v' => DOWN,
                    '<' => LEFT,
                    _ => unreachable!()
                });
            }
        }
    }

    (first, data.lines().count() - 2, data.lines().nth(0).unwrap().chars().count() - 2)
}

fn generate_timeslices(mut working: Timeslice, height: usize, width: usize) -> Vec<Timeslice> {
    let num_slices = lcm(height, width);

    let mut timeslices = Vec::<Timeslice>::new();
    timeslices.push(working.clone());

    for _ in 1..num_slices {
        let mut next_slice = Timeslice::new();
        for (coord, space) in working {
            if space == WALL { next_slice.insert(coord, WALL);
            } else {
                if (space & UP) == UP {
                    let next_coord = (coord.0, if coord.1 == 1 { height } else { coord.1 - 1 });
                    if !next_slice.contains_key(&next_coord) {
                        next_slice.insert(next_coord, UP);
                    } else {
                        *next_slice.get_mut(&next_coord).unwrap() |= UP;
                    }
                }
                if (space & DOWN) == DOWN {
                    let next_coord = (coord.0, if coord.1 == height { 1 } else { coord.1 + 1 });
                    if !next_slice.contains_key(&next_coord) {
                        next_slice.insert(next_coord, DOWN);
                    } else {
                        *next_slice.get_mut(&next_coord).unwrap() |= DOWN;
                    }
                }
                if (space & LEFT) == LEFT {
                    let next_coord = (if coord.0 == 1 { width } else { coord.0 - 1 }, coord.1);
                    if !next_slice.contains_key(&next_coord) {
                        next_slice.insert(next_coord, LEFT);
                    } else {
                        *next_slice.get_mut(&next_coord).unwrap() |= LEFT;
                    }
                }
                if (space & RIGHT) == RIGHT {
                    let next_coord = (if coord.0 == width { 1 } else { coord.0 + 1 }, coord.1);
                    if !next_slice.contains_key(&next_coord) {
                        next_slice.insert(next_coord, RIGHT);
                    } else {
                        *next_slice.get_mut(&next_coord).unwrap() |= RIGHT;
                    }
                }
            }
        }
        timeslices.push(next_slice.clone());
        working = next_slice;
    }

    timeslices
}

fn print_slice(timeslice: &Timeslice, height: usize, width: usize) {
    for y in 0..=height + 1 {
        for x in 0..=width + 1 {
            print!("{}", match timeslice.get(&(x, y)) {
                None => ' ',
                Some(&WALL) => '#',
                Some(&UP) => '^',
                Some(&RIGHT) => '>',
                Some(&DOWN) => 'v',
                Some(&LEFT) => '<',
                Some(_) => 'O'
            });
        }
        println!("");
    }
}

fn part_one(timeslices: &Vec<Timeslice>, start_step: usize, height: usize, width: usize) -> usize {
    let mut steps = 0usize;
    let num_slices = timeslices.len();
    let mut searchfront: Vec<(usize, usize)> = vec![(1, 0)];
    let mut visited = HashSet::<(usize, (usize, usize))>::new();
    visited.insert((start_step % num_slices, (1, 0)));

    for timeslice in timeslices.iter().cycle().skip(start_step + 1) {
        steps += 1;
        
        let mut next_searchfront = Vec::<(usize, usize)>::new();
        for (now_x, now_y) in searchfront {
            // UP
            if now_y > 0 && !timeslice.contains_key(&(now_x, now_y - 1)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x, now_y - 1))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x, now_y - 1)));
                    next_searchfront.push((now_x, now_y - 1));
                }
            }
            // DOWN
            if !timeslice.contains_key(&(now_x, now_y + 1)) {
                if now_y == height {
                    // This solves the maze!
                    return steps;
                }

                if !visited.contains(&((steps + start_step) % num_slices, (now_x, now_y + 1))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x, now_y + 1)));
                    next_searchfront.push((now_x, now_y + 1));
                }
            }
            // LEFT
            if !timeslice.contains_key(&(now_x - 1, now_y)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x - 1, now_y))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x - 1, now_y)));
                    next_searchfront.push((now_x - 1, now_y));
                }
            }
            // RIGHT
            if !timeslice.contains_key(&(now_x + 1, now_y)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x + 1, now_y))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x + 1, now_y)));
                    next_searchfront.push((now_x + 1, now_y));
                }
            }
            // WAIT
            if !timeslice.contains_key(&(now_x, now_y)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x, now_y))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x, now_y)));
                    next_searchfront.push((now_x, now_y));
                }
            }
        }

        searchfront = next_searchfront;
        if searchfront.is_empty() {
            panic!("Empty searchfront at step {}", steps);
        }
    }

    unreachable!();
}

fn part_two(timeslices: &Vec<Timeslice>, start_step: usize, height: usize, width: usize) -> usize {
    let mut steps = 0usize;
    let num_slices = timeslices.len();
    let mut searchfront: Vec<(usize, usize)> = vec![(width, height + 1)];
    let mut visited = HashSet::<(usize, (usize, usize))>::new();
    visited.insert((start_step % num_slices, (width, height + 1)));

    for timeslice in timeslices.iter().cycle().skip(start_step + 1) {
        steps += 1;
        
        let mut next_searchfront = Vec::<(usize, usize)>::new();
        for (now_x, now_y) in searchfront {
            // DOWN
            if now_y <= height && !timeslice.contains_key(&(now_x, now_y + 1)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x, now_y + 1))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x, now_y + 1)));
                    next_searchfront.push((now_x, now_y + 1));
                }
            }
            // UP
            if !timeslice.contains_key(&(now_x, now_y - 1)) {
                if now_y == 1 {
                    // This solves the maze!
                    return steps;
                }

                if !visited.contains(&((steps + start_step) % num_slices, (now_x, now_y - 1))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x, now_y - 1)));
                    next_searchfront.push((now_x, now_y - 1));
                }
            }
            // LEFT
            if !timeslice.contains_key(&(now_x - 1, now_y)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x - 1, now_y))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x - 1, now_y)));
                    next_searchfront.push((now_x - 1, now_y));
                }
            }
            // RIGHT
            if !timeslice.contains_key(&(now_x + 1, now_y)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x + 1, now_y))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x + 1, now_y)));
                    next_searchfront.push((now_x + 1, now_y));
                }
            }
            // WAIT
            if !timeslice.contains_key(&(now_x, now_y)) {
                if !visited.contains(&((steps + start_step) % num_slices, (now_x, now_y))) {
                    visited.insert(((steps + start_step) % num_slices, (now_x, now_y)));
                    next_searchfront.push((now_x, now_y));
                }
            }
        }

        searchfront = next_searchfront;
        if searchfront.is_empty() {
            panic!("Empty searchfront at step {}", steps);
        }
    }

    unreachable!();
}
fn main() {
    let data = load_data();
    let (first, height, width) = parse_data(&data);
    let timeslices = generate_timeslices(first, height, width);

    let steps = part_one(&timeslices, 0, height, width);
    println!("Part one: {}", steps);

    let extra_steps_back = part_two(&timeslices, steps, height, width);
    println!("Part two: extra {} steps. Total: {}", extra_steps_back, steps + extra_steps_back);
    let extra_steps_forward = part_one(&timeslices, steps + extra_steps_back, height, width);
    println!("Part two: extra {} & {} steps. Total: {}", extra_steps_back, extra_steps_forward, steps + extra_steps_back + extra_steps_forward);
}
