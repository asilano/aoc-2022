use std::{fs, iter::Cycle, collections::{HashSet, HashMap}, cmp::max};

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum RockType {
    Horiz,
    Plus,
    Jay,
    Vert,
    Square
}
use RockType::*;
static ROCK_TYPES: &[RockType] = &[RockType::Horiz, RockType::Plus, RockType::Jay, RockType::Vert, RockType::Square];
impl RockType {
    fn cycle() -> impl Iterator<Item = &'static Self> {
        ROCK_TYPES.iter().cycle()
    }

    fn collision(&self, position: &(usize, usize), fallen: &HashSet<(usize, usize)>) -> bool {
        let right_limit: usize = match self {
            Horiz => 5,
            Plus | Jay => 6,
            Vert => 8,
            Square => 7
        };
        position.0 <= 0 || position.0 >= right_limit || position.1 <= 0 || self.cells(position).iter().any(|c| fallen.contains(c))
    }

    fn cells(&self, position: &(usize, usize)) -> Vec<(usize, usize)> {
        match self {
            Horiz => vec![*position, (position.0 + 1, position.1), (position.0 + 2, position.1), (position.0 + 3, position.1)],
            Plus => vec![(position.0, position.1 + 1), (position.0 + 1, position.1), (position.0 + 1, position.1 + 1), (position.0 + 1, position.1 + 2), (position.0 + 2, position.1 + 1)],
            Jay => vec![*position, (position.0 + 1, position.1), (position.0 + 2, position.1), (position.0 + 2, position.1 + 1), (position.0 + 2, position.1 + 2),],
            Vert => vec![*position, (position.0, position.1 + 1), (position.0, position.1 + 2), (position.0, position.1 + 3)],
            Square => vec![*position, (position.0, position.1 + 1), (position.0 + 1, position.1), (position.0 + 1, position.1 +1)]
        }
    }
}

fn part_one(data: &String) -> usize {
    let mut jets = data.chars().filter(|&c| c == '<' || c == '>').cycle();
    let mut rocks = RockType::cycle();
    let mut fallen = HashSet::<(usize, usize)>::new();
    let mut pinnacle = 0usize;

    for _ in 0..2022 {
        let rock = rocks.next().unwrap();
        let mut pos = (3usize, pinnacle + 4);
        loop {
            let jet = jets.next().unwrap();
            let tentative_pos = match jet {
                '<' => (pos.0 - 1, pos.1),
                '>' => (pos.0 + 1, pos.1),
                _ => { println!("{:?}", jet); unreachable!() }
            };

            if !rock.collision(&tentative_pos, &fallen) {
                pos = tentative_pos;

            }
            // Drop
            let tentative_pos = (pos.0, pos.1 - 1);
            if !rock.collision(&tentative_pos, &fallen) {
                pos = tentative_pos;
            } else {
                let fixed_cells = rock.cells(&pos);
                pinnacle = max(pinnacle, fixed_cells.iter().map(|c| c.1).max().unwrap());
                for cell in fixed_cells { fallen.insert(cell); }
                //println!("{:?}", pinnacle);

                break;
            }
        }
    }
    pinnacle
}

fn part_two(data: &String) -> usize {
    let mut jets = data.chars().filter(|&c| c == '<' || c == '>').enumerate().cycle();
    let mut rocks = RockType::cycle();
    let mut fallen = HashSet::<(usize, usize)>::new();
    let mut pinnacle = 0usize;
    let mut skipped_dist = 0usize;
    let mut jet_ix: usize;
    let mut rock_count = 0;
    let mut skipped = false;

    let mut visited_positions = HashMap::<(Vec<(usize, usize)>, usize, RockType), (usize, usize)>::new();

    while rock_count < 1000000000000 {
        rock_count += 1;
        let rock = rocks.next().unwrap();
        let mut pos = (3usize, pinnacle + 4);
        loop {
            let jet_pair = jets.next().unwrap();
            jet_ix = jet_pair.0;
            let jet = jet_pair.1;
            let tentative_pos = match jet {
                '<' => (pos.0 - 1, pos.1),
                '>' => (pos.0 + 1, pos.1),
                _ => { println!("{:?}", jet); unreachable!() }
            };

            if !rock.collision(&tentative_pos, &fallen) {
                pos = tentative_pos;

            }
            // Drop
            let tentative_pos = (pos.0, pos.1 - 1);
            if !rock.collision(&tentative_pos, &fallen) {
                pos = tentative_pos;
            } else {
                let fixed_cells = rock.cells(&pos);
                pinnacle = max(pinnacle, fixed_cells.iter().map(|c| c.1).max().unwrap());
                for cell in fixed_cells { fallen.insert(cell); }

                break;
            }
        }

        if !skipped {
            for y in 0..=pinnacle {
                if [1usize,2,3,4,5,6,7].iter().all(|&x| fallen.contains(&(x, y)) || fallen.contains(&(x, y + 1))) {
                    // Collect a vector of fallen blocks above the new floor level, as if the new floor
                    // were 0.
                    let mut above_floor = fallen.iter().filter(|&(_,fally)| *fally >= y).map(|&(fallx, fally)| (fallx, fally - y)).collect::<Vec<(usize, usize)>>();
                    above_floor.sort();
                    // Store and compare states - shape above false floor, index in jet array, rock
                    // type; store the last height this floor was seen at, and how many rocks had
                    // fallen
                    let key = (above_floor.clone(), jet_ix, *rock);
                    if let Some((floor_height, seen_at_count)) = visited_positions.get(&key) {
                        let repeat_dist = y - floor_height;
                        let rocks_to_go = 1000000000000usize - rock_count;
                        let rocks_between = rock_count - seen_at_count;
                        let iterations = rocks_to_go / rocks_between;

                        rock_count += rocks_between * iterations;
                        skipped_dist = repeat_dist * iterations;
                        skipped = true;
                    } else {
                        visited_positions.insert(key, (y, rock_count));
                   }
                }
            }
        }
    }
    pinnacle + skipped_dist
}

fn main() {
    let data = load_data();
    let height = part_one(&data);
    println!("Part one {}", height);
    let height = part_two(&data);
    println!("Part two {}", height);
}
