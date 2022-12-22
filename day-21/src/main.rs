use std::{fs, collections::HashMap, fmt::Display};

use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(Clone)]
enum MonkeyResult {
    Value(i64),
    Equation(String)
}
impl Display for MonkeyResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            MonkeyResult::Value(val) => write!(f, "{}", val),
            MonkeyResult::Equation(str) => f.write_str(&str.clone()),
        }
    }
}

struct Monkey {
    name: String,
    left_opand: Option<String>,
    right_opand: Option<String>,
    oper: Option<char>,
    value: Option<i64>
}
impl Monkey {
    fn calculate(&self, troupe: &HashMap<String, Monkey>) -> i64 {
        if let Some(val) = self.value { return val; }

        let left_val = troupe.get(&self.left_opand.clone().unwrap()).unwrap().calculate(troupe);        
        let right_val = troupe.get(&self.right_opand.clone().unwrap()).unwrap().calculate(troupe);

        match self.oper.unwrap() {
            '+' => left_val + right_val,
            '-' => left_val - right_val,
            '*' => left_val * right_val,
            '/' => left_val / right_val,
            _ => unreachable!()
        }
    }

    fn calculate_two(&self, troupe: &HashMap<String, Monkey>) -> MonkeyResult {
        if self.name == "humn" { return MonkeyResult::Equation("x".to_string()); }
        if let Some(val) = self.value { return MonkeyResult::Value(val); }

        let left_res = troupe.get(&self.left_opand.clone().unwrap()).unwrap().calculate_two(troupe);        
        let right_res = troupe.get(&self.right_opand.clone().unwrap()).unwrap().calculate_two(troupe);

        if let MonkeyResult::Value(left_val) = left_res {
            if let MonkeyResult::Value(right_val) = right_res {
		        return MonkeyResult::Value(match self.oper.unwrap() {
		            '+' => left_val + right_val,
		            '-' => left_val - right_val,
		            '*' => left_val * right_val,
		            '/' => left_val / right_val,
		            _ => unreachable!()
		        });
            }
        }

        if let MonkeyResult::Equation(right_val) = right_res.clone() {
            if let MonkeyResult::Value(left_val) = left_res {
                return MonkeyResult::Equation(format!("{} {} ({})", left_val, self.oper.unwrap(), right_val));
            }
        }

        if let MonkeyResult::Value(right_val) = right_res {
            if let MonkeyResult::Equation(left_val) = left_res {
                return MonkeyResult::Equation(format!("({}) {} {}", left_val, self.oper.unwrap(), right_val));
            }
        }

        unreachable!();
    }
}

fn parse_data(data: &String) -> HashMap<String, Monkey> {
    let mut troupe = HashMap::<String, Monkey>::new();
    for line in data.lines() {
        let (name, remain) = line.split_once(": ").unwrap();

        if remain.contains(['+','-','*','/']) {
            let (left_opand, remain) = remain.split_once(' ').unwrap();
            let (oper, right_opand) = remain.split_once(' ').unwrap();

            troupe.insert(name.to_string(), Monkey {
                name: name.to_string(), 
                left_opand: Some(left_opand.to_string()), 
                right_opand: Some(right_opand.to_string()), 
                oper: Some(oper.chars().nth(0).unwrap()),
                value: None
            });
        } else {
            let value = remain.parse::<i64>().unwrap();

            troupe.insert(name.to_string(), Monkey {
                name: name.to_string(), 
                left_opand: None,
                right_opand: None,
                oper: None,
                value: Some(value)
            });
        }
    }

    troupe
}

fn part_one(troupe: &HashMap<String, Monkey>, interested: String) -> i64 {
    let monkey = troupe.get(&interested).unwrap();
    monkey.calculate(troupe)
}

fn part_two(troupe: &HashMap<String, Monkey>, equality: String) -> i64 {
    let equal_monkey = troupe.get(&equality).unwrap();
    
    let left_name = equal_monkey.left_opand.clone().unwrap();
    let right_name = equal_monkey.right_opand.clone().unwrap();
    let left_monkey = troupe.get(&left_name).unwrap();
    let right_monkey = troupe.get(&right_name).unwrap();

    let left = left_monkey.calculate_two(troupe);
    let right = right_monkey.calculate_two(troupe);
    println!("{} = {}", left, right);
    
    let mut val = match left {
        MonkeyResult::Value(val) => val,
        MonkeyResult::Equation(_) => {
            match right {
                MonkeyResult::Value(val) => val,
                _ => unreachable!()
            }
        }
    };
    let mut eqn = match left {
        MonkeyResult::Equation(eqn) => eqn,
        MonkeyResult::Value(_) => {
            match right {
                MonkeyResult::Equation(eqn) => eqn,
                _ => unreachable!()
            }
        }
    };

    let compound_left = Regex::new(r"^\((.*)\) ([+\-*/]) (-?\d+)$").unwrap();
    let compound_right = Regex::new(r"^(-?\d+) ([+\-*/]) \((.*)\)$").unwrap();
    while eqn != "(x)"  && eqn != "x" {
        // println!("{} = {}", eqn, val); 
        if let Some(caps) = compound_left.captures(&eqn) {
            let number = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            match caps.get(2).unwrap().as_str() {
                "+" => val -= number,
                "-" => val += number,
                "*" => val /= number,
                "/" => val *= number,
                _ => unreachable!()
            };
            eqn = caps.get(1).unwrap().as_str().to_string();
        } else if let Some(caps) = compound_right.captures(&eqn) {
            let number = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            match caps.get(2).unwrap().as_str() {
                "+" => val -= number,
                "-" => val = -(val - number),
                "*" => val /= number,
                "/" => val = number / val,
                _ => unreachable!()
            };
            eqn = caps.get(3).unwrap().as_str().to_string();
        } else { 
            unreachable!() 
        }
    }

    val
}

fn main() {
    let data = load_data();
    let troupe = parse_data(&data);
    
    let root_val = part_one(&troupe, "root".to_string());
    println!("Part one: {}", root_val);

    let my_val = part_two(&troupe, "root".to_string());
    println!("Part two: {}", my_val);
}
