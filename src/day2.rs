use crate::Error;

#[derive(PartialEq)]
enum Ordering {
    Ascending,
    Descending,
    Unknown,
}
fn are_numbers_safe(acc: &mut Ordering, pair: (i32, i32)) -> bool {
    let diff = (pair.0 - pair.1).abs();
    if diff > 3 || diff < 1 {
        return false;
    }

    if pair.0 < pair.1 {
        if *acc == Ordering::Descending {
            return false;
        }
        *acc = Ordering::Ascending;
    } else if pair.0 > pair.1 {
        if *acc == Ordering::Ascending {
            return false;
        }
        *acc = Ordering::Descending;
    } else {
        return false;
    }
    return true;
}

fn is_line_safe(line: &&str) -> bool {
    let mut prev_order = Ordering::Unknown;
    line.split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|window| (window[0], window[1]))
        .all(|pair| are_numbers_safe(&mut prev_order, pair))
}

pub fn day2_part1(input: &str) -> Result<i32, Error> {
    match i32::try_from(
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(is_line_safe)
            .count(),
    ) {
        Ok(num) => Ok(num),
        Err(_) => Err(Error::from("no no")),
    }
}

fn is_any_subcombination_save(line: &&str) -> bool {
    let numbers = line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for i in 0..numbers.len() {
        let mut clone = numbers.clone();
        let mut prev_order = Ordering::Unknown;
        clone.remove(i);
        let is_safe = clone
            .windows(2)
            .map(|window| (window[0], window[1]))
            .all(|pair| are_numbers_safe(&mut prev_order, pair));
        if is_safe {
            return true;
        }
    }
    return false;
}

pub fn day2_part2(input: &str) -> Result<i32, Error> {
    match i32::try_from(
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| is_line_safe(line) || is_any_subcombination_save(line))
            .count(),
    ) {
        Ok(num) => Ok(num),
        Err(_) => Err(Error::from("no no")),
    }
}
