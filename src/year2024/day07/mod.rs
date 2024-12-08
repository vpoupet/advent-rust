use nom::{ bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult };

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(utils::parse_int, tag(": "), separated_list1(tag(" "), utils::parse_int))(input)
}

fn can_solve_1(target: i64, operands: Vec<i64>) -> bool {
    if target < operands[0] {
        return false;
    }

    if operands.len() == 1 {
        return operands[0] == target;
    }

    // try multiplication
    if
        can_solve_1(
            target,
            std::iter
                ::once(operands[0] * operands[1])
                .chain(operands[2..].iter().cloned())
                .collect()
        )
    {
        return true;
    }

    // try addition
    if
        can_solve_1(
            target,
            std::iter
                ::once(operands[0] + operands[1])
                .chain(operands[2..].iter().cloned())
                .collect()
        )
    {
        return true;
    }

    false
}

fn can_solve_2(target: i64, operands: Vec<i64>) -> bool {
    if target < operands[0] {
        return false;
    }

    if operands.len() == 1 {
        return operands[0] == target;
    }

    // try multiplication
    if
        can_solve_2(
            target,
            std::iter
                ::once(operands[0] * operands[1])
                .chain(operands[2..].iter().cloned())
                .collect()
        )
    {
        return true;
    }

    // try addition
    if
        can_solve_2(
            target,
            std::iter
                ::once(operands[0] + operands[1])
                .chain(operands[2..].iter().cloned())
                .collect()
        )
    {
        return true;
    }

    // try concatenation
    let str_0 = operands[0].to_string();
    let str_1 = operands[1].to_string();
    let concatenated = format!("{}{}", str_0, str_1);
    let x = concatenated.parse::<i64>().unwrap();
    if
        can_solve_2(
            target,
            std::iter
                ::once(x)
                .chain(operands[2..].iter().cloned())
                .collect()
        )
    {
        return true;
    }

    return false;
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2024/day07/input.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let (_, (target, operands)) = parse_line(line).unwrap();
        if can_solve_1(target, operands) {
            total += target;
        }
    }
    total
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2024/day07/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, (target, operands)) = parse_line(line).unwrap();
        if can_solve_2(target, operands) {
            total += target;
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
        assert_eq!(solution, 1620690235709);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 145397611075341);
    }
}
