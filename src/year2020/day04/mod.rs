use nom::{IResult, sequence::separated_pair, character::complete::{alpha1, one_of}, bytes::complete::tag, multi::separated_list1};

use crate::utils;

fn parse_item(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(":"), alpha1)(input)
}

fn parse_passport(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(one_of(" \n"), parse_item)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(&str, &str)>>> {
    separated_list1(tag("\n\n"), parse_passport)(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/dayXX/input.txt").unwrap();
    let (_, data) = parse_input(&input).unwrap();
    0
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/dayXX/input.txt").unwrap();
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
