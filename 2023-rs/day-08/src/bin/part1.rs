use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-08/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn solve(input: &str) -> i32 {
    let (instructions, node_map_text) = input.split_once("\n\n").unwrap();
    let node_map: HashMap<&str, (&str, &str)> = node_map_text
        .lines()
        .map(|line| {
            let (node, children) = line.split_once(" = ").unwrap();
            let (left, right) = children.trim_start_matches("(").trim_end_matches(")").split_once(", ").unwrap();
            (node, (left, right))
        })
        .collect();

    let mut steps = 0;
    let mut current_node = "AAA";
    instructions.chars().cycle().any(|instruction| {
        current_node = match instruction {
            'L' => node_map.get(current_node).unwrap().0,
            'R' => node_map.get(current_node).unwrap().1,
            _ => panic!("Invalid instruction: {}", instruction),
        };
        steps += 1;
        current_node == "ZZZ"
    });
    steps
}

const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const TEST_INPUT_SECOND: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

fn main() {
    assert_eq!(solve(TEST_INPUT), 2);
    assert_eq!(solve(TEST_INPUT_SECOND), 6);
    println!("Tests passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
