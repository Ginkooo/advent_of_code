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
