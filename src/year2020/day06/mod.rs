use std::collections::HashSet;

use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1, IResult,
};

use crate::utils;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(tag("\n\n"), separated_list1(tag("\n"), alpha1))(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/day06/input.txt").unwrap();
    let (_, input_data) = parse_input(&input).unwrap();

    let mut total = 0;
    for group in input_data {
        let mut letters = HashSet::new();
        for word in group {
            letters.extend(word.chars());
        }
        total += letters.len();
    }

    total as i32
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/day06/input.txt").unwrap();
    let (_, input_data) = parse_input(&input).unwrap();

    let mut total = 0;
    for group in input_data {
        let mut letters: HashSet<_> = group[0].chars().collect();
        for word in group {
            let new_letters: HashSet<_> = word.chars().collect();
            letters.retain(|c| new_letters.contains(c));
        }
        total += letters.len();
    }
    total as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 6596);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 3219);
    }
}
