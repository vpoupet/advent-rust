use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (i32, Vec<i32>, Vec<i32>)> {
    tuple((
        delimited(
            pair(tag("Card"), multispace1),
            utils::parse_int,
            pair(tag(":"), multispace1),
        ),
        terminated(
            separated_list1(multispace1, utils::parse_int),
            pair(tag(" |"), multispace1),
        ),
        separated_list1(multispace1, utils::parse_int),
    ))(input)
}

fn make_cards_scores() -> Vec<i32> {
    let input = utils::read_input("src/year2023/day04/input.txt").unwrap();
    let mut scores = Vec::new();
    for line in input.lines() {
        let (_, (_, winning_numbers, card_numbers)) = parse_line(line).unwrap();
        let winning_numbers: HashSet<i32> = winning_numbers.into_iter().collect();
        let card_numbers: HashSet<i32> = card_numbers.into_iter().collect();

        let mut nb_winning_numbers = 0;
        for number in card_numbers {
            if winning_numbers.contains(&number) {
                nb_winning_numbers += 1;
            }
        }
        scores.push(nb_winning_numbers);
    }
    scores
}

pub fn solve1() -> i32 {
    let scores = make_cards_scores();
    let mut total = 0;
    for score in scores {
        if score > 0 {
            total += 1 << (score - 1);
        }
    }
    total
}

pub fn solve2() -> i32 {
    let scores = make_cards_scores();
    let mut t = vec![0; scores.len()];
    for i in (0..scores.len()).rev() {
        let mut n = 1;
        for j in 0..(scores[i] as usize) {
            n += t[i + 1 + j];
        }
        t[i] = n;
    }
    t.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 23847);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 8570000);
    }
}
