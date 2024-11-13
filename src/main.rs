use std::collections::HashMap;

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(&arg).unwrap();
    let function = match &arg[..] {
        "11" => day1_part1,
        "12" => day1_part2,
        "21" => day2_part1,
        "22" => day2_part2,
        "31" => day3_part1,
        "32" => day3_part2,
        _ => unimplemented!()
    };
    let result = function(&data);
    println!("{result}");
}

fn day1_part1(data: &str) -> String {
    let mut floor = 0;
    for ch in data.chars() {
        match ch {
            ')' => {floor -= 1},
            '(' => {floor += 1},
            _ => {},
        }
    }
    floor.to_string()
}

fn day1_part2(data: &str) -> String {
    let mut floor = 0;
    for (index, ch) in data.chars().enumerate() {
        match ch {
            ')' => {floor -= 1},
            '(' => {floor += 1},
            _ => {},
        }
        if floor == -1 {
            return (index + 1).to_string();
        }
    }
    floor.to_string()
}

fn day2_part1(data: &str) -> String {
    let mut total = 0;
    for line in data.lines() {
        let dupa = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (l, w, h) = (dupa[0], dupa[1], dupa[2]);
        let side1 = 2 * l * w;
        let side2 = 2 * w * h;
        let side3 = 2 * h * l;
        total += side1 + side2 + side3 + vec![side1, side2, side3].iter().min().unwrap() / 2;
    }
    return total.to_string()
}

fn day2_part2(data: &str) -> String {
    let mut total = 0;
    for line in data.lines() {
        let mut dupa = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        dupa.sort();
        let (shortest, second_shortest, longest) = (dupa[0], dupa[1], dupa[2]);
        let wrap_len = shortest * 2 + second_shortest * 2;
        let bow_len = shortest * second_shortest * longest;
        total += wrap_len + bow_len;
    }
    return total.to_string()
}

fn day3_part1(data: &str) -> String {
    let mut house_presents: HashMap<(i32, i32), i32> = HashMap::new();
    let mut position = (0, 0);
    house_presents.insert(position, 1);
    for ch in data.chars() {
        match ch {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            '\n' => {},
            _ => {dbg!(ch); panic!("wrong input")},
        }
        *house_presents.entry(position).or_insert(-1) += 1;
    }
    return house_presents.len().to_string();
}

fn day3_part2(data: &str) -> String {
    let mut house_presents: HashMap<(i32, i32), i32> = HashMap::new();
    let mut positions = HashMap::from([("santa", (0, 0)), ("robot", (0, 0))]);
    house_presents.insert((0, 0), 2);
    let mut current_turn = "santa";
    for ch in data.chars() {
        let position = positions.get_mut(current_turn).unwrap();
        match ch {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            '\n' => {},
            _ => {dbg!(ch); panic!("wrong input")},
        }
        *house_presents.entry(*position).or_insert(-1) += 1;
        if current_turn == "santa" {
            current_turn = "robot";
        } else {
            current_turn = "santa";
        }
    }
    return house_presents.len().to_string();
}
