use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::utils;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    divisibility: i32,
    on_true: usize,
    on_false: usize,
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(i32),
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Operation {
    x: Operand,
    op: Operator,
    y: Operand,
}

impl Operation {
    fn exec(&self, input: i32) -> i32 {
        let x = match self.x {
            Operand::Old => input,
            Operand::Value(x) => x,
        };
        let y = match self.y {
            Operand::Old => input,
            Operand::Value(y) => y,
        };
        match self.op {
            Operator::Add => x + y,
            Operator::Multiply => x * y,
        }
    }
}

fn parse_header_line(input: &str) -> IResult<&str, usize> {
    map(delimited(tag("Monkey "), digit1, tag(":\n")), |s: &str| {
        s.parse().unwrap()
    })(input)
}

fn parse_items_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (remaining, items) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), digit1),
        tag("\n"),
    )(input)?;
    Ok((
        remaining,
        items.iter().map(|x| x.parse().unwrap()).collect(),
    ))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(digit1, |s: &str| Operand::Value(s.parse().unwrap())),
    ))(input)
}

fn parse_operation_line(input: &str) -> IResult<&str, Operation> {
    let (remaining, (x, op, y)) = delimited(
        tag("  Operation: new = "),
        tuple((
            terminated(parse_operand, char(' ')),
            one_of("+*"),
            preceded(char(' '), parse_operand),
        )),
        tag("\n"),
    )(input)?;

    let op = match op {
        '+' => Operator::Add,
        '*' => Operator::Multiply,
        _ => panic!("Invalid operator"),
    };

    Ok((remaining, Operation { x, op, y }))
}

fn parse_divisibility(input: &str) -> IResult<&str, (i32, usize, usize)> {
    tuple((
        map(
            delimited(tag("  Test: divisible by "), digit1, tag("\n")),
            |s: &str| s.parse::<i32>().unwrap(),
        ),
        map(
            delimited(tag("    If true: throw to monkey "), digit1, tag("\n")),
            |s: &str| s.parse::<usize>().unwrap(),
        ),
        map(
            delimited(tag("    If false: throw to monkey "), digit1, tag("\n")),
            |s: &str| s.parse::<usize>().unwrap(),
        ),
    ))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (remaining, index) = parse_header_line(input)?;
    let (remaining, items) = parse_items_line(remaining)?;
    let (remaining, operation) = parse_operation_line(remaining)?;
    let (remaining, (divisibility, on_true, on_false)) = parse_divisibility(remaining)?;

    Ok((
        remaining,
        Monkey {
            items,
            operation,
            divisibility,
            on_true,
            on_false,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(char('\n'), parse_monkey)(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/day11/input.txt").unwrap();
    let (_, monkeys) = parse_monkeys(&input).unwrap();
    println!("{:?}", monkeys);
    println!("{}", monkeys.len());
    0
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/dayXX/input.txt").unwrap();
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
