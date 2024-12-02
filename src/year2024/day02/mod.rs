use nom::{ bytes::complete::tag, multi::separated_list1, IResult };

use crate::utils;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(tag("\n"), separated_list1(tag(" "), utils::parse_int))(input)
}

fn remove_at_index(v: &Vec<i32>, i: usize) -> Vec<i32> {
    let mut result = v.clone();
    result.remove(i);
    result
}

fn is_safe(v: &Vec<i32>) -> bool {
    let should_increase = v[1] > v[0];
    for i in 0..v.len() - 1 {
        let d = (v[i + 1] - v[i]).abs();
        if d < 1 || d > 3 {
            return false;
        }
        if should_increase && v[i + 1] <= v[i] {
            return false;
        }
        if !should_increase && v[i + 1] >= v[i] {
            return false;
        }
    }
    true
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day02/input.txt").unwrap();
    let (_, data) = parse_input(&input).unwrap();
    let mut total = 0;
    for report in &data {
        if is_safe(report) {
            total += 1;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day02/input.txt").unwrap();
    let (_, data) = parse_input(&input).unwrap();
    let mut total = 0;
    for report in &data {
        if is_safe(report) {
            total += 1;
        } else {
            for i in 0..report.len() {
                if is_safe(&remove_at_index(report, i)) {
                    total += 1;
                    break;
                }
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
        assert_eq!(solution, 572);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 612);
    }
}
