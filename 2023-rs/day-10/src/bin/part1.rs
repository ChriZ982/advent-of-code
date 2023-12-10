use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-10/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            '|' => Pipe::NorthSouth,
            '-' => Pipe::EastWest,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid pipe char: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [(Direction, (i32, i32)); 4] = [
    (Direction::North, (0, -1)),
    (Direction::East, (1, 0)),
    (Direction::South, (0, 1)),
    (Direction::West, (-1, 0)),
];

fn check_match(current: Pipe, next: Pipe, dir: Direction) -> bool {
    match current {
        Pipe::NorthSouth => match dir {
            Direction::North => next == Pipe::NorthSouth || next == Pipe::SouthWest || next == Pipe::SouthEast || next == Pipe::Start,
            Direction::South => next == Pipe::NorthSouth || next == Pipe::NorthWest || next == Pipe::NorthEast || next == Pipe::Start,
            _ => false,
        },
        Pipe::EastWest => match dir {
            Direction::East => next == Pipe::EastWest || next == Pipe::NorthWest || next == Pipe::SouthWest || next == Pipe::Start,
            Direction::West => next == Pipe::EastWest || next == Pipe::NorthEast || next == Pipe::SouthEast || next == Pipe::Start,
            _ => false,
        },
        Pipe::NorthEast => match dir {
            Direction::North => next == Pipe::NorthSouth || next == Pipe::SouthWest || next == Pipe::SouthEast || next == Pipe::Start,
            Direction::East => next == Pipe::EastWest || next == Pipe::NorthWest || next == Pipe::SouthWest || next == Pipe::Start,
            _ => false,
        },
        Pipe::NorthWest => match dir {
            Direction::North => next == Pipe::NorthSouth || next == Pipe::SouthWest || next == Pipe::SouthEast || next == Pipe::Start,
            Direction::West => next == Pipe::EastWest || next == Pipe::NorthEast || next == Pipe::SouthEast || next == Pipe::Start,
            _ => false,
        },
        Pipe::SouthWest => match dir {
            Direction::South => next == Pipe::NorthSouth || next == Pipe::NorthWest || next == Pipe::NorthEast || next == Pipe::Start,
            Direction::West => next == Pipe::EastWest || next == Pipe::NorthEast || next == Pipe::SouthEast || next == Pipe::Start,
            _ => false,
        },
        Pipe::SouthEast => match dir {
            Direction::South => next == Pipe::NorthSouth || next == Pipe::NorthWest || next == Pipe::NorthEast || next == Pipe::Start,
            Direction::East => next == Pipe::EastWest || next == Pipe::NorthWest || next == Pipe::SouthWest || next == Pipe::Start,
            _ => false,
        },
        Pipe::Start => match dir {
            Direction::North => next == Pipe::NorthSouth || next == Pipe::SouthWest || next == Pipe::SouthEast,
            Direction::East => next == Pipe::EastWest || next == Pipe::NorthWest || next == Pipe::SouthWest,
            Direction::South => next == Pipe::NorthSouth || next == Pipe::NorthWest || next == Pipe::NorthEast,
            Direction::West => next == Pipe::EastWest || next == Pipe::NorthEast || next == Pipe::SouthEast,
        },
        Pipe::Ground => panic!("Should not be on ground!"),
    }
}

fn find_next(
    world: &Vec<Vec<Pipe>>,
    last_dir: Option<Direction>,
    current_pos: (usize, usize),
) -> (Option<Direction>, Option<(usize, usize)>) {
    let (x, y) = current_pos;
    for (dir, (dx, dy)) in DIRECTIONS {
        if match last_dir {
            Some(Direction::North) => dir == Direction::South,
            Some(Direction::East) => dir == Direction::West,
            Some(Direction::South) => dir == Direction::North,
            Some(Direction::West) => dir == Direction::East,
            None => false,
        } {
            continue;
        }
        let next_pos = (x as i32 + dx, y as i32 + dy);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= world[0].len() as i32 || next_pos.1 >= world.len() as i32 {
            continue;
        }
        let next = world[next_pos.1 as usize][next_pos.0 as usize];
        if check_match(world[y][x], next, dir) {
            if next == Pipe::Start {
                return (None, None);
            }
            return (Some(dir), Some((next_pos.0 as usize, next_pos.1 as usize)));
        }
    }
    (None, None)
}

fn solve(input: &str) -> i32 {
    let world: Vec<Vec<Pipe>> = input.lines().map(|line| line.chars().map(|c| Pipe::from_char(c)).collect()).collect();
    let mut current_pos = world.iter().enumerate().find_map(|(y, line)| line.iter().position(|c| *c == Pipe::Start).map(|x| (x, y)));
    let mut pipe_len = 0;
    let mut last_dir = None;
    while current_pos.is_some() {
        pipe_len += 1;
        (last_dir, current_pos) = find_next(&world, last_dir, current_pos.unwrap());
    }
    (pipe_len as f32 / 2.0).ceil() as i32
}

const TEST_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

const TEST_INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

fn main() {
    assert_eq!(solve(TEST_INPUT), 4);
    assert_eq!(solve(TEST_INPUT2), 8);
    println!("Tests passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
