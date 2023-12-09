use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-9/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn get_next_val(nums: Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![nums];
    while sequences.last().unwrap().iter().any(|num| *num != 0) {
        sequences.push(sequences.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect());
    }
    sequences.iter().rev().skip(1).fold(0, |acc, win| win.first().unwrap() - acc)
}

fn solve(input: &str) -> i32 {
    input.lines().map(|line| get_next_val(line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect())).sum()
}

const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

fn main() {
    assert_eq!(solve(TEST_INPUT), 2);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
