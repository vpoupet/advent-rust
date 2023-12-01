use std::collections::HashMap;

use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day01/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut c1: Option<char> = None;
        let mut c2: Option<char> = None;
        for c in line.chars() {
            if c.is_numeric() {
                if c1.is_none() {
                    c1 = Some(c);
                }
                c2 = Some(c);
            }
        }
        if let (Some(c1), Some(c2)) = (c1, c2) {
            let s = c1.to_string() + &c2.to_string();
            total += s.parse::<i32>().unwrap();
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/day01/input.txt").unwrap();
    let mut total = 0;

    let digit_names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut digits: HashMap<String, i32> = HashMap::new();
    for i in 1..10 {
        digits.insert(i.to_string(), i as i32);
        digits.insert(digit_names[i-1].to_owned(), i as i32);
    }

    for line in input.lines() {
        let mut c1: Option<i32> = None;
        let mut c2: Option<i32> = None;
        for i in 0..line.len() {
            for key in digits.keys() {
                if line[i..].starts_with(key) {
                    if c1.is_none() {
                        c1 = Some(digits[key]);
                    }
                    c2 = Some(digits[key]);
                    break;
                }
            }
        }
        if let (Some(c1), Some(c2)) = (c1, c2) {
            total += 10 * c1 + c2;
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
        assert_eq!(solution, 55621);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 53592);
    }
}
