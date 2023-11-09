use crate::utils;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char},
    sequence::{terminated, tuple},
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, (i32, i32, char, &str)> {
    tuple((
        terminated(utils::parse_int, char('-')),
        terminated(utils::parse_int, char(' ')),
        terminated(anychar, tag(": ")),
        alpha1,
    ))(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/day02/input.txt").unwrap();
    let mut total = 0;
    
    for line in input.lines() {
        let (_, (min, max, letter, password)) = parse_line(line).unwrap();
        let n = password.chars().filter(|c| c == &letter).count();
        if (min as usize..=max as usize).contains(&n) {
            total += 1;
        }
    }
    
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/day02/input.txt").unwrap();
    let mut total = 0;
    
    for line in input.lines() {
        let (_, (min, max, letter, password)) = parse_line(line).unwrap();
        let a = password.chars().nth(min as usize - 1).unwrap();
        let b = password.chars().nth(max as usize - 1).unwrap();
        if (a == letter) ^ (b == letter) {
            total += 1;
        }
    }
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 600);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 245);
    }
}
