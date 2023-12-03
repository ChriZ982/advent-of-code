use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-3/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn sourround_with_dots(input: &str) -> Vec<String> {
    let mut input_lines: Vec<String> = input.lines().map(|l| String::from(".") + l + ".").collect();
    let blank_line = ".".repeat(input_lines[0].len());
    input_lines.insert(0, blank_line.clone());
    input_lines.push(blank_line);
    input_lines
}

fn get_nums(line: &String) -> Vec<((usize, usize), i32)> {
    let mut nums = Vec::new();
    let mut num_start = 0;
    while let Some(advance) = line.chars().skip(num_start).position(|c| c.is_digit(10)) {
        num_start = num_start + advance;
        let num_end = num_start + line.chars().skip(num_start).position(|c| !c.is_digit(10)).unwrap();

        nums.push(((num_start, num_end), line[num_start..num_end].parse::<i32>().unwrap()));

        num_start = num_end;
    }
    nums
}

fn get_two_adjacent_nums(index: usize, nums: Vec<((usize, usize), i32)>) -> Option<(i32, i32)> {
    let mut first_num = None;
    let mut second_num = None;
    for ((start, end), num) in nums {
        if start - 1 <= index && index <= end {
            if first_num.is_none() {
                first_num = Some(num);
            } else if second_num.is_none() {
                second_num = Some(num);
            } else {
                return None;
            }
        }
    }

    if let (Some(a), Some(b)) = (first_num, second_num) {
        Some((a, b))
    } else {
        None
    }
}

fn solve(input: &str) -> i32 {
    sourround_with_dots(input).windows(3).fold(0, |mut acc, window| {
        let mut gear_pos = 0;
        while let Some(advance) = window[1].chars().skip(gear_pos).position(|c| c == '*') {
            gear_pos = gear_pos + advance;
            acc += get_two_adjacent_nums(gear_pos, window.iter().flat_map(get_nums).collect::<Vec<_>>()).map(|(a, b)| a * b).unwrap_or(0);
            gear_pos += 1;
        }
        acc
    })
}

const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn main() {
    assert_eq!(solve(TEST_INPUT), 467835);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
