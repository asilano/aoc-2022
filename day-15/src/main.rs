use std::{fs, ops::{RangeInclusive, RangeBounds}, fmt::Debug, collections::VecDeque, cmp::{min, max}};

use itertools::Itertools;
use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

struct Sensor {
    coords: (i32, i32),
    beacon_coords: (i32, i32)
}
impl Sensor {
    fn dist_to_beacon(&self) -> u32 {
        self.coords.0.abs_diff(self.beacon_coords.0) +
        self.coords.1.abs_diff(self.beacon_coords.1)
    }
}

fn parse_data(data: &String) -> Vec<Sensor> {
    let rex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    data.lines().map(|line| {
        let caps = rex.captures(line).unwrap();
        Sensor {
            coords: (caps.get(1).unwrap().as_str().parse::<i32>().unwrap(), caps.get(2).unwrap().as_str().parse::<i32>().unwrap()),
            beacon_coords: (caps.get(3).unwrap().as_str().parse::<i32>().unwrap(), caps.get(4).unwrap().as_str().parse::<i32>().unwrap()),
        }
    }).collect()
}

fn part_one(sensors: &Vec<Sensor>, row: i32, min_x: Option<i32>, max_x: Option<i32>) -> (usize, Vec<RangeInclusive<i32>>, Vec<i32>) {
    let mut excluded_ranges: Vec<RangeInclusive<i32>> = sensors.iter().map(|sensor| {
        let dist_to_beacon = sensor.dist_to_beacon() as i32;
        let dist_to_row = sensor.coords.1.abs_diff(row) as i32;
        let remain = dist_to_beacon - dist_to_row;

        let mut start = sensor.coords.0 - remain;
        if let Some(clamp) = min_x {
            start = max(start, clamp);
        }
        let mut end = sensor.coords.0 + remain;
        if let Some(clamp) = max_x {
            end = min(end, clamp);
        }

        start..=end
    }).collect();
    let beacons_x: Vec<i32> = sensors.iter().filter(|s| s.beacon_coords.1 == row).map(|sensor| sensor.beacon_coords.0).unique().collect();

    excluded_ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let excluded_deque: VecDeque<RangeInclusive<i32>> = excluded_ranges.into_iter().collect();
    let combined_ranges = combine_ranges(excluded_deque);
    let sensed: usize = combined_ranges.iter().map(|r| (r.end() - r.start() + 1) as usize).sum();
    let covered_beacons = if min_x.is_none() {
        beacons_x.iter().filter(|bx| combined_ranges.iter().any(|r| r.contains(bx))).count()
    } else { 0 };
    (sensed - covered_beacons, combined_ranges, beacons_x)
}

fn combine_ranges<T: PartialOrd + Copy + Debug>(mut in_ranges: VecDeque<RangeInclusive<T>>) -> Vec<RangeInclusive<T>> {
    let mut out_ranges: Vec<RangeInclusive<T>> = Vec::new();

    let mut working_range = in_ranges.pop_front().unwrap();
    while working_range.is_empty() {
        working_range = in_ranges.pop_front().unwrap();
    }
    while !in_ranges.is_empty() {
        let next_range = in_ranges.pop_front().unwrap();
        if next_range.is_empty() { continue; }

        let ws = working_range.start();
        let we = working_range.end();
        let ns = next_range.start();
        let ne = next_range.end();
        if working_range.contains(ns) || next_range.contains(ws) {
            let new_s = if ws < ns { ws } else { ns };
            let new_e = if we > ne { we } else { ne };
            working_range = *new_s..=*new_e;
        } else {
            out_ranges.push(working_range);
            working_range = next_range;
        }
    }
    out_ranges.push(working_range);

    out_ranges
}

fn part_two(sensors: &Vec<Sensor>, max_coord: i32) -> usize {
    let mut exclusions: Vec<RangeInclusive<i32>> = vec![];
    let mut beacons: Vec<i32> = vec![];
    let mut count = 0;
    let row = (0..=max_coord).find(|y| {
        (count, exclusions, beacons) = part_one(sensors, *y, Some(0), Some(max_coord));
        count < max_coord as usize + 1
    }).unwrap();

    let col = (0..=max_coord).find(|x| {
        exclusions.iter().all(|r| !r.contains(x)) && beacons.iter().find(|bx| *bx == x).is_none()
    }).unwrap();
    row as usize + (col as usize * 4000000)
}

fn main() {
    let data = load_data();
    let sensors = parse_data(&data);

    let row = if sensors.len() > 20 { 2000000 } else { 10 };
    let (count, _, _) = part_one(&sensors, row, None, None);
    println!("Part one {}", count);
    
    let (_min_coord, max_coord) = if sensors.len() > 20 { (0, 4000000) } else { (0, 20) };
    println!("Part 2 : {}", part_two(&sensors, max_coord));
}
