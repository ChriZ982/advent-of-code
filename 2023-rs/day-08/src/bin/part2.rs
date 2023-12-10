use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-08/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn steps_until_end(node_map: &HashMap<&str, (&str, &str)>, instructions: &str, start_node: &str) -> u64 {
    let mut steps = 0;
    let mut current_node = start_node;
    instructions.chars().cycle().any(|instruction| {
        current_node = match instruction {
            'L' => node_map.get(current_node).unwrap().0,
            'R' => node_map.get(current_node).unwrap().1,
            _ => panic!("Invalid instruction: {}", instruction),
        };
        steps += 1;
        current_node.ends_with("Z")
    });
    steps
}

fn solve(input: &str) -> u64 {
    let (instructions, node_map_text) = input.split_once("\n\n").unwrap();
    let node_map: HashMap<&str, (&str, &str)> = node_map_text
        .lines()
        .map(|line| {
            let (node, children) = line.split_once(" = ").unwrap();
            let (left, right) = children.trim_start_matches("(").trim_end_matches(")").split_once(", ").unwrap();
            (node, (left, right))
        })
        .collect();
    let mut current_iterations = node_map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| steps_until_end(&node_map, instructions, node))
        .map(|i| (i, i))
        .collect::<Vec<_>>();
    let mut max = current_iterations.iter().max().unwrap().1;
    while current_iterations.iter().any(|(_, acc_iter)| acc_iter < &max) {
        current_iterations.iter_mut().for_each(|(base_iter, acc_iter)| {
            while *acc_iter < max {
                *acc_iter += *base_iter;
            }
            max = max.max(*acc_iter);
        });
    }
    max
}

const TEST_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn main() {
    assert_eq!(solve(TEST_INPUT), 6);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
