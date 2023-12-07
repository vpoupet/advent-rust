use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    combinator::map,
    IResult, sequence::separated_pair, multi::many1,
};

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (Vec<i32>, i32)> {
    let labels = "AKQJT98765432";
    // associate a value to each card
    let mut values = HashMap::new();
    for (i, c) in labels.chars().rev().enumerate() {
        values.insert(c, i);
    }

    // let (_, (hand_string, bid)) = separated_pair(
    //     is_a(labels),
    //     tag(" "),
    //     utils::parse_int,
    // )(input).unwrap();

    // let hand = hand_string.chars().map(|c| *values.get(&c).unwrap() as i32).collect();

    let (_, (hand_string, bid)) = separated_pair(
        is_a(labels),
        tag(" "),
        utils::parse_int,
    )(input).unwrap();

    let hand = hand_string.chars().map(|c| *values.get(&c).unwrap() as i32).collect();
    Ok((input, (hand, bid)))
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/dayXX/input.txt").unwrap();
    0
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
