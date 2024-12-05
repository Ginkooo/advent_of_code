use std::collections::HashSet;

use itertools::Itertools;

use crate::Error;

pub fn day4_part1(input: &str) -> Result<i32, Error> {
    let array = input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars().collect_vec())
        .collect::<Vec<Vec<char>>>();

    let mut seen = HashSet::new();

    let mut all = 0;

    for y in 0..array.len() {
        for x in 0..array[y].len() {
            if array[y][x] == 'X' {
                if seen.contains(&(y, x)) {
                    continue;
                }
                seen.insert((y, x));
                let occurs = get_neighbors(y, x, &array);
                all += occurs;
            }
        }
    }

    Ok(all)
}

fn get_neighbors(y: usize, x: usize, array: &Vec<Vec<char>>) -> i32 {
    let mut neighbours_add = vec![
        vec![(-1, 0), (-2, 0), (-3, 0)],
        vec![(-1, 1), (-2, 2), (-3, 3)],
        vec![(0, 1), (0, 2), (0, 3)],
        vec![(1, 1), (2, 2), (3, 3)],
        vec![(1, 0), (2, 0), (3, 0)],
        vec![(1, -1), (2, -2), (3, -3)],
        vec![(0, -1), (0, -2), (0, -3)],
        vec![(-1, -1), (-2, -2), (-3, -3)],
    ];

    let mut occurences = 0;
    neighbours_add
        .iter_mut()
        .for_each(|vec| vec.insert(0, (0, 0)));
    for next in neighbours_add {
        let dupa = next.iter().filter_map(|(add_y, add_x)| {
            let check_y = y as i32 + add_y;
            if check_y < 0 {
                return None;
            }
            let check_y = check_y as usize;
            array.get(check_y).and_then(|vec| {
                let check_x = x as i32 + add_x;
                if check_x < 0 {
                    return None;
                }
                let check_x = check_x as usize;
                vec.get(check_x)
            })
        });

        let res = String::from_iter(dupa);
        if res == "XMAS" {
            occurences += 1;
        }
    }
    return occurences;
}

fn get_x_mas(y: usize, x: usize, array: &Vec<Vec<char>>) -> i32 {
    let mut possibs = vec![
        vec![(-1, -1), (0, 0), (1, 1)],
        vec![(-1, 1), (0, 0), (1, -1)],
    ];

    let mut occurences = 0;
    for next in &mut possibs {
        let dupa = next.iter().filter_map(|(add_y, add_x)| {
            let check_y = y as i32 + add_y;
            if check_y < 0 {
                return None;
            }
            let check_y = check_y as usize;
            array.get(check_y).and_then(|vec| {
                let check_x = x as i32 + add_x;
                if check_x < 0 {
                    return None;
                }
                let check_x = check_x as usize;
                vec.get(check_x)
            })
        });

        let res = String::from_iter(dupa);
        if res == "MAS" || res == "SAM" {
            occurences += 1;
        }
    }
    if occurences == 2 {
        return 1;
    }
    return 0;
}

pub fn day4_part2(input: &str) -> Result<i32, Error> {
    let array = input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars().collect_vec())
        .collect::<Vec<Vec<char>>>();

    let mut seen = HashSet::new();

    let mut all = 0;

    for y in 0..array.len() {
        for x in 0..array[y].len() {
            if array[y][x] == 'A' {
                if seen.contains(&(y, x)) {
                    continue;
                }
                seen.insert((y, x));
                let occurs = get_x_mas(y, x, &array);
                all += occurs;
            }
        }
    }

    Ok(all)
}
