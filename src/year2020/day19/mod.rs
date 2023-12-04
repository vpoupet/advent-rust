use regex::Regex;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::utils;

#[derive(Debug)]
enum Rule {
    Char(char),
    Or(Vec<Vec<i32>>),
}

fn parse_rule(input: &str) -> IResult<&str, (i32, Rule)> {
    separated_pair(
        utils::parse_int,
        tag(": "),
        alt((
            map(delimited(tag("\""), one_of("ab"), tag("\"")), |c| {
                Rule::Char(c)
            }),
            map(
                separated_list1(tag(" | "), separated_list1(tag(" "), utils::parse_int)),
                |v| Rule::Or(v),
            ),
        )),
    )(input)
}

fn make_rules() -> HashMap<i32, Rule> {
    let input = utils::read_input("src/year2020/day19/rules.txt").unwrap();
    let mut rules = HashMap::new();
    for line in input.lines() {
        let (_, (i, r)) = parse_rule(line).unwrap();
        rules.insert(i, r);
    }
    rules
}

fn get_string_re(rules: &HashMap<i32, Rule>, i: i32) -> String {
    match rules.get(&i).unwrap() {
        Rule::Char(c) => c.to_string(),
        Rule::Or(v) => {
            let mut strings = Vec::new();
            for seq in v {
                let mut s = String::new();
                for n in seq {
                    s.push_str(&get_string_re(rules, *n));
                }
                strings.push(s);
            }
            return format!("(?:{})", strings.join("|"));
        }
    }
}

pub fn solve1() -> i32 {
    let rules = make_rules();
    let s = get_string_re(&rules, 0);
    let re = Regex::new(&format!("^({})$", s)).unwrap();

    let words = utils::read_input("src/year2020/day19/words.txt").unwrap();
    let mut total = 0;
    for word in words.lines() {
        if re.is_match(word) {
            total += 1;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let rules = make_rules();
    
    let s_42 = get_string_re(&rules, 42);
    let s_31 = get_string_re(&rules, 31);
    let re_42 = Regex::new(&s_42).unwrap();
    let re_31 = Regex::new(&s_31).unwrap();
    let re = Regex::new(&format!("^((?:{})+)((?:{})+)$", s_42, s_31)).unwrap();

    let words = utils::read_input("src/year2020/day19/words.txt").unwrap();
    let mut total = 0;
    for word in words.lines() {
        if re.is_match(word) {
            let captures = re.captures(word).unwrap();
            let n = re_42.find_iter(captures.get(1).unwrap().as_str()).count();
            let m = re_31.find_iter(captures.get(2).unwrap().as_str()).count();
            if n > m {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 178);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 346);
    }
}
