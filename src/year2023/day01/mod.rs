use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::utils;

fn parse_digit(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("one"), |_| 1),
        map(tag("two"), |_| 2),
        map(tag("three"), |_| 3),
        map(tag("four"), |_| 4),
        map(tag("five"), |_| 5),
        map(tag("six"), |_| 6),
        map(tag("seven"), |_| 7),
        map(tag("eight"), |_| 8),
        map(tag("nine"), |_| 9),
        map(tag("0"), |_| 0),
        map(tag("1"), |_| 1),
        map(tag("2"), |_| 2),
        map(tag("3"), |_| 3),
        map(tag("4"), |_| 4),
        map(tag("5"), |_| 5),
        map(tag("6"), |_| 6),
        map(tag("7"), |_| 7),
        map(tag("8"), |_| 8),
        map(tag("9"), |_| 9),
    ))(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day01/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut c1: Option<char> = None;
        let mut c2: Option<char> = None;
        for c in line.chars() {
            if c.is_numeric() {
                if c1.is_none() {
                    c1 = Some(c);
                }
                c2 = Some(c);
            }
        }
        if let (Some(c1), Some(c2)) = (c1, c2) {
            let s = c1.to_string() + &c2.to_string();
            total += s.parse::<i32>().unwrap();
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/day01/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut c1: Option<i32> = None;
        let mut c2: Option<i32> = None;
        for i in 0..line.len() {
            let p = parse_digit(&line[i..]);
            if let Ok((_, digit)) = p {
                if c1.is_none() {
                    c1 = Some(digit);
                }
                c2 = Some(digit);
            }
        }
        if let (Some(c1), Some(c2)) = (c1, c2) {
            total += 10 * c1 + c2;
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
        assert_eq!(solution, 55621);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 53592);
    }
}
