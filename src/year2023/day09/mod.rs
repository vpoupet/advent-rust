<<<<<<< HEAD
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::utils;

fn get_input_sequences() -> Vec<Vec<i64>> {
    let input = utils::read_input("src/year2023/day08/input.txt").unwrap();
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let (_, values) = separated_list1(tag(" "), utils::parse_int)(line).unwrap();
        sequences.push(values);
    }
    sequences
}

fn extrapolate(sequence: Vec<i64>) -> (i64, i64) {
    if sequence.iter().all(|&x| x == sequence[0]) {
        return (sequence[0], sequence[0]);
    }
    let differences: Vec<i64> = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    let (d1, d2) = extrapolate(differences);
    (sequence[0] - d1, sequence[sequence.len() - 1] + d2)
}

pub fn solve1() -> i64 {
    let sequences = get_input_sequences();
    let mut total = 0;
    for seq in sequences {
        total += extrapolate(seq).1;
    }
    total
}

pub fn solve2() -> i64 {
    let sequences = get_input_sequences();
    let mut total = 0;
    for seq in sequences {
        total += extrapolate(seq).0;
    }
    total
=======
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
>>>>>>> 017fd185244b8d3df7d4c5f2be4fe2ff4686c66a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
<<<<<<< HEAD
        assert_eq!(solution, 1702218515);
=======
        assert_eq!(solution, 13207);
>>>>>>> 017fd185244b8d3df7d4c5f2be4fe2ff4686c66a
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
<<<<<<< HEAD
        assert_eq!(solution, 925);
=======
        assert_eq!(solution, 12324145107121);
>>>>>>> 017fd185244b8d3df7d4c5f2be4fe2ff4686c66a
    }
}
