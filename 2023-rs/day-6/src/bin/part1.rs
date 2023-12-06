use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

fn load_file() -> String {
    let mut file = File::open("day-6/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn simulate_race(max_time: i32, record: i32) -> i32 {
    (1..max_time).fold(0, |acc, charge_time| {
        acc + if (max_time - charge_time) * charge_time > record { 1 } else { 0 }
    })
}

fn parse_nums(input: &str, index: usize) -> Vec<i32> {
    input.lines().nth(index).unwrap().split_ascii_whitespace().skip(1).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn solve(input: &str) -> i32 {
    let times = parse_nums(input, 0);
    let records = parse_nums(input, 1);
    zip(times, records).map(|(time, record)| simulate_race(time, record)).product()
}

const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

fn main() {
    assert_eq!(solve(TEST_INPUT), 288);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
