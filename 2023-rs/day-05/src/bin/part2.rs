use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-05/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn transform_matching_return_unmatched(
    (range_start, range_len): (u32, u32),
    (dest_start, src_start, len): (u32, u32, u32),
) -> (Option<(u32, u32)>, Vec<(u32, u32)>) {
    let range_end = range_start + range_len;
    let src_end = src_start + len;
    if range_end <= src_start || src_end <= range_start {
        (None, vec![(range_start, range_len)])
    } else {
        let mut unmatched_ranges = Vec::new();
        if range_start < src_start {
            unmatched_ranges.push((range_start, src_start - range_start));
        }
        if range_end > src_end {
            unmatched_ranges.push((src_end, range_end - src_end));
        }

        let new_range_start = range_start.max(src_start);
        (
            Some((dest_start + (new_range_start - src_start), range_end.min(src_end) - new_range_start)),
            unmatched_ranges,
        )
    }
}

fn solve(input: &str) -> u32 {
    let mut seeds = input
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|seed_desc| (seed_desc[0].parse::<u32>().unwrap(), seed_desc[1].parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>();
    input.split("\n\n").skip(1).for_each(|map_desc| {
        let map_rules: Vec<(u32, u32, u32)> = map_desc
            .lines()
            .skip(1)
            .map(|line| line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
            .map(|v| (v[0], v[1], v[2]))
            .collect();
        seeds = seeds.iter().fold(Vec::new(), |mut new_seeds, seed_range| {
            let mut remaining_ranges = VecDeque::from(vec![seed_range.clone()]);
            while let Some(range) = remaining_ranges.pop_front() {
                if map_rules.iter().all(|map_rule| {
                    let (transformed_range, unmatched_ranges) = transform_matching_return_unmatched(range, *map_rule);
                    if let Some(transformed_range) = transformed_range {
                        new_seeds.push(transformed_range);
                        remaining_ranges.extend(unmatched_ranges);
                        false
                    } else {
                        true
                    }
                }) {
                    new_seeds.push(range);
                }
            }
            new_seeds
        })
    });
    seeds.iter().min_by_key(|(num, _)| *num).unwrap().0
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
    assert_eq!(solve(TEST_INPUT), 46);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
