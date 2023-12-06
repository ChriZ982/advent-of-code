use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-4/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_nums(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
}

fn solve(input: &str) -> i32 {
    let mut next_multipliers: VecDeque<i32> = VecDeque::new();
    input.lines().fold(0, |acc, line| {
        let parts = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning = parse_nums(parts.0);
        let winning_count = parse_nums(parts.1).iter().filter(|card| winning.contains(card)).count();
        let multiplier = 1 + next_multipliers.pop_front().unwrap_or(0);

        next_multipliers.iter_mut().take(winning_count).for_each(|m| *m += multiplier);
        if winning_count > next_multipliers.len() {
            next_multipliers.append(&mut VecDeque::from(vec![multiplier; winning_count - next_multipliers.len()]));
        }

        acc + multiplier
    })
}

const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn main() {
    assert_eq!(solve(TEST_INPUT), 30);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
