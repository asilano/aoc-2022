use std::{fs, collections::{HashSet, VecDeque}};

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}
impl Point {
    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1
        }
    }
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1
        }
    }
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z
        }
    }
    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z
        }
    }
    fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z
        }
    }
    fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![self.up(), self.down(), self.north(), self.south(), self.east(), self.west()]
    }
}

fn part_one(data: &String) -> (i32, (i32, i32, i32)) {
    let mut cubes = HashSet::<Point>::new();
    let mut faces = 0i32;

    for line in data.lines() {
        faces += 6; // new cube
        
        let (x_str, remain) = line.split_once(",").unwrap();
        let (y_str, z_str) = remain.split_once(",").unwrap();
        let x = x_str.parse::<i32>().unwrap();
        let y = y_str.parse::<i32>().unwrap();
        let z = z_str.parse::<i32>().unwrap();
        let new_cube = Point { x, y, z };
        for neighbour in new_cube.neighbours() {
            if cubes.contains(&neighbour) { faces -= 2 }
        }

        cubes.insert(new_cube);
    }

    let max_x = cubes.iter().max_by_key(|c| c.x).unwrap().x;
    let max_y = cubes.iter().max_by_key(|c| c.y).unwrap().y;
    let max_z = cubes.iter().max_by_key(|c| c.z).unwrap().z;

    (faces, (max_x, max_y, max_z))
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum FaceDirection {
    Up,
    Down,
    North,
    South,
    East,
    West
}
type Face = (Point, FaceDirection);
use FaceDirection::*;

fn part_two(data: &String, extents: (i32, i32, i32)) -> i32 {
    let mut cubes = HashSet::<Point>::new();
    let mut faces = 0i32;

    for line in data.lines() {
        let (x_str, remain) = line.split_once(",").unwrap();
        let (y_str, z_str) = remain.split_once(",").unwrap();
        let x = x_str.parse::<i32>().unwrap();
        let y = y_str.parse::<i32>().unwrap();
        let z = z_str.parse::<i32>().unwrap();
        let new_cube = Point { x, y, z };

        cubes.insert(new_cube);
    }
    
    let mut outside = HashSet::<Point>::new();
    let mut searchfront = VecDeque::<Point>::new();
    searchfront.push_back(Point { x: -1, y: -1, z: -1 });

    while !searchfront.is_empty() {
        let consider = searchfront.pop_front().unwrap();
        if outside.contains(&consider) { continue; }
        if cubes.contains(&consider) { continue; }
        if consider.x < -1 || consider.y < -1 || consider.z < -1 { continue; }
        if consider.x > extents.0 + 1 || consider.y > extents.1 + 1 || consider.z > extents.2 + 1 { continue; }
        let neighbours = consider.neighbours();

        for n in neighbours {
            if cubes.contains(&n) { 
                faces += 1; 
            } else {
                searchfront.push_back(n.clone());
            }
        }
        outside.insert(consider.clone());
    }
    faces
}

fn main() {
    let data = load_data();
    let (surface, extents) = part_one(&data);
    println!("Part one: {}", surface);

    let surface = part_two(&data, extents);
    println!("Part two: {}", surface);
}
