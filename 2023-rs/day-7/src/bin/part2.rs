use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

fn load_file() -> String {
    let mut file = File::open("day-7/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

fn to_card(c: char) -> Card {
    match c {
        'J' => Card::Jack,
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card!"),
    }
}

fn get_joker_hand_type(jokers: i32, counts: &[&i32]) -> HandType {
    match jokers {
        5 | 4 => HandType::FiveOfAKind,
        3 => match counts {
            [1, 1] => HandType::FourOfAKind,
            [2] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand!"),
        },
        2 => match counts {
            [1, 1, 1] => HandType::ThreeOfAKind,
            [2, 1] => HandType::FourOfAKind,
            [3] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand!"),
        },
        1 => match counts {
            [1, 1, 1, 1] => HandType::OnePair,
            [2, 1, 1] => HandType::ThreeOfAKind,
            [2, 2] => HandType::FullHouse,
            [3, 1] => HandType::FourOfAKind,
            [4] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand!"),
        },
        0 => match counts {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] => HandType::OnePair,
            [2, 2, 1] => HandType::TwoPair,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [3, 2] => HandType::FullHouse,
            [4, 1] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand!"),
        },
        _ => panic!("Invalid hand!"),
    }
}

fn get_hand_type(hand: &Vec<Card>) -> HandType {
    let mut card_counts: [i32; 13] = [0; 13];
    hand.iter().for_each(|c| {
        card_counts[*c as usize] += 1;
    });
    let jokers = card_counts[0];
    card_counts[0] = 0;
    card_counts.sort();
    get_joker_hand_type(
        jokers,
        card_counts.iter().rev().filter(|count| **count > 0).collect::<Vec<&i32>>().as_slice(),
    )
}

fn compare_hands(hand1: &Vec<Card>, hand2: &Vec<Card>) -> Ordering {
    if let order @ Ordering::Less | order @ Ordering::Greater = get_hand_type(hand1).cmp(&get_hand_type(hand2)) {
        return order;
    }

    for (c1, c2) in hand1.iter().zip(hand2.iter()) {
        if c1 != c2 {
            return c1.cmp(c2);
        }
    }
    Ordering::Equal
}

fn solve(input: &str) -> i32 {
    let mut hands = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (hand.chars().map(|c| to_card(c)).collect::<Vec<Card>>(), bid.parse::<i32>().unwrap()))
        .collect::<Vec<(Vec<Card>, i32)>>();
    hands.sort_by(|(hand1, _), (hand2, _)| compare_hands(hand1, hand2));
    hands.iter().enumerate().fold(0, |acc, (index, (_, bid))| acc + (index as i32 + 1) * bid)
}

const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

fn main() {
    assert_eq!(solve(TEST_INPUT), 5905);
    println!("Test passed!");

    println!("Solution: {}", solve(load_file().trim_end()));
}
