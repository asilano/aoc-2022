use std::{fs, collections::VecDeque};

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(Clone, Debug)]
struct Location {
    coords: (usize, usize),
    height: u32,
    dist_from_s: Option<usize>,
    from: Option<(usize, usize)>,
    to: Option<(usize, usize)>
}
type Heightmap = Vec<Vec<Location>>;

fn make_heightmap(data: &String) -> (Heightmap, (usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0,0);
    let mut end: (usize, usize) = (0,0);
    let heightmap = data.lines().enumerate().map(|(y, row)| {
        row.chars().enumerate().map(|(x, mut height)| {
            let mut dist: Option<usize> = None;
            if height == 'S' { start = (x, y); height = 'a'; dist = Some(0); }
            if height == 'E' { end = (x, y); height = 'z'; }
            Location {
                coords: (x, y),
                height: height as u32,
                dist_from_s: dist,
                from: None,
                to: None
            }
        }).collect()
    }).collect();

    (heightmap, start, end)
}

fn part_one(heightmap: &mut Heightmap, start: (usize, usize), end: (usize, usize)) -> usize {
    let width = heightmap.first().unwrap().len();
    let height = heightmap.len();
    let mut searchfront: VecDeque<(usize, usize)> = VecDeque::new();
    let mut current = start;
    let mut current_height = 'a' as u32;
    let mut current_dist = 0;
    while current != end {
        // North
        if current.1 > 0 {
            let x = current.0;
            let y = current.1 - 1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height <= current_height + 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }
        // South
        if current.1 < height - 1 {
            let x = current.0;
            let y = current.1 + 1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height <= current_height + 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }
        // East
        if current.0 > 0 {
            let x = current.0 - 1;
            let y = current.1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height <= current_height + 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }
        // West
        if current.0 < width - 1 {
            let x = current.0 + 1;
            let y = current.1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height <= current_height + 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }

        current = searchfront.pop_front().unwrap();
        current_height = heightmap.get(current.1).unwrap().get(current.0).unwrap().height;
        current_dist = heightmap.get(current.1).unwrap().get(current.0).unwrap().dist_from_s.unwrap();
    }
    current_dist
}

fn part_two(heightmap: &mut Heightmap, start: (usize, usize), end: (usize, usize)) -> usize {
    let width = heightmap.first().unwrap().len();
    let height = heightmap.len();

    heightmap.get_mut(start.1).unwrap().get_mut(start.0).unwrap().dist_from_s = None;
    heightmap.get_mut(end.1).unwrap().get_mut(end.0).unwrap().dist_from_s = Some(0);

    let mut searchfront: VecDeque<(usize, usize)> = VecDeque::new();
    searchfront.push_back(end);
    let mut min_dist = 1000;
    while !searchfront.is_empty() {
        let current = searchfront.pop_front().unwrap();
        let current_height = heightmap.get(current.1).unwrap().get(current.0).unwrap().height;
        let current_dist = heightmap.get(current.1).unwrap().get(current.0).unwrap().dist_from_s.unwrap();
        
        if current_height == 'a' as u32 && current_dist < min_dist { min_dist = current_dist; }
        
        // North
        if current.1 > 0 {
            let x = current.0;
            let y = current.1 - 1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height >= current_height - 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }
        // South
        if current.1 < height - 1 {
            let x = current.0;
            let y = current.1 + 1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height >= current_height - 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }
        // East
        if current.0 > 0 {
            let x = current.0 - 1;
            let y = current.1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height >= current_height - 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }
        // West
        if current.0 < width - 1 {
            let x = current.0 + 1;
            let y = current.1;
            if heightmap.get(y).unwrap().get(x).unwrap().dist_from_s == None {
            if heightmap.get(y).unwrap().get(x).unwrap().height >= current_height - 1 {
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().from = Some(current);
                heightmap.get_mut(y).unwrap().get_mut(x).unwrap().dist_from_s = Some(current_dist + 1);
                searchfront.push_back((x, y));
            }
            }
        }

    }
    min_dist
}
fn main() {
    let data = load_data();
    let (heightmap, start, end) = make_heightmap(&data);
    let mut heightmap_clone = heightmap.clone();
    let dist = part_one(&mut heightmap_clone, start, end);
    println!("Part one: {}", dist);

    if false {
    for row in heightmap_clone {
        for cell in row {
            match cell.from {
                None => print!("."),
                Some((x, y)) => {
                    if x as i32 == cell.coords.0 as i32 - 1 { print!("<"); }
                    else if x == cell.coords.0 + 1 { print!(">"); }
                    else if y as i32 == cell.coords.1 as i32 - 1 { print!("^"); }
                    else if y == cell.coords.1 + 1 { print!("v"); }
                }
            }
        }
        println!("");
    }
    }
    
    let mut heightmap_clone = heightmap.clone();
    let dist = part_two(&mut heightmap_clone, start, end);
    println!("Part two: {}", dist);
    for row in heightmap_clone {
        for cell in row {
            match cell.from {
                None => print!("."),
                Some((x, y)) => {
                    if x as i32 == cell.coords.0 as i32 - 1 { print!("<"); }
                    else if x == cell.coords.0 + 1 { print!(">"); }
                    else if y as i32 == cell.coords.1 as i32 - 1 { print!("^"); }
                    else if y == cell.coords.1 + 1 { print!("v"); }
                }
            }
        }
        println!("");
    }
}
