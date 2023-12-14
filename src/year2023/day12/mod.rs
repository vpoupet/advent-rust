use nom::{IResult, sequence::separated_pair, bytes::complete::{is_a, tag}, multi::separated_list1};

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<i32>)> {
    separated_pair(
        is_a(".#?"),
        tag(" "),
        separated_list1(
            tag(","),
            utils::parse_int
        )
    )(input)
}

fn get_lengths(input: &Vec<char>) -> Vec<i32> {
    let mut lengths: Vec<i32> = Vec::new();
    let mut counter = 0;
    for c in input {
        if *c == '#' {
            counter += 1;
        } else {
            if counter > 0 {
                lengths.push(counter);
                counter = 0;
            }
        }
    }
    if counter > 0 {
        lengths.push(counter);
    }
    lengths
}

fn count_solutions(input: &str, lengths: Vec<i32>) -> i32 {
    let mut chars = Vec::new();
    let mut unknown_positions = Vec::new();
    let mut counter = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '?' {
            unknown_positions.push(i);
        }
        chars.push(c);
    }

    for n in 0..1<<unknown_positions.len() {
        for (i, p) in unknown_positions.iter().enumerate() {
            if n & (1 << i) != 0 {
                chars[*p] = '#';
            } else {
                chars[*p] = '.';
            }
        }
        if get_lengths(&chars) == lengths {
            counter += 1;
        }
    }
    counter
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day12/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, (pattern, lengths)) = parse_line(line).unwrap();
        total += count_solutions(pattern, lengths);
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
    fn test() {
        assert_eq!(count_solutions("???.###", vec![1,1,3]), 1);
        assert_eq!(count_solutions(".??..??...?##.", vec![1,1,3]), 4);
    }

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 7017);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
