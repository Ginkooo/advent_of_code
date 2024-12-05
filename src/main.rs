mod day1;
mod day2;
mod day3;
mod day4;

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
    let input = std::fs::read_to_string(String::from_iter([&day.to_string(), ".txt"]))
        .unwrap_or_else(|_| panic!("missing {day} input file"));

    let function = match (day, part) {
        (1, 1) => day1::day1_part1,
        (1, 2) => day1::day1_part2,
        (2, 1) => day2::day2_part1,
        (2, 2) => day2::day2_part2,
        (3, 1) => day3::day3_part1,
        (3, 2) => day3::day3_part2,
        (4, 1) => day4::day4_part1,
        (4, 2) => day4::day4_part2,
        _ => {
            unimplemented!()
        }
    };

    let result = function(&input).unwrap();

    println!("{result}");
}
