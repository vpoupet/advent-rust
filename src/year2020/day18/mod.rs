use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many1, IResult};

use crate::utils;

#[derive(Debug)]
enum Token {
    Num(i64),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Token>> {
    many1(alt((
        map(tag(" + "), |_| Token::Add),
        map(tag(" * "), |_| Token::Mul),
        map(tag("("), |_| Token::OpenParen),
        map(tag(")"), |_| Token::CloseParen),
        map(utils::parse_int, |n| Token::Num(n as i64)),
    )))(input)
}

fn simplify(tokens: &mut Vec<Token>) {
    let mut should_continue = true;
    while should_continue {
        should_continue = false;
        if let Token::Num(x) = tokens[0] {
            if let Token::Num(y) = tokens[2] {
                if let Token::Add = tokens[1] {
                    tokens.splice(0..3, vec![Token::Num(x + y)]);
                    should_continue = true;
                    continue;
                }
                if let Token::Mul = tokens[1] {
                    tokens.splice(0..3, vec![Token::Num(x * y)]);
                    should_continue = true;
                    continue;
                }
            }
        }
    }
    
    let mut i = 0;
    while i + 2 < tokens.len() {
        let mut should_increment = true;
        match tokens[i] {
            Token::OpenParen => {
                if let Token::Num(v) = tokens[i + 1] {
                    if let Token::CloseParen = tokens[i + 2] {
                        tokens.splice(i..i + 3, vec![Token::Num(v)]);
                        should_increment = false;
                    }
                }
            }
            Token::Num(v) => {
                if let Token::Add = tokens[i + 1] {
                    if let Token::Num(v2) = tokens[i + 2] {
                        tokens.splice(i..i + 3, vec![Token::Num(v + v2)]);
                        should_increment = false;
                    }
                } else if let Token::Mul = tokens[i + 1] {
                    if let Token::Num(v2) = tokens[i + 2] {
                        tokens.splice(i..i + 3, vec![Token::Num(v * v2)]);
                        should_increment = false;
                    }
                }
            }
            _ => {}
        }
        if should_increment {
            i += 1;
        }
    }
}

fn eval(expression: &str) -> i64 {
    let (_, mut tokens) = parse_line(expression).unwrap();
    while tokens.len() > 1 {
        simplify(&mut tokens);
    }
    if let Token::Num(v) = tokens[0] {
        v
    } else {
        panic!("Expected a number");
    }
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2020/day18/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, mut tokens) = parse_line(line).unwrap();
        while tokens.len() > 1 {
            simplify(&mut tokens);
        }
        if let Token::Num(v) = tokens[0] {
            total += v;
        } else {
            panic!("Expected a number");
        }
    }
    total
    // 10588784462817 too high
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/dayXX/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        // assert_eq!(solution, 0);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
