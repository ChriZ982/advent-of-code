use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-11/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn solve(input: &str, expand: usize) -> i64 {
    let star_map = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let empty_rows = star_map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if line.iter().all(|&c| c == '.') { Some(i) } else { None })
        .collect::<Vec<_>>();
    let empty_cols = (0..star_map[0].len()).filter(|&i| star_map.iter().all(|line| line[i] == '.')).collect::<Vec<_>>();

    let galaxies = star_map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().filter_map(move |(x, &c)| if c == '#' { Some((x, y)) } else { None }))
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| galaxies.iter().skip(i + 1).map(move |&p2| (p1, p2)))
        .map(|(p1, p2)| {
            (p1.0 as i64 - p2.0 as i64).abs()
                + (p1.1 as i64 - p2.1 as i64).abs()
                + (p1.0.min(p2.0)..=p1.0.max(p2.0))
                    .filter_map(|x| if empty_cols.contains(&x) { Some(expand as i64) } else { None })
                    .sum::<i64>()
                + (p1.1.min(p2.1)..=p1.1.max(p2.1))
                    .filter_map(|y| if empty_rows.contains(&y) { Some(expand as i64) } else { None })
                    .sum::<i64>()
        })
        .sum::<i64>()
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
    assert_eq!(solve(TEST_INPUT, 1), 374);
    assert_eq!(solve(TEST_INPUT, 9), 1030);
    assert_eq!(solve(TEST_INPUT, 99), 8410);
    println!("Tests passed!");

    println!("Solution: {}", solve(load_file().trim_end(), 999999));
}
