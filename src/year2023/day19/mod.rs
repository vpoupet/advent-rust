use std::collections::HashMap;

use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::character::complete::{ alpha1, char, one_of };
use crate::utils::{ self, parse_unsigned_int };

type MachinePart = [i32; 4];

#[derive(Debug)]
struct WorkflowRule {
    condition: Option<RuleCondition>,
    output: RuleOutput,
}

#[derive(Debug)]
struct RuleCondition {
    category: usize,
    comparator: char,
    value: i32,
}

impl RuleCondition {
    fn evaluate(&self, machine_part: &MachinePart) -> bool {
        let value = machine_part[self.category];
        match self.comparator {
            '<' => value < self.value,
            '>' => value > self.value,
            _ => panic!("Invalid comparator"),
        }
    }
}

#[derive(Debug)]
enum RuleOutput {
    Accept,
    Reject,
    Redirect(String),
}

fn parse_name(input: &str) -> IResult<&str, String> {
    // reads a non empty sequence of lowercase letters
    let (input, name) = nom::character::complete::alpha1(input)?;
    Ok((input, name.to_string()))
}

fn parse_condition(input: &str) -> IResult<&str, RuleCondition> {
    let (input, category) = one_of("xmas")(input)?;
    let (input, comparator) = one_of("<>")(input)?;
    let (input, value) = parse_unsigned_int(input)?;
    let (input, _) = char(':')(input)?;
    Ok((input, RuleCondition { category: "xmas".find(category).unwrap(), comparator, value }))
}

fn parse_rule(input: &str) -> IResult<&str, WorkflowRule> {
    let (input, condition) = opt(parse_condition)(input)?;
    let (input, target) = alpha1(input)?;

    Ok((
        input,
        WorkflowRule {
            condition,
            output: match target {
                "A" => RuleOutput::Accept,
                "R" => RuleOutput::Reject,
                _ => RuleOutput::Redirect(target.to_string()),
            },
        },
    ))
}

fn parse_machine_part(input: &str) -> IResult<&str, MachinePart> {
    let (input, _) = char('{')(input)?;
    let (input, pairs) = separated_list1(
        char(','),
        tuple((one_of("xmas"), char('='), parse_unsigned_int::<i32>))
    )(input)?;
    let (input, _) = char('}')(input)?;

    let mut machine_part = [0; 4];
    for i in 0..4 {
        machine_part[i] = pairs[i].2;
    }
    Ok((input, machine_part))
}

fn parse_input(input: &str) -> (HashMap<String, Vec<WorkflowRule>>, Vec<MachinePart>) {
    let sections = input
        .split("\n\n")
        .map(|section| section.to_string())
        .collect::<Vec<String>>();

    let mut workflows: HashMap<String, Vec<WorkflowRule>> = HashMap::new();
    for line in sections[0].lines() {
        let (_, (name, _, rules)) = tuple((
            parse_name,
            char('{'),
            separated_list1(char(','), parse_rule),
        ))(line).unwrap();
        workflows.insert(name, rules);
    }

    let mut machine_parts: Vec<MachinePart> = Vec::new();
    for line in sections[1].lines() {
        let (_, machine_part) = parse_machine_part(line).unwrap();
        machine_parts.push(machine_part);
    }
    (workflows, machine_parts)
}

fn sort(
    machine_part: &MachinePart,
    workflow: &Vec<WorkflowRule>,
    workflows: &HashMap<String, Vec<WorkflowRule>>
) -> bool {
    for rule in workflow {
        match &rule.condition {
            Some(condition) => {
                if !condition.evaluate(machine_part) {
                    continue;
                }
                match rule.output {
                    RuleOutput::Accept => {
                        return true;
                    }
                    RuleOutput::Reject => {
                        return false;
                    }
                    RuleOutput::Redirect(ref target) => {
                        return sort(machine_part, workflows.get(target).unwrap(), workflows);
                    }
                }
            }
            None => {
                match rule.output {
                    RuleOutput::Accept => {
                        return true;
                    }
                    RuleOutput::Reject => {
                        return false;
                    }
                    RuleOutput::Redirect(ref target) => {
                        return sort(machine_part, workflows.get(target).unwrap(), workflows);
                    }
                }
            }
        }
    }
    false
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
    let (workflows, machine_parts) = parse_input(&input);

    let mut total = 0;
    let initial_workflow = workflows.get("in").unwrap();

    for machine_part in &machine_parts {
        if sort(machine_part, initial_workflow, &workflows) {
            total += machine_part.iter().sum::<i32>();
        }
    }
    total
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
    // let (workflows, _) = parse_input(&input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 383682);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
