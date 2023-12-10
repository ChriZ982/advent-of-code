use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-05/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn solve(input: &str) -> u64 {
    let mut seeds = input.lines().nth(0).unwrap().split_ascii_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    input.split("\n\n").skip(1).for_each(|map_desc| {
        let map_rules: Vec<(u64, u64, u64)> = map_desc
            .lines()
            .skip(1)
            .map(|line| line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .map(|v| (v[0], v[1], v[2]))
            .collect();
        seeds.iter_mut().for_each(|seed| {
            map_rules.iter().any(|(dest_start, src_start, len)| {
                if *src_start <= *seed && *seed < *src_start + *len {
                    *seed = *dest_start + (*seed - *src_start);
                    true
                } else {
                    false
                }
            });
        });
    });
    *seeds.iter().min().unwrap()
}

const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

fn main() {
    assert_eq!(solve(TEST_INPUT), 35);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
