use std::collections::HashMap;

use nom::{ bytes::complete::tag, multi::separated_list1 };

use crate::utils::{ self, parse_unsigned_int };

fn apply_rule(x: i64) -> Vec<i64> {
    if x == 0 {
        return vec![1];
    }
    let s = x.to_string();
    if s.len() % 2 == 0 {
        return vec![s[..s.len() / 2].parse().unwrap(), s[s.len() / 2..].parse().unwrap()];
    }
    return vec![x * 2024];
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2024/day11/input.txt").unwrap();
    let (_, mut values) = separated_list1(tag(" "), parse_unsigned_int::<i64>)(&input).unwrap();
    for _ in 0..25 {
        let mut new_values = Vec::new();
        for v in values {
            new_values.extend(apply_rule(v));
        }
        values = new_values;
    }
    values.len() as i64
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2024/day11/input.txt").unwrap();
    let (_, input_values) = separated_list1(tag(" "), parse_unsigned_int::<i64>)(&input).unwrap();

    let mut count = HashMap::new();
    for v in input_values {
        *count.entry(v).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_count = HashMap::new();
        for v in count.keys() {
            for x in apply_rule(*v) {
                *new_count.entry(x).or_insert(0) += count[v];
            }
        }
        count = new_count;
    }
    count.values().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 191690);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 228651922369703);
    }
}
