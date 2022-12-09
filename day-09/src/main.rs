use std::fs;

use itertools::Itertools;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

struct Rope {
    head: (i32, i32),
    knots: Vec<(i32, i32)>,
    tail_visited: Vec<(i32, i32)>
}
impl Rope {
    fn new(knot_count: usize) -> Self {
        Self {
            head: (0, 0),
            knots: vec![(0,0); knot_count],
            tail_visited: vec![(0,0)]
        }
    }

    fn apply_move(&mut self, motion: &str) {
        let (dir, count) = motion.split_once(" ").unwrap();
        for _ in 0..count.parse::<usize>().unwrap() {
            match dir {
                "L" => self.head.0 -= 1,
                "R" => self.head.0 += 1,
                "U" => self.head.1 -= 1,
                "D" => self.head.1 += 1,
                _ => unreachable!()
            }
            self.pull_tail();
        }
    }

    fn pull_tail(&mut self) {
        let mut pulling = self.head.clone();
        let knot_count = self.knots.len();
        for pulled_ix in 0..knot_count {
            {
		        let pulled = &mut self.knots[pulled_ix];
	
	            // Pulled _was_ touching Pulling, so there is at most one empty space now
		        let dx = pulling.0 - pulled.0;
		        let dy = pulling.1 - pulled.1;
		        if dx.abs() < 2 && dy.abs() < 2 { return; }
		
		        pulled.0 += dx.signum();
		        pulled.1 += dy.signum();
            }
            pulling = self.knots[pulled_ix].clone();
        }
        self.tail_visited.push(self.knots.last().unwrap().clone());
    }
}

fn part_one(data: &String) -> usize {
    let mut rope = Rope::new(1);
    for motion in data.lines() {
        rope.apply_move(motion);
    }

    rope.tail_visited.into_iter().unique().count()
}

fn part_two(data: &String) -> usize {
    let mut rope = Rope::new(9);
    for motion in data.lines() {
        rope.apply_move(motion);
    }

    rope.tail_visited.into_iter().unique().count()
}

fn main() {
    let data = load_data();
    let count = part_one(&data);
    println!("Part one: {}", count);
    let count = part_two(&data);
    println!("Part two: {}", count);
}
