use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::sequence::{pair, preceded};
use nom::IResult;

use crate::utils;

enum Line {
    Noop,
    Add(i32),
}

fn parse_signed(input: &str) -> IResult<&str, i32> {
    map(
        pair(opt(char('-')), digit1),
        |(sign, number): (Option<char>, &str)| {
            let mut value = number.parse::<i32>().unwrap();
            if sign == Some('-') {
                value *= -1;
            }
            value
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(tag("noop"), |_| Line::Noop),
        map(preceded(tag("addx "), parse_signed), |value| {
            Line::Add(value)
        }),
    ))(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/day10/input.txt").unwrap();
    let mut state = Vec::new();
    let mut x = 1;
    state.push(x);
    for line in input.lines() {
        match parse_line(line).unwrap() {
            (_, Line::Noop) => {
                state.push(x);
            }
            (_, Line::Add(value)) => {
                state.push(x);
                x += value;
                state.push(x);
            }
        }
    }
    let mut total = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        total += i as i32 * state[i-1];
    }
    total
}

pub fn solve2() -> Vec<String>{
    let input = utils::read_input("src/day10/input.txt").unwrap();
    let mut crt = Vec::new();
    let mut state = Vec::new();
    let mut x = 1;
    state.push(x);
    for line in input.lines() {
        match parse_line(line).unwrap() {
            (_, Line::Noop) => {
                state.push(x);
            }
            (_, Line::Add(value)) => {
                state.push(x);
                x += value;
                state.push(x);
            }
        }
    }
    for j in 0..6 {
        let mut crt_line = String::new();
        for i in 0..40 {
            if state[40*j + i] - 1 <= i as i32 && i as i32 <= state[40*j + i] + 1 {
                crt_line.push_str("#");
            } else {
                crt_line.push_str(" ");
            }
        }
        crt.push(crt_line);
    }
    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 14860);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two:");
        for line in solution {
            println!("{}", line);
        }
        // Solution: "RGZEHURK"
    }
}
