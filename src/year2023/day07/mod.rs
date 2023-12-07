use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    sequence::separated_pair,
    IResult,
};

use crate::utils;

#[derive(PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_line(input: &str) -> IResult<&str, (&str, i32)> {
    let labels = "23456789TJQKA";
    separated_pair(is_a(labels), tag(" "), utils::parse_int)(input)
}

fn get_hand_type(cards: &str, jokers: bool) -> HandType {
    let mut counter = HashMap::new();
    let mut nb_jokers = 0;
    for c in cards.chars() {
        if jokers && c == 'J' {
            nb_jokers += 1;
        } else {
            let count = counter.entry(c).or_insert(0);
            *count += 1;
        }
    }

    if nb_jokers == 5 {
        return HandType::FiveOfAKind;
    }
    let mut counts = counter.values().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();
    match counts[0] + nb_jokers {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if counts[1] == &2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        },
        2 => {
            if counts[1] == &2 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            }
        },
        _ => HandType::HighCard, // high card
    }
}

fn get_card_values(cards: &str, jokers: bool) -> Vec<i32> {
    let labels: Vec<char> = if jokers {
        "J23456789TQKA".chars().collect()
    } else {
        "23456789TJQKA".chars().collect()
    };
    let mut values = HashMap::new();
    for (i, c) in labels.iter().enumerate() {
        values.insert(c, i as i32);
    }
    // associate a value to each card
    cards.chars().map(|c| *values.get(&c).unwrap()).collect()
}

fn compare_hands(hand1: &str, hand2: &str, jokers: bool) -> Option<std::cmp::Ordering> {
    let type1 = get_hand_type(hand1, jokers);
    let type2 = get_hand_type(hand2, jokers);
    if type1 != type2 {
        return type1.partial_cmp(&type2);
    }

    let values1 = get_card_values(hand1, jokers);
    let values2 = get_card_values(hand2, jokers);
    values1.partial_cmp(&values2)
}

fn get_winnings(jokers: bool) -> i32 {
    let input = utils::read_input("src/year2023/day07/input.txt").unwrap();

    let mut hands = Vec::new();
    for line in input.lines() {
        let (_, (hand, bid)) = parse_line(line).unwrap();
        hands.push((hand, bid));
    }
    hands.sort_by(|a, b| compare_hands(&a.0, &b.0, jokers).unwrap());

    let mut total = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        total += bid * (i + 1) as i32;
    }
    total
}

pub fn solve1() -> i32 {
    get_winnings(false)
}

pub fn solve2() -> i32 {
    get_winnings(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 246163188);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 245794069);
    }
}
