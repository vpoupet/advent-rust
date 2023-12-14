use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<usize>)> {
    separated_pair(
        is_a(".#?"),
        tag(" "),
        separated_list1(tag(","), utils::parse_unsigned_int),
    )(input)
}

fn count(
    springs: &[char],
    segments: &[usize],
    memo: &mut HashMap<(Vec<char>, Vec<usize>), i64>,
) -> i64 {
    let key = (springs.to_vec(), segments.to_vec());
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }

    let mut value;
    match springs.get(0) {
        None => {
            if segments.len() == 0 {
                value = 1;
            } else {
                value = 0;
            }
        }
        Some('.') => {
            value = count(&springs[1..], segments, memo);
        }
        Some('#') => {
            if segments.len() == 0 {
                value = 0;
            } else {
                let n = segments[0];
                if springs.len() < n {
                    value = 0;
                } else if springs.len() == n && springs.iter().all(|c| *c != '.') {
                    if segments.len() == 1 {
                        value = 1;
                    } else {
                        value = 0;
                    }
                } else if springs[..n].iter().all(|c| *c != '.') && (springs[n] != '#') {
                    value = count(&springs[n + 1..], &segments[1..], memo);
                } else {
                    value = 0;
                }
            }
        }
        Some('?') => {
            value = count(&springs[1..], segments, memo);
            let mut new_springs = springs.to_vec();
            new_springs[0] = '#';
            value += count(&new_springs, segments, memo);
        }
        _ => panic!("Invalid input"),
    }

    memo.insert(key, value);
    value
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2023/day12/input.txt").unwrap();
    let mut total = 0;
    let mut memo = HashMap::new();
    for line in input.lines() {
        let (_, (springs, segments)) = parse_line(line).unwrap();
        let springs = springs.chars().collect::<Vec<char>>();
        total += count(&springs, &segments, &mut memo);
    }
    total
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2023/day12/input.txt").unwrap();
    let mut total = 0;
    let mut memo = HashMap::new();
    for line in input.lines() {
        let (_, (root_springs, root_segments)) = parse_line(line).unwrap();
        let root_springs = root_springs.chars().collect::<Vec<char>>();

        // copy root springs and segments 5 times
        let mut springs = root_springs.clone();
        for _ in 0..4 {
            springs.push('?');
            springs.append(&mut root_springs.clone());
        }
        let mut segments = root_segments.clone();
        for _ in 0..4 {
            segments.append(&mut root_segments.clone());
        }

        total += count(&springs, &segments, &mut memo);
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
        assert_eq!(solution, 7017);
    }

    #[test]
    #[ignore = "long test (2s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 527570479489);
    }
}
