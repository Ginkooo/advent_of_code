use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;

fn main() {
    let day: i32 = std::env::args()
        .nth(1)
        .expect("missing 1st argument - day")
        .parse()
        .unwrap();
    let part: i32 = std::env::args()
        .nth(2)
        .expect("missing 2nd argument - part")
        .parse()
        .unwrap();
    let input = std::fs::read_to_string(&String::from_iter([&day.to_string(), ".txt"]))
        .expect(&format!("missing {day} input file"));

    let function = match (day, part) {
        (1, 1) => day1_part1,
        (1, 2) => day1_part2,
        _ => {
            unimplemented!()
        }
    };

    let result = function(&input).unwrap();

    println!("{result}");
}

fn day1_part1(input: &str) -> Result<i32, Error> {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in input.lines() {
        let splitted: Result<Vec<i32>, _> = line.split_whitespace().map(|n| n.parse()).collect();
        let splitted = splitted.expect("malformed list");
        left_list.push(splitted[0]);
        right_list.push(splitted[1]);
    }
    left_list.sort();
    right_list.sort();

    Ok(std::iter::zip(left_list, right_list)
        .map(|(left, right)| (left - right).abs())
        .sum())
}

fn day1_part2(input: &str) -> Result<i32, Error> {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for right_num in input
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .filter_map(|number| number.parse::<i32>().ok())
    {
        *counter.entry(right_num).or_insert(0) += 1;
    }
    Ok(input
        .lines()
        .filter_map(|line| line.split_whitespace().nth(0))
        .filter_map(|number| number.parse::<i32>().ok())
        .map(|num| num * counter.get(&num).unwrap_or(&0))
        .sum())
}
