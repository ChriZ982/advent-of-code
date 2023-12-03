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

fn is_near_symbol(lines: &[String], start: usize, len: usize) -> bool {
    lines[1].chars().skip(start - 1).take(1).any(|c| c != '.')
        || lines[1].chars().skip(start + len).take(1).any(|c| c != '.')
        || lines[0].chars().skip(start - 1).take(len + 2).any(|c| c != '.')
        || lines[2].chars().skip(start - 1).take(len + 2).any(|c| c != '.')
}

fn solve(input: &str) -> i32 {
    sourround_with_dots(input).windows(3).fold(0, |mut acc, window| {
        let mut num_start = 0;
        while let Some(advance) = window[1].chars().skip(num_start).position(|c| c.is_digit(10)) {
            num_start = num_start + advance;
            let num_len = window[1].chars().skip(num_start).position(|c| !c.is_digit(10)).unwrap();

            if is_near_symbol(window, num_start, num_len) {
                acc += window[1][num_start..num_start + num_len].parse::<i32>().unwrap();
            }

            num_start += num_len;
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
    assert_eq!(solve(TEST_INPUT), 4361);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
