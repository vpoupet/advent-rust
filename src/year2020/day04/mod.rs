use std::collections::HashMap;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::utils;

fn as_date(input: &str) -> Option<i32> {
    if input.len() != 4 {
        return None;
    }
    match input.parse::<i32>() {
        Ok(v) => Some(v),
        _ => None,
    }
}

fn check_items(items: HashMap<&str, &str>) -> bool {
    for key in ["byr", "iyr", "eyr", "hgt", "hcl", "pid", "ecl"] {
        if !items.contains_key(key) {
            return false;
        }
    }

    let byr = as_date(items.get("byr").unwrap());
    if byr.is_none() || byr.unwrap() < 1920 || byr.unwrap() > 2002 {
        return false;
    }

    let iyr = as_date(items.get("iyr").unwrap());
    if iyr.is_none() || iyr.unwrap() < 2010 || iyr.unwrap() > 2020 {
        return false;
    }

    let eyr = as_date(items.get("eyr").unwrap());
    if eyr.is_none() || eyr.unwrap() < 2020 || eyr.unwrap() > 2030 {
        return false;
    }

    let hgt = items.get("hgt").unwrap();
    if hgt.ends_with("cm") {
        match hgt[..hgt.len() - 2].parse::<i32>() {
            Ok(v) => {
                if v < 150 || v > 193 {
                    return false;
                }
            }
            _ => return false,
        }
    } else if hgt.ends_with("in") {
        match hgt[..hgt.len() - 2].parse::<i32>() {
            Ok(v) => {
                if v < 59 || v > 76 {
                    return false;
                }
            }
            _ => return false,
        }
    } else {
        return false;
    }

    let hcl = items.get("hcl").unwrap();
    if hcl.len() != 7 || !hcl.starts_with("#") || !hcl[1..].chars().all(|c| c.is_digit(16)) {
        return false;
    }

    let ecl = items.get("ecl").unwrap();
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl) {
        return false;
    }

    let pid = items.get("pid").unwrap();
    if pid.len() != 9 || !pid.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    true
}

fn parse_item(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(":"), is_not(" \n"))(input)
}

fn parse_passport(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    let (remainining, data) = separated_list1(one_of(" \n"), parse_item)(input)?;
    let mut items = HashMap::new();
    for (key, value) in data {
        items.insert(key, value);
    }
    Ok((remainining, items))
}

fn parse_input(input: &str) -> IResult<&str, Vec<HashMap<&str, &str>>> {
    separated_list1(tag("\n\n"), parse_passport)(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/day04/input.txt").unwrap();
    let (_, input_data) = parse_input(&input).unwrap();
    let mut counter = 0;
    'outer: for items in input_data {
        for key in ["byr", "iyr", "eyr", "hgt", "hcl", "pid", "ecl"] {
            if !items.contains_key(key) {
                continue 'outer;
            }
        }
        counter += 1;
    }
    counter
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/day04/input.txt").unwrap();
    let (_, input_data) = parse_input(&input).unwrap();
    let mut counter = 0;
    for items in input_data {
        if check_items(items) {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 264);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 224);
    }
}
