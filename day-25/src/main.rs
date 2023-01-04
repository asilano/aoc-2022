use std::fs;

use num_traits::FromPrimitive;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

trait IsSnafu {
    fn parse_snafu(&self) -> u64;
}
impl IsSnafu for String {
    fn parse_snafu(&self) -> u64 {
        let mut accum = 0i64;
        for snigit in self.chars() {
            accum *= 5;
            accum += match snigit {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!()
            };
        }

        accum as u64
    }
}
impl IsSnafu for str {
    fn parse_snafu(&self) -> u64 { self.to_string().parse_snafu() }
}

trait ToSnafu {
    fn to_snafu(self) -> String;
}
impl<T: num_traits::PrimInt + num_traits::ToPrimitive> ToSnafu for T {
    fn to_snafu(self) -> String {
        if self.to_u64().expect("") == 0 { return "0".to_string(); }

        let mut working = self.to_u64().expect("");
        let mut reverse_ans = "".to_string();
        while working > 0 {
            match working % 5 {
                snigit @ (0 | 1 | 2) => {
                    reverse_ans.push(char::from_digit(snigit as u32, 10).unwrap()); 
                },
                3 => {
                    reverse_ans.push('=');
                    working += 2;
                },
                4 => {
                    reverse_ans.push('-');
                    working += 1;
                },
                _ => unreachable!()
            };
            working /= 5;
        }

        reverse_ans.chars().rev().collect()
    }
}

fn part_one(data: &String) -> String {
    data.lines().map(|sn| sn.parse_snafu()).sum::<u64>().to_snafu()
}

fn main() {
    let data = load_data();
    let ans = part_one(&data);
    println!("Part one: {}", ans);

}
