use crate::Error;
use regex::Regex;

pub fn day3_part1(input: &str) -> Result<i32, Error> {
    let pattern = Regex::new(r"mul\((\d{1, 3}),(\d{1, 3})\)").unwrap();

    Ok(pattern
        .captures_iter(input)
        .filter_map(|matches| {
            [matches[1].parse::<i32>(), matches[2].parse::<i32>()]
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .map(|nums| nums[0] * nums[1])
        .sum())
}

fn get_mul_if_enabled(enable: &mut bool, matches: regex::Captures) -> Option<i32> {
    if matches.name("do").is_some() {
        *enable = true;
    }
    if matches.name("dont").is_some() {
        *enable = false;
    }
    if !*enable {
        return None;
    }
    match matches.name("mul") {
        None => None,
        Some(_) => {
            let first_num = matches
                .name("first_num")
                .expect("regex should work")
                .as_str()
                .parse::<i32>();
            let second_num = matches
                .name("second_num")
                .expect("regex should work")
                .as_str()
                .parse::<i32>();

            match (first_num, second_num) {
                (Ok(first), Ok(second)) => Some(first * second),
                _ => None,
            }
        }
    }
}

pub fn day3_part2(input: &str) -> Result<i32, Error> {
    let pattern = Regex::new(
        r"(?<mul>mul\((?<first_num>\d{1, 3}),(?<second_num>\d{1, 3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();
    let mut muls_enabled = true;
    Ok(pattern
        .captures_iter(input)
        .filter_map(|matches| get_mul_if_enabled(&mut muls_enabled, matches))
        .sum())
}
