use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-1/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn solve(input: String) -> i32 {
    input
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let first_idx = line.find(char::is_numeric).unwrap();
            let last_idx = line.rfind(char::is_numeric).unwrap();
            let first_char = line.chars().nth(first_idx).unwrap().to_string();
            let last_char = line.chars().nth(last_idx).unwrap().to_string();
            let num_str = first_char + &last_char;
            num_str.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

fn main() {
    assert_eq!(solve(TEST_INPUT.to_string()), 142);

    println!("Solution: {}", solve(load_file()));
}
