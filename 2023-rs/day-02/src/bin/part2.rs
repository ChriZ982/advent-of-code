use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-02/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn solve(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        acc + line
            .split_once(":")
            .unwrap()
            .1
            .split(";")
            .map(|s| {
                s.split(",")
                    .map(|s| s.trim().split_once(" ").map(|(count, color)| (color, count.parse::<i32>().unwrap())).unwrap())
                    .collect()
            })
            .fold([0, 0, 0], |[mr, mg, mb], subset: Vec<(&str, i32)>| {
                subset.iter().fold([mr, mg, mb], |[r, g, b], (color, count)| match color {
                    &"red" => [r.max(*count), g, b],
                    &"green" => [r, g.max(*count), b],
                    &"blue" => [r, g, b.max(*count)],
                    _ => panic!("Unknown color: {}", color),
                })
            })
            .iter()
            .product::<i32>()
    })
}

const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    assert_eq!(solve(TEST_INPUT), 2286);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
