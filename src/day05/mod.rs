use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

use crate::utils;

fn parse_move(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (rem, (x, y, z)) = tuple((
        preceded(tag("move "), digit1),
        preceded(tag(" from "), digit1),
        preceded(tag(" to "), digit1),
    ))(input)?;

    Ok((
        rem,
        (
            x.parse::<usize>().unwrap(),
            y.parse::<usize>().unwrap(),
            z.parse::<usize>().unwrap(),
        ),
    ))
}

fn make_stacks() -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    let input = utils::read_input("src/day05/start.txt").unwrap();
    for line in input.lines() {
        for i in 0..9 {
            let c = line.chars().nth(4*i + 1).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    for i in 0..9 {
        stacks[i].reverse();
    }
    stacks
}

pub fn solve1() -> String {
    let mut stacks = make_stacks();
    
    let input = utils::read_input("src/day05/moves.txt").unwrap();
    for line in input.lines() {
        let (n, start, end) = parse_move(line).unwrap().1;
        for _ in 0..n {
            let c = stacks[start - 1].pop().unwrap();
            stacks[end - 1].push(c);
        }
    }
    
    let mut result = String::new();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
}

pub fn solve2() -> String {
    let mut stacks = make_stacks();
    
    let input = utils::read_input("src/day05/moves.txt").unwrap();
    for line in input.lines() {
        let (n, start, end) = parse_move(line).unwrap().1;
        let l = stacks[start - 1].len();
        let moved = stacks[start - 1].drain(l - n..).collect::<Vec<char>>();
        stacks[end - 1].extend(moved);
    }
    
    let mut result = String::new();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        println!("Part One: {}", solve1());
        assert_eq!(solve1(), "QNHWJVJZW");
    }

    #[test]
    fn test_solve2() {
        println!("Part Two: {}", solve2());
        assert_eq!(solve2(), "BPCZJLFJW");
    }
}
