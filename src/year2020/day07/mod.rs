use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{pair, separated_pair, terminated},
    IResult,
};

use crate::utils;

fn parse_bag_name(input: &str) -> IResult<&str, String> {
    terminated(
        map(separated_pair(alpha1, tag(" "), alpha1), |(c1, c2)| {
            format!("{} {}", c1, c2)
        }),
        pair(tag(" bag"), opt(tag("s"))),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<(i32, String)>)> {
    separated_pair(
        parse_bag_name,
        tag(" contain "),
        alt((
            map(tag("no other bags"), |_| Vec::new()),
            separated_list1(
                tag(", "),
                separated_pair(
                    map(digit1, |s: &str| s.parse::<i32>().unwrap()),
                    tag(" "),
                    parse_bag_name,
                ),
            ),
        )),
    )(input)
}

fn get_rules(filename: &str) -> HashMap<String, Vec<(i32, String)>> {
    let input = utils::read_input(filename).unwrap();
    let mut rules = HashMap::new();
    for line in input.lines() {
        let (_, (bag, contents)) = parse_line(line).unwrap();
        rules.insert(bag, contents);
    }
    rules
}

fn nb_bags(rules: &HashMap<String, Vec<(i32, String)>>, color: &str) -> i32 {
    let mut result = 0;
    for (count, color) in &rules[color] {
        result += count * (1 + nb_bags(rules, color));
    }
    result
}

pub fn solve1() -> i32 {
    let rules = get_rules("src/year2020/day07/input.txt");
    let mut colors = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold".to_string());

    while let Some(color) = queue.pop_front() {
        for (bag_color, contents) in &rules {
            if colors.contains(bag_color) {
                continue;
            }
            if contents.iter().any(|(_, c)| c == &color) {
                queue.push_back(bag_color.clone());
                colors.insert(bag_color.clone());
            }
        }
    }

    colors.len() as i32
}

pub fn solve2() -> i32 {
    let rules = get_rules("src/year2020/day07/input.txt");
    nb_bags(&rules, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 300);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 8030);
    }
}
