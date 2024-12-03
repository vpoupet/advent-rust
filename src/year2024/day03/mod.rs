use nom::{ bytes::complete::tag, combinator::map, sequence::tuple, IResult };
use regex::Regex;

use crate::utils::{ self, parse_unsigned_int };

struct Multiplication {
    left: i32,
    right: i32,
}

impl Multiplication {
    fn new(left: i32, right: i32) -> Self {
        Self {
            left,
            right,
        }
    }

    fn eval(&self) -> i32 {
        self.left * self.right
    }
}

fn parse_multiplication(input: &str) -> IResult<&str, Multiplication> {
    map(
        tuple((
            tag("mul("),
            parse_unsigned_int,
            tag(","),
            parse_unsigned_int,
            tag(")"),
        )),
        |(_1, left, _2, right, _3)| Multiplication::new(left, right)
    )(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day03/input.txt").unwrap();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut total = 0;
    for m in re.find_iter(&input) {
        let (_, mult) = parse_multiplication(m.as_str()).unwrap();
        total += mult.eval();
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day03/input.txt").unwrap();

    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut total = 0;
    let mut is_active = true;
    for m in re.find_iter(&input) {
        if m.as_str() == "do()" {
            is_active = true;
        } else if m.as_str() == "don't()" {
            is_active = false;
        } else {
            if is_active {
                let (_, mult) = parse_multiplication(m.as_str()).unwrap();
                total += mult.eval();
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
        assert_eq!(solution, 160672468);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 84893551);
    }
}
