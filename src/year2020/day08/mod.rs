use std::collections::HashSet;

use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::preceded, IResult, multi::separated_list1};

use crate::utils;

#[derive(Debug, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Acc(value) => write!(f, "acc {}", value),
            Instruction::Jmp(value) => write!(f, "jmp {}", value),
            Instruction::Nop(value) => write!(f, "nop {}", value),
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("acc "), utils::parse_int), Instruction::Acc),
        map(preceded(tag("jmp "), utils::parse_int), Instruction::Jmp),
        map(preceded(tag("nop "), utils::parse_int), Instruction::Nop),
    ))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), parse_instruction)(input)
}

fn execute(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut pc = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&pc) {
            return (acc, false);
        }
        visited.insert(pc);

        match instructions[pc] {
            Instruction::Acc(value) => {
                acc += value;
                pc += 1;
            }
            Instruction::Jmp(value) => {
                pc = (pc as i32 + value) as usize;
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }

        if pc >= instructions.len() {
            return (acc, true);
        }
    }
}

fn swap(instructions: &mut Vec<Instruction>, i: usize) {
    match instructions[i] {
        Instruction::Acc(_) => (),
        Instruction::Jmp(value) => instructions[i] = Instruction::Nop(value),
        Instruction::Nop(value) => instructions[i] = Instruction::Jmp(value),
    }
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/day08/input.txt").unwrap();
    let (_, instructions) = parse_instructions(&input).unwrap();
    let (acc, _) = execute(&instructions);
    acc
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/day08/input.txt").unwrap();
    let (_, mut instructions) = parse_instructions(&input).unwrap();
    for i in 0..instructions.len() {
        swap(&mut instructions, i);
        let (acc, terminated) = execute(&instructions);
        if terminated {
            return acc;
        }
        swap(&mut instructions, i);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 1867);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 1303);
    }
}
