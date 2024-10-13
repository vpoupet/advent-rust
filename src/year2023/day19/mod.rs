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

#[derive(Debug, Clone, Copy)]
struct Interval {
    min: i32,
    max: i32,
}

impl Interval {
    fn split(&self, value: i32) -> (Option<Interval>, Option<Interval>) {
        if value <= self.min {
            (None, Some(Interval { min: self.min, max: self.max }))
        } else if value >= self.max {
            (Some(Interval { min: self.min, max: self.max }), None)
        } else {
            (
                Some(Interval { min: self.min, max: value }),
                Some(Interval { min: value, max: self.max }),
            )
        }
    }
}

type CombinationsBox = [Interval; 4];

fn get_box_size(combinations: &CombinationsBox) -> i64 {
    let mut size = 1;
    for interval in combinations {
        size *= (interval.max - interval.min) as i64;
    }
    size
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

/// Counts the number of combinations in a given intervals box that will be accepted if processed through a workflow
/// 
/// # Arguments
/// 
/// * `combinations` - The box of intervals representing the combinations
/// * `workflow` - The workflow used to process the combinations
/// * `rule_index` - The index of the workflow's rule currently processing the combinations
/// * `workflows` - The map of all workflows
fn count_accepted(
    combinations: &CombinationsBox,
    workflow: &Vec<WorkflowRule>,
    rule_index: usize,
    workflows: &HashMap<String, Vec<WorkflowRule>>
) -> i64 {
    let mut total: i64 = 0;
    let rule = &workflow[rule_index];
    match &rule.condition {
        Some(condition) => {
            let passed_combinations: Option<CombinationsBox>; // combinations that pass the rule condition
            let failed_combinations: Option<CombinationsBox>; // combinations that fail the rule condition
            if condition.comparator == '<' {
                let (low, high) = combinations[condition.category].split(condition.value);
                match low {
                    Some(low) => {
                        let mut passed = combinations.clone();
                        passed[condition.category] = low;
                        passed_combinations = Some(passed);
                    }
                    None => {
                        passed_combinations = None;
                    }
                }
                match high {
                    Some(high) => {
                        let mut failed = combinations.clone();
                        failed[condition.category] = high;
                        failed_combinations = Some(failed);
                    }
                    None => {
                        failed_combinations = None;
                    }
                }
            } else {
                let (low, high) = combinations[condition.category].split(condition.value + 1);
                match low {
                    Some(low) => {
                        let mut failed = combinations.clone();
                        failed[condition.category] = low;
                        failed_combinations = Some(failed);
                    }
                    None => {
                        failed_combinations = None;
                    }
                }
                match high {
                    Some(high) => {
                        let mut passed = combinations.clone();
                        passed[condition.category] = high;
                        passed_combinations = Some(passed);
                    }
                    None => {
                        passed_combinations = None;
                    }
                }
            }
            match rule.output {
                // process the combinations that passed the rule condition
                RuleOutput::Accept => {
                    // all combinations that passed the rule condition are accepted
                    if let Some(passed_combinations) = passed_combinations {
                        total += get_box_size(&passed_combinations);
                    }
                }
                RuleOutput::Reject => {} // no combinations are accepted
                RuleOutput::Redirect(ref target) => {
                    // forward the passed combinations to the next workflow
                    if let Some(passed_combinations) = passed_combinations {
                        total += count_accepted(
                            &passed_combinations,
                            workflows.get(target).unwrap(),
                            0,
                            workflows
                        );
                    }
                }
            }
            if let Some(failed_combinations) = failed_combinations {
                // process the combinations that failed the rule condition
                // combinations are passed through the next rule in the current workflow
                total += count_accepted(&failed_combinations, workflow, rule_index + 1, workflows);
            }
        }
        None => {
            // if no condition process all combinations depending on the rule output
            match rule.output {
                RuleOutput::Accept => {
                    // all combinations are accepted
                    total += get_box_size(combinations);
                }
                RuleOutput::Reject => {} // no combinations are accepted
                RuleOutput::Redirect(ref target) => {
                    // forward all combinations to the next workflow
                    total += count_accepted(
                        combinations,
                        workflows.get(target).unwrap(),
                        0,
                        workflows
                    );
                }
            }
        }
    }
    total
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

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2023/day19/input.txt").unwrap();
    let (workflows, _) = parse_input(&input);

    let combinations: CombinationsBox = [
        Interval { min: 1, max: 4001 },
        Interval { min: 1, max: 4001 },
        Interval { min: 1, max: 4001 },
        Interval { min: 1, max: 4001 },
    ];
    count_accepted(&combinations, &workflows.get("in").unwrap(), 0, &workflows)
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
        assert_eq!(solution, 117954800808317);
    }
}
