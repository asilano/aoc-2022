use std::fs;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Blank
}

fn parse_data_flat(data: &String) -> (Vec<Vec<Tile>>, String) {
    let (plan, instructions) = data.split_once("\n\n").unwrap();
    let row_len = plan.lines().nth(0).unwrap().len();
    let mut flat_map = Vec::<Vec<Tile>>::new();
    flat_map.push(vec![Tile::Blank; row_len]);
    for line in plan.lines() {
        let mut row = vec![Tile::Blank];
        for cell in line.chars() {
            row.push(match cell {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                ' ' => Tile::Blank,
                _ => unreachable!()
            });
        }
        row.push(Tile::Blank);
        flat_map.push(row);
    };
    let row_len = flat_map.first().unwrap().len();
    flat_map.push(vec![Tile::Blank; row_len]);

    (flat_map, instructions.to_string())
}

enum Facing {
    Right,
    Down,
    Left,
    Up
}
impl Facing {
    fn to_string(&self) -> String {
        match *self {
            Facing::Right => "R",
            Facing::Left => "L",
            Facing::Up => "U",
            Facing::Down => "D"
        }.to_string()
    }
}

fn part_one(map: &Vec<Vec<Tile>>, instructions: &String) -> u64 {
    let mut position = (map.get(1).unwrap().iter().position(|t| *t == Tile::Floor).unwrap(), 1usize);
    let mut facing = Facing::Right;
    
    let mut instruct_chars = instructions.chars().peekable();
    while !instruct_chars.peek().is_none() {
        let mut steps = 0;
        let mut turn = '.';

        loop {
            match instruct_chars.next() {
                Some(d) if d.is_ascii_digit() => steps = steps * 10 + d.to_digit(10).unwrap(),
                Some(c) => { if c == 'R' || c == 'L' { turn = c; } break; }
                None => break
            };
        }

        for _ in 0..steps {
            let mut tentative = match facing {
                Facing::Right => (position.0 + 1, position.1),
                Facing::Left => (position.0 - 1, position.1),
                Facing::Up => (position.0, position.1 - 1),
                Facing::Down => (position.0, position.1 + 1)
            };

            match map.get(tentative.1).unwrap().get(tentative.0).unwrap_or(&Tile::Blank) {
                Tile::Floor => position = tentative,
                Tile::Wall => break,
                Tile::Blank => {
                    match facing {
                        Facing::Right => { 
                            tentative = (map.get(position.1).unwrap().iter().position(|t| *t != Tile::Blank).unwrap(), position.1);
                            if *map.get(tentative.1).unwrap().get(tentative.0).unwrap() == Tile::Floor {
                                position = tentative;
                            } else {
                                break;
                            }
                        }
                        Facing::Left => { 
                            tentative = (map.get(position.1).unwrap().iter().rposition(|t| *t != Tile::Blank).unwrap(), position.1);
                            if *map.get(tentative.1).unwrap().get(tentative.0).unwrap() == Tile::Floor {
                                position = tentative;
                            } else {
                                break;
                            }
                        }
                        Facing::Up => { 
                            tentative = (position.0, map.iter().rposition(|r| *r.get(position.0).unwrap_or(&Tile::Blank) != Tile::Blank).unwrap());
                            if *map.get(tentative.1).unwrap().get(tentative.0).unwrap() == Tile::Floor {
                                position = tentative;
                            } else {
                                break;
                            }
                        }
                        Facing::Down => { 
                            tentative = (position.0, map.iter().position(|r| *r.get(position.0).unwrap() != Tile::Blank).unwrap());
                            if *map.get(tentative.1).unwrap().get(tentative.0).unwrap() == Tile::Floor {
                                position = tentative;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        match turn {
            'R' => {
                facing = match facing {
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Up => Facing::Right
                }
            },
            'L' => {
                facing = match facing {
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                    Facing::Up => Facing::Left
                }
            }
            '.' => {},
            _ => { println!("{}", turn); unreachable!() }
        }

        println!("Now at {:?}, facing {}", position, facing.to_string());
    }
    let face_value: u64 = match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3
    };
    1000 * (position.1 as u64) + 4 * (position.0 as u64) + face_value
}

fn part_two(map: &Vec<Vec<Tile>>, instructions: &String) -> u64 {
    let mut position = (map.get(1).unwrap().iter().position(|t| *t == Tile::Floor).unwrap(), 1usize);
    let mut facing = Facing::Right;
    let sample = map.len() < 50;
    
    let mut instruct_chars = instructions.chars().peekable();
    while !instruct_chars.peek().is_none() {
        let mut steps = 0;
        let mut turn = '.';

        loop {
            match instruct_chars.next() {
                Some(d) if d.is_ascii_digit() => steps = steps * 10 + d.to_digit(10).unwrap(),
                Some(c) => { if c == 'R' || c == 'L' { turn = c; } break; }
                None => break
            };
        }

        for _ in 0..steps {
            let tentative = match facing {
                Facing::Right => (position.0 + 1, position.1),
                Facing::Left => (position.0 - 1, position.1),
                Facing::Up => (position.0, position.1 - 1),
                Facing::Down => (position.0, position.1 + 1)
            };

            match map.get(tentative.1).unwrap().get(tentative.0).unwrap_or(&Tile::Blank) {
                Tile::Floor => position = tentative,
                Tile::Wall => break,
                Tile::Blank => {
                    let (wrapped, wrapped_face) = next_position(&position, &facing, sample);
                    if *map.get(wrapped.1).unwrap().get(wrapped.0).unwrap() == Tile::Wall { break; }
                    position = wrapped;
                    facing = wrapped_face;
                }
            }
        }

        match turn {
            'R' => {
                facing = match facing {
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Up => Facing::Right
                }
            },
            'L' => {
                facing = match facing {
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                    Facing::Up => Facing::Left
                }
            }
            '.' => {},
            _ => { println!("{}", turn); unreachable!() }
        }

        println!("Now at {:?}, facing {}", position, facing.to_string());
    }
    let face_value: u64 = match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3
    };
    1000 * (position.1 as u64) + 4 * (position.0 as u64) + face_value
}

fn next_position(position: &(usize, usize), facing: &Facing, sample: bool) -> ((usize, usize), Facing) {
    if sample {
        match (position, facing) {
            ((x, 1), Facing::Up) => ((4 - (x - 9), 5), Facing::Down),
            ((9, y), Facing::Left) if (1..=4).contains(y) => ((4 + y, 5), Facing::Down),
            ((12, y), Facing::Right) if (1..=4).contains(y) => ((16, 13 - y), Facing::Left),
            ((1, y), Facing::Left) => ((12 + (9 - y), 12), Facing::Up),
            ((x, 5), Facing::Up) if (1..=4).contains(x) => ((13 - x, 5), Facing::Down),
            ((x, 5), Facing::Up) if (5..=8).contains(x) => ((9, x - 4), Facing::Right),
            ((12, y), Facing::Right) if (5..=8).contains(y) => ((12 + (9 - y), 9), Facing::Down),
            ((x, 8), Facing::Down) if (1..=4).contains(x) => ((8 + (5 - x), 12), Facing::Up),
            ((x, 8), Facing::Down) if (5..=8).contains(x) => ((9, 8 + (9 - x)), Facing::Right),
            ((x, 9), Facing::Up) => ((12, 9 - (x - 12)), Facing::Left),
            ((9, y), Facing::Left) if (9..=12).contains(y) => ((9 - (y - 12), 8), Facing::Up),
            ((16, y), Facing::Right) => ((12, 5 - (y - 12)), Facing::Left),
            ((x, 12), Facing::Down) if (9..=12).contains(x) => ((5 - (x - 8), 8), Facing::Up),
            ((x, 12), Facing::Down) if (13..=16).contains(x) => ((1, 9 - (x - 12)), Facing::Right),
            _ => { println!("At {:?} facing {}", position, facing.to_string()); unreachable!() }
        }
    } else {
        match (position, facing) {
            ((x, 1), Facing::Up) if (51..=100).contains(x) => ((1, 150 + (x - 50)), Facing::Right),
            ((x, 1), Facing::Up) if (101..=150).contains(x) => ((x - 100, 200), Facing::Up),
            ((51, y), Facing::Left) if (1..=50).contains(y) => ((1, 100 + (51 - y)), Facing::Right),
            ((150, y), Facing::Right) => ((100, 100 + (51 - y)), Facing::Left),
            ((x, 50), Facing::Down) => ((100, 50 + (x - 100)), Facing::Left),
            ((51, y), Facing::Left) if (51..=100).contains(y) => ((50 - (100 - y), 101), Facing::Down),
            ((100, y), Facing::Right) if (51..=100).contains(y) => ((100 + (y - 50), 50), Facing::Up),
            ((x, 101), Facing::Up) => ((51, 100 - (50 - x)), Facing::Right),
            ((1, y), Facing::Left) if (101..=150).contains(y) => ((51, 151 - y), Facing::Right),
            ((100, y), Facing::Right) if (101..=150).contains(y) => ((150, 151 - y), Facing::Left),
            ((x, 150), Facing::Down) => ((50, 151 + (x - 51)), Facing::Left),
            ((1, y), Facing::Left) if (151..=200).contains(y) => ((y - 100, 1), Facing::Down),
            ((50, y), Facing::Right) => ((y - 100, 150), Facing::Up),
            ((x, 200), Facing::Down) => ((x + 100, 1), Facing::Down),
            _ => { println!("At {:?} facing {}", position, facing.to_string()); unreachable!() }
        }
    }
}

fn main() {
    let data = load_data();
    let (flat_map, instructions) = parse_data_flat(&data);
    let password = part_one(&flat_map, &instructions);
    println!("Part one {}", password);
    let password = part_two(&flat_map, &instructions);
    println!("Part one {}", password);
}
