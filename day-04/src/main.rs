use std::fs;
use std::ops::Range;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn parse_data(data: String) -> Vec<(Range<u32>, Range<u32>)> {
    data.lines().map(|pair| {
        let (left, right) = pair.split_once(',').unwrap();
        let (lsstart, lsend) = left.split_once('-').unwrap();
        let lstart = lsstart.parse::<u32>().unwrap();
        let lend = lsend.parse::<u32>().unwrap();
        let (rsstart, rsend) = right.split_once('-').unwrap();
        let rstart = rsstart.parse::<u32>().unwrap();
        let rend = rsend.parse::<u32>().unwrap();

        (Range { start: lstart, end: lend }, Range { start: rstart, end: rend })
    }).collect::<Vec<(Range<u32>, Range<u32>)>>()
}

fn part_one(ranges: &Vec<(Range<u32>, Range<u32>)>) -> usize {
    ranges.into_iter().filter(|(left, right)| {
       (left.start <= right.start && left.end >= right.end) ||
        (right.start <= left.start && right.end >= left.end) 
    }).count()
}

fn part_two(ranges: &Vec<(Range<u32>, Range<u32>)>) -> usize {
    ranges.into_iter().filter(|(left, right)| {
       (left.start <= right.end) && left.end >= right.start
    }).count()
}

fn main() {
    let data = load_data();
    let ranges = parse_data(data);

    let score = part_one(&ranges);
    println!("Part one score: {}", score);
    let score = part_two(&ranges);
    println!("Part two score: {}", score);
}
