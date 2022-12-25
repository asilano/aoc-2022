use std::{fs, collections::{VecDeque, HashSet, HashMap}};

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn parse_data(data: &String) -> HashSet<(i32, i32)> {
    let mut elves = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' { elves.insert((x as i32, y as i32)); }
        }
    }

    elves
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}
use Direction::*;

fn part_one_and_two(elves: &mut HashSet<(i32, i32)>) {
    let num_elves = elves.len();
    let mut proposals = VecDeque::from_iter(&[North, South, West, East]);
    let mut steps = 0;

    loop {
        let mut proposed = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();
        elves.iter().for_each(|&(elf_x, elf_y)| {
            let mut proposal = (elf_x, elf_y);
            if      [(elf_x - 1, elf_y - 1), (elf_x, elf_y - 1), (elf_x + 1, elf_y - 1),
                     (elf_x - 1, elf_y),                         (elf_x + 1, elf_y),
                     (elf_x - 1, elf_y + 1), (elf_x, elf_y + 1), (elf_x + 1, elf_y + 1)].iter().any(|c| elves.contains(&c)) {
                for dir in proposals.clone() {
                    match dir {
                        North => {
                            if ![(elf_x - 1, elf_y - 1), (elf_x, elf_y - 1), (elf_x + 1, elf_y - 1)].iter().any(|c| elves.contains(&c)) {
                                proposal = (elf_x, elf_y - 1);
                                break;
                            }
                        },
                        South => {
                            if ![(elf_x - 1, elf_y + 1), (elf_x, elf_y + 1), (elf_x + 1, elf_y + 1)].iter().any(|c| elves.contains(&c)) {
                                proposal = (elf_x, elf_y + 1);
                                break;
                            }
                        },
                        East => {
                            if ![(elf_x + 1, elf_y - 1), (elf_x + 1, elf_y), (elf_x + 1, elf_y + 1)].iter().any(|c| elves.contains(&c)) {
                                proposal = (elf_x + 1, elf_y);
                                break;
                            }
                        },
                        West => {
                            if ![(elf_x - 1, elf_y - 1), (elf_x - 1, elf_y), (elf_x - 1, elf_y + 1)].iter().any(|c| elves.contains(&c)) {
                                proposal = (elf_x - 1, elf_y);
                                break;
                            }
                        },
                    }
                }
            }

            if proposal != (elf_x, elf_y) { 
                if let Some(clashes) = proposed.get_mut(&proposal) {
                    clashes.push((elf_x, elf_y));
                } else {
                    proposed.insert(proposal, vec![(elf_x, elf_y)]);
                }
            }
        });

        if proposed.is_empty() {
            println!("Part two: {}", steps + 1);
            break;
        }
        
        for (new_pos, clashes) in proposed {
            if clashes.len() == 1 {
                elves.remove(clashes.first().unwrap());
                elves.insert(new_pos.clone());
            }
        }

        let shift = proposals.pop_front().unwrap();
        proposals.push_back(shift);

        steps += 1;
        if steps == 10 {
            let min_x = elves.iter().min_by_key(|(x, _)| x).unwrap().0;
            let max_x = elves.iter().max_by_key(|(x, _)| x).unwrap().0;
            let min_y = elves.iter().min_by_key(|(_, y)| y).unwrap().1;
            let max_y = elves.iter().max_by_key(|(_, y)| y).unwrap().1;

            println!("Part one: {}", (max_x - min_x + 1) * (max_y - min_y + 1) - num_elves as i32);
        }
    }
}

fn main() {
    let data = load_data();
    let mut elves = parse_data(&data);
    part_one_and_two(&mut elves);
}
