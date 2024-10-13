use std::collections::HashMap;

use nom::IResult;
use nom::character::complete::{alpha1, char, one_of};
use crate::utils::{self, parse_unsigned_int};

struct WorkflowRule {
    parameter: char,
    comparator: char,
    value: u32,
    target: WorkflowOutput,
}

enum WorkflowOutput {
    Accept,
    Reject,
    Redirect(String),
}

fn parse_name(input: &str) -> IResult<&str, String> {
    // reads a non empty sequence of lowercase letters
    let (input, name) = nom::character::complete::alpha1(input)?;
    Ok((input, name.to_string()))
}

fn parse_rule(input: &str) -> IResult<&str, WorkflowRule> {
    let (input, parameter) = one_of("xmas")(input)?;
    let (input, comparator) = one_of("<>")(input)?;
    let (input, value) = parse_unsigned_int(input)?;
    let (input, _) = char(':')(input)?;
    let (input, target) = alpha1(input)?;

    let rule = WorkflowRule {
        parameter,
        comparator,
        value,
        target: match target {
            "A" => WorkflowOutput::Accept,
            "R" => WorkflowOutput::Reject,
            _ => WorkflowOutput::Redirect(target.to_string()),
        },
    };
    Ok((input, rule))
}

fn parse_input(input: &str) -> Vec<String> {
    let sections = input.split("\n\n").map(|section| section.to_string()).collect::<Vec<String>>();
    
    let workflows: HashMap<String, Vec<WorkflowRule>> = HashMap::new();
    for line in sections[0].lines() {
        let (input, name) = parse_name(line).unwrap();
        let (input, rules) = nom::multi::many1(parse_rule)(input).unwrap();
        workflows.insert(name, rules);
    }
    vec![]
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
    parse_input(&input);
    0
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
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
