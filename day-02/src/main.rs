use std::fs;
use phf::phf_map;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

static OPP_PLAYS: phf::Map<char, &'static str> = phf_map! { 
    'A' => "Rock",
    'B' => "Paper",
    'C' => "Scissors"
};

static MY_PLAYS: phf::Map<char, &'static str> = phf_map! { 
    'X' => "Rock",
    'Y' => "Paper",
    'Z' => "Scissors"
};

fn part_one(data: &String) -> i64 {
    let mut score: i64 = 0;
    for round in data.lines() {
        let opp = OPP_PLAYS.get(&round.chars().nth(0).unwrap()).cloned();
        let mine = MY_PLAYS.get(&round.chars().nth(2).unwrap()).cloned();
        match mine {
            Some("Rock") => { score += 1; }
            Some("Paper") => { score += 2; }
            Some("Scissors") => { score += 3; }
            _ => { panic!("My play not correct!"); }
        }

        match (opp, mine) {
           (Some(a), Some(b)) if a==b => { score += 3; }
           (Some("Rock"), Some("Paper")) |
               (Some("Paper"), Some("Scissors")) |
               (Some("Scissors"), Some("Rock")) => { score += 6; }
           (Some(_), Some(_)) => { score += 0; }
           (_, _) => { panic!("Can't run match"); }
        }
    }
    score
}

enum RESULT {
    WIN,
    DRAW,
    LOSE
}

static ROUND_RESULT: phf::Map<char, RESULT> = phf_map! { 
    'X' => RESULT::LOSE,
    'Y' => RESULT::DRAW,
    'Z' => RESULT::WIN
};

fn part_two(data: &String) -> i64 {
    let mut score: i64 = 0;
    for round in data.lines() {
        let opp = OPP_PLAYS.get(&round.chars().nth(0).unwrap()).cloned();
        let result = ROUND_RESULT.get(&round.chars().nth(2).unwrap());
        
        let mine = match result {
           Some(RESULT::DRAW) => { 
               score += 3; 
                opp.unwrap()
           }
           Some(RESULT::WIN) => {
               score += 6;
               match opp {
                   Some("Rock") => { "Paper" }
                   Some("Paper") => { "Scissors" }
                   Some("Scissors") => { "Rock" }
                   _ => { panic!("Opponent play not correct!") }
               }
           }
           Some(RESULT::LOSE) => {
               score += 0;
               match opp {
                   Some("Rock") => { "Scissors" }
                   Some("Paper") => { "Rock" }
                   Some("Scissors") => { "Paper" }
                   _ => { panic!("Opponent play not correct!") }
               }
           }
           _ => { panic!("Can't run match"); }
        };

        match mine {
            "Rock" => { score += 1; }
            "Paper" => { score += 2; }
            "Scissors" => { score += 3; }
            _ => { panic!("My play not correct!"); }
        }

    }
    score
}

fn main() {
    let data = load_data();
    let score = part_one(&data);
    println!("Part one score: {}", score);
    let score = part_two(&data);
    println!("Part two score: {}", score);
}
