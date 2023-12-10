use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-06/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_num(input: &str, index: usize) -> u64 {
    input.lines().nth(index).unwrap().replace(" ", "").split(":").nth(1).unwrap().parse::<u64>().unwrap()
}

fn solve(input: &str) -> u64 {
    let max_time = parse_num(input, 0);
    let record = parse_num(input, 1);
    (1..max_time).fold(0, |acc, charge_time| {
        acc + if (max_time - charge_time) * charge_time > record { 1 } else { 0 }
    })
}

const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

fn main() {
    assert_eq!(solve(TEST_INPUT), 71503);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
