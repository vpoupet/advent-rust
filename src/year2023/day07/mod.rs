use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

use crate::utils;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
}
impl Hand {
    fn get_type(&self) -> i32 {
        let mut counter = HashMap::new();
        for c in self.cards.chars() {
            let count = counter.entry(c).or_insert(0);
            *count += 1;
        }

        let mut counts = counter.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        match counts[0] {
            5 => 7,
            4 => 6,
            3 => {
                if counts[1] == &2 {
                    5
                } else {
                    4
                }
            }
            2 => {
                if counts[1] == &2 {
                    3
                } else {
                    2
                }
            }
            _ => 1,
        }
    }

    fn get_card_values(&self) -> Vec<i32> {
        let labels: Vec<char> = "23456789TJQKA".chars().collect();
        let mut values = HashMap::new();
        for (i, c) in labels.iter().enumerate() {
            values.insert(c, i as i32);
        }
        // associate a value to each card
        self.cards
            .chars()
            .map(|c| *values.get(&c).unwrap())
            .collect()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type != other_type {
            return self_type.partial_cmp(&other_type);
        }

        let self_values = self.get_card_values();
        let other_values = other.get_card_values();
        self_values.partial_cmp(&other_values)
    }
}

// !!!
// Strange function: works with "return expr;" but not with "expr"
// !!!
// fn parse_line(input: &str) -> IResult<&str, (Vec<i32>, i32)> {
//     let labels = "AKQJT98765432";
//     // associate a value to each card
//     let mut values = HashMap::new();
//     for (i, c) in labels.chars().rev().enumerate() {
//         values.insert(c, i);
//     }

//     return separated_pair(
//         map(is_a(labels), |s: &str| {
//             s.chars().map(|c| *values.get(&c).unwrap() as i32).collect()
//         }),
//         tag(" "),
//         utils::parse_int,
//     )(input);
// }

fn parse_line(input: &str) -> IResult<&str, (Hand, i32)> {
    let labels = "AKQJT98765432";
    map(
        separated_pair(is_a(labels), tag(" "), utils::parse_int),
        |(c, bid)| {
            (
                Hand {
                    cards: c.to_string(),
                },
                bid,
            )
        },
    )(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day07/input.txt").unwrap();

    let mut hands = Vec::new();
    for line in input.lines() {
        let (_, (hand, bid)) = parse_line(line).unwrap();
        hands.push((hand, bid));
    }
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut total = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        total += bid * (i + 1) as i32;
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/dayXX/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        // assert_eq!(solution, 0);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
