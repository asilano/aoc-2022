use std::fs;
use json::{self, JsonValue};

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

#[derive(Clone, PartialEq, Eq)]
struct WrappedJson(JsonValue);

fn parse_data(data: &String) -> Vec<(WrappedJson, WrappedJson)> {
    data.split("\n\n").map(|pair| {
        let (lstr, rstr) = pair.split_once("\n").unwrap();
        let left = json::parse(lstr).unwrap();
        let right = json::parse(rstr).unwrap();
        (WrappedJson(left), WrappedJson(right))
    }).collect()
}

fn parse_data_two(data: &String) -> Vec<WrappedJson> {
    data.lines().filter_map(|line| {
        if line.is_empty() { None } else {
            Some(WrappedJson(json::parse(line).unwrap()))
        }
    }).collect()
}

//impl PartialEq for WrappedJson {
//    fn eq(&self, other: &Self) -> bool {
//        self.0.eq(&other.0)
//    }
//}
impl Ord for WrappedJson {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.0.clone(), other.0.clone()) {
            (JsonValue::Number(_), JsonValue::Number(_)) => (self.0.as_u32().unwrap()).cmp(&(other.0.as_u32().unwrap())),
            (JsonValue::Number(left), right) if right.is_array() => WrappedJson(json::array![left]).cmp(&other),
            (left, JsonValue::Number(right)) if left.is_array() => self.cmp(&WrappedJson(json::array![right])),
            (JsonValue::Array(left), JsonValue::Array(right)) => {
                let diff_pair_opt = left.iter().zip(right.iter()).find(|(l, r)| {
                   WrappedJson((*l).clone()) < WrappedJson((*r).clone()) ||
                   WrappedJson((*l).clone()) > WrappedJson((*r).clone())
                });

                if let Some(diff_pair) = diff_pair_opt {
                    WrappedJson((*diff_pair.0).clone()).cmp(&WrappedJson((*diff_pair.1).clone()))
                } else {
                    left.len().cmp(&right.len())
                }
            },
            _ => unreachable!()
        }

    }
}
impl PartialOrd for WrappedJson {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = load_data();
    let packet_pairs = parse_data(&data);
    let sum: usize = packet_pairs.iter().enumerate().filter_map(|(ix, pair)| {
        if pair.0 < pair.1 { Some(ix + 1) } else { None }
    }).sum();

    println!("Part one: {}", sum);

    let mut packets = parse_data_two(&data);
    let two = WrappedJson(json::array![json::array![2]]);
    let six = WrappedJson(json::array![json::array![6]]);
    packets.push(two.clone());
    packets.push(six.clone());
    packets.sort();
    println!("Part two: {}", (packets.iter().position(|item| item == &two).unwrap() + 1) * (packets.iter().position(|item| item == &six).unwrap() + 1));
}
