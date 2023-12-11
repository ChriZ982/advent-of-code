use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-11/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn solve(input: &str) -> i32 {
    let mut star_map = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let empty_rows = star_map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if line.iter().all(|&c| c == '.') { Some(i) } else { None })
        .collect::<Vec<_>>();
    for i in empty_rows.iter().rev() {
        star_map.insert(*i, ".".repeat(star_map[0].len()).chars().collect());
    }
    let empty_cols = (0..star_map[0].len()).filter(|&i| star_map.iter().all(|line| line[i] == '.')).collect::<Vec<_>>();
    star_map.iter_mut().for_each(|line| {
        for i in empty_cols.iter().rev() {
            line.insert(*i, '.');
        }
    });

    let galaxies = star_map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().filter_map(move |(x, &c)| if c == '#' { Some((x, y)) } else { None }))
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| galaxies.iter().skip(i + 1).map(move |&p2| (p1, p2)))
        .map(|(p1, p2)| (p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs())
        .sum::<i32>()
}

const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

fn main() {
    assert_eq!(solve(TEST_INPUT), 374);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
