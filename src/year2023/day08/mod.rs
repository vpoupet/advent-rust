use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{delimited, separated_pair},
    IResult,
};
use num::Integer;

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
}

fn parse_input() -> (Vec<usize>, HashMap<String, Vec<String>>) {
    let input = utils::read_input("src/year2023/day08/input.txt").unwrap();
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let _ = lines.next();
    let mut map = HashMap::new();
    for line in lines {
        let (_, (node, (left, right))) = parse_line(line).unwrap();
        map.insert(node.to_string(), vec![left.to_string(), right.to_string()]);
    }
    (directions, map)
}

fn next_node<'a>(
    map: &'a HashMap<String, Vec<String>>,
    node: &str,
    directions: &Vec<usize>,
    dir_index: usize,
) -> &'a str {
    let dir = directions[dir_index % directions.len()];
    &map.get(node).unwrap()[dir]
}

pub fn solve1() -> usize {
    let (directions, map) = parse_input();
    let mut counter = 0;
    let mut node = "AAA";

    while node != "ZZZ" {
        node = next_node(&map, &node, &directions, counter);
        counter += 1;
    }
    counter
}

fn find_period(
    map: &HashMap<String, Vec<String>>,
    start_node: &str,
    directions: &Vec<usize>,
) -> (usize, usize) {
    let mut counter = 0;
    let mut node = start_node;
    let len = directions.len();

    while !node.ends_with("Z") {
        node = next_node(map, node, &directions, counter);
        counter += 1;
    }

    let n = counter;
    node = next_node(map, node, &directions, counter);
    counter += 1;
    while !(node.ends_with("Z") && (counter - n) % len == 0) {
        node = next_node(map, node, &directions, counter);
        counter += 1;
    }

    (n, counter - n)
}

pub fn solve2() -> usize {
    let (directions, map) = parse_input();
    let mut m = 1;
    for (node, _) in &map {
        if node.ends_with("A") {
            let (_, p) = find_period(&map, node, &directions);
            m = m.lcm(&p);
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 13207);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 12324145107121);
    }
}
