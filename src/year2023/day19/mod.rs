use std::collections::HashMap;

use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::character::complete::{ alpha1, char, one_of };
use crate::utils::{ self, parse_unsigned_int };

#[derive(Debug)]
struct WorkflowRule {
    condition: Option<RuleCondition>,
    output: RuleOutput,
}

#[derive(Debug)]
struct RuleCondition {
    parameter: char,
    comparator: char,
    value: i32,
}

impl RuleCondition {
    fn evaluate(&self, object: &HashMap<char, i32>) -> bool {
        let value = object.get(&self.parameter).unwrap();
        match self.comparator {
            '<' => value < &self.value,
            '>' => value > &self.value,
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
    let (input, parameter) = one_of("xmas")(input)?;
    let (input, comparator) = one_of("<>")(input)?;
    let (input, value) = parse_unsigned_int(input)?;
    let (input, _) = char(':')(input)?;
    Ok((input, RuleCondition { parameter, comparator, value }))
}

fn parse_rule(input: &str) -> IResult<&str, WorkflowRule> {
    let (input, condition) = opt(parse_condition)(input)?;
    let (input, target) = alpha1(input)?;

    Ok((input, WorkflowRule {
        condition,
        output: match target {
            "A" => RuleOutput::Accept,
            "R" => RuleOutput::Reject,
            _ => RuleOutput::Redirect(target.to_string()),
        }
    }))
}

fn parse_object(input: &str) -> IResult<&str, HashMap<char, i32>> {
    let (input, _) = char('{')(input)?;
    let (input, pairs) = separated_list1(char(','), tuple((one_of("xmas"), char('='), parse_unsigned_int)))(input)?;
    let (input, _) = char('}')(input)?;

    let mut object = HashMap::new();
    for (key, _, value) in pairs {
        object.insert(key, value);
    }
    Ok((input, object))
}

fn parse_input(input: &str) -> (HashMap<String, Vec<WorkflowRule>>, Vec<HashMap<char, i32>>) {
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

    let mut objects: Vec<HashMap<char, i32>> = Vec::new();
    for line in sections[1].lines() {
        let (_, object) = parse_object(line).unwrap();
        objects.push(object);
    }
    (workflows, objects)
}

fn sort(object: &HashMap<char, i32>, workflow: &Vec<WorkflowRule>, workflows: &HashMap<String, Vec<WorkflowRule>>) -> bool {
    for rule in workflow {
        match &rule.condition {
            Some(condition) => {
                if !condition.evaluate(object) {
                    continue;
                }
                match rule.output {
                    RuleOutput::Accept => return true,
                    RuleOutput::Reject => return false,
                    RuleOutput::Redirect(ref target) => {
                        return sort(object, workflows.get(target).unwrap(), workflows);
                    }
                }
            }
            None => {
                match rule.output {
                    RuleOutput::Accept => return true,
                    RuleOutput::Reject => return false,
                    RuleOutput::Redirect(ref target) => {
                        return sort(object, workflows.get(target).unwrap(), workflows);
                    }
                }
            }
        }
    }
    false
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
    let (workflows, objects) = parse_input(&input);
    
    let mut total = 0;
    let initial_workflow = workflows.get("in").unwrap();

    for object in &objects {
        if sort(object, initial_workflow, &workflows) {
            total += object.get(&'x').unwrap();
            total += object.get(&'m').unwrap();
            total += object.get(&'a').unwrap();
            total += object.get(&'s').unwrap();
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
    let (workflows, _) = parse_input(&input);
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
