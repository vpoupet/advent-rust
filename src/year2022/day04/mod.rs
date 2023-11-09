use nom::character::complete::{char, digit1};
use nom::sequence::separated_pair;
use nom::IResult;

use crate::utils;

struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

fn parse_interval(input: &str) -> IResult<&str, Interval> {
    let (remaining, (start, end)) = separated_pair(digit1, char('-'), digit1)(input)?;
    Ok((
        remaining,
        Interval {
            start: start.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
        },
    ))
}

fn parse_line(input: &str) -> (Interval, Interval) {
    let (_, (i1, i2)) = separated_pair(parse_interval, char(','), parse_interval)(input).unwrap();
    (i1, i2)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2022/day04/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (i1, i2) = parse_line(line);
        if i1.contains(&i2) || i2.contains(&i1) {
            total += 1;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2022/day04/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (i1, i2) = parse_line(line);
        if i1.overlaps(&i2) {
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
        assert_eq!(solution, 509);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 870);
    }
}
