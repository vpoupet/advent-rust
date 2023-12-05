use crate::utils;
use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many1, IResult};

#[derive(Debug, PartialEq)]
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
        map(utils::parse_int, |n| Token::Num(n)),
    )))(input)
}

fn eval_operand(tokens: &[Token]) -> (i64, usize) {
    match tokens[0] {
        Token::Num(v) => (v, 1),
        Token::OpenParen => {
            // find end of operand
            let mut depth = 1;
            let mut j = 1;
            while depth > 0 {
                match tokens[j] {
                    Token::OpenParen => depth += 1,
                    Token::CloseParen => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            return (eval_expression(&tokens[1..j - 1]), j);
        }
        _ => panic!("Invalid operand"),
    }
}

fn eval_expression(tokens: &[Token]) -> i64 {
    let (mut acc, mut i) = eval_operand(tokens);
    while i < tokens.len() {
        let (x, j) = eval_operand(&tokens[i + 1..]);
        match tokens[i] {
            Token::Add => acc += x,
            Token::Mul => acc *= x,
            _ => panic!("Invalid operator"),
        }
        i += j + 1;
    }
    acc
}

fn add_priorities(tokens: &mut Vec<Token>) {
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Add = tokens[i] {
            // add parenthesis after second operand
            match tokens[i + 1] {
                Token::Num(_) => {
                    tokens.insert(i + 2, Token::CloseParen);
                }
                Token::OpenParen => {
                    // find position of end of operand
                    let mut depth = 1;
                    let mut j = i + 2;
                    while depth > 0 {
                        match tokens[j] {
                            Token::OpenParen => depth += 1,
                            Token::CloseParen => depth -= 1,
                            _ => {}
                        }
                        j += 1;
                    }
                    tokens.insert(j, Token::CloseParen);
                }
                _ => panic!("Invalid operand"),
            }
            // add parenthesis before first operand
            match tokens[i - 1] {
                Token::Num(_) => {
                    tokens.insert(i - 1, Token::OpenParen);
                }
                Token::CloseParen => {
                    // find position of start of operand
                    let mut depth = 1;
                    let mut j = i - 1;
                    while depth > 0 {
                        match tokens[j-1] {
                            Token::OpenParen => depth -= 1,
                            Token::CloseParen => depth += 1,
                            _ => {}
                        }
                        j -= 1;
                    }
                    tokens.insert(j, Token::OpenParen);
                }
                _ => panic!("Invalid operand"),
            }
            i += 2;
        } else {
            i += 1;
        }
    }
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2020/day18/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, tokens) = parse_line(line).unwrap();
        total += eval_expression(&tokens);
    }
    total
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2020/day18/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, mut tokens) = parse_line(line).unwrap();
        add_priorities(&mut tokens);
        total += eval_expression(&tokens);
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
        assert_eq!(solution, 4491283311856);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 68852578641904);
    }
}
