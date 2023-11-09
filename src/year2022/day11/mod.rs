use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use num::integer;

use crate::utils;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisibility: i64,
    on_true: usize,
    on_false: usize,
    nb_inspected: i64,
}

impl Monkey {
    fn new(
        items: Vec<i64>,
        operation: Operation,
        divisibility: i64,
        on_true: usize,
        on_false: usize,
    ) -> Self {
        Monkey {
            items,
            operation,
            divisibility,
            on_true,
            on_false,
            nb_inspected: 0,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(i64),
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
    fn exec(&self, input: i64) -> i64 {
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

fn parse_items_line(input: &str) -> IResult<&str, Vec<i64>> {
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

fn parse_divisibility(input: &str) -> IResult<&str, (i64, usize, usize)> {
    tuple((
        map(
            delimited(tag("  Test: divisible by "), digit1, tag("\n")),
            |s: &str| s.parse().unwrap(),
        ),
        map(
            delimited(tag("    If true: throw to monkey "), digit1, tag("\n")),
            |s: &str| s.parse().unwrap(),
        ),
        map(
            delimited(tag("    If false: throw to monkey "), digit1, tag("\n")),
            |s: &str| s.parse().unwrap(),
        ),
    ))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (remaining, _) = parse_header_line(input)?;
    let (remaining, items) = parse_items_line(remaining)?;
    let (remaining, operation) = parse_operation_line(remaining)?;
    let (remaining, (divisibility, on_true, on_false)) = parse_divisibility(remaining)?;

    Ok((
        remaining,
        Monkey::new(items, operation, divisibility, on_true, on_false),
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(char('\n'), parse_monkey)(input)
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2022/day11/input.txt").unwrap();
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();
    let nb_monkeys = monkeys.len();
    // play 20 rounds
    for _ in 0..20 {
        // play each monkey's turn
        for i in 0..nb_monkeys {
            while !monkeys[i].items.is_empty() {
                let m = &mut monkeys[i];
                let mut item = m.items.pop().unwrap();
                m.nb_inspected += 1;
                item = m.operation.exec(item);
                item /= 3;
                let target_monkey = if (item % m.divisibility) == 0 {
                    m.on_true
                } else {
                    m.on_false
                };
                monkeys[target_monkey].items.push(item);
            }
        }
    }

    let mut nb_inspected: Vec<i64> = monkeys.iter().map(|m| m.nb_inspected).collect();
    nb_inspected.sort();
    nb_inspected.reverse();

    nb_inspected[0] * nb_inspected[1]
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2022/day11/input.txt").unwrap();
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();
    let nb_monkeys = monkeys.len();
    let mut lcm: i64 = 1;
    for monkey in &monkeys {
        lcm = integer::lcm(lcm, monkey.divisibility as i64);
    }

    // play 10000 rounds
    for _ in 0..10000 {
        // play each monkey's turn
        for i in 0..nb_monkeys {
            while !monkeys[i].items.is_empty() {
                let m = &mut monkeys[i];
                let mut item = m.items.pop().unwrap();
                m.nb_inspected += 1;
                item = m.operation.exec(item);
                item = item % lcm;
                let target_monkey = if (item % m.divisibility) == 0 {
                    m.on_true
                } else {
                    m.on_false
                };
                monkeys[target_monkey].items.push(item);
            }
        }
    }

    let mut nb_inspected: Vec<i64> = monkeys.iter().map(|m| m.nb_inspected).collect();
    nb_inspected.sort();
    nb_inspected.reverse();

    nb_inspected[0] * nb_inspected[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 54752);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 13606755504);
    }
}
