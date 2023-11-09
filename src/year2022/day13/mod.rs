use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, terminated},
    IResult,
};

use crate::utils;

#[derive(Debug, Eq, Clone)]
enum Message {
    Value(i32),
    List(Vec<Message>),
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Message::Value(v1), Message::Value(v2)) => v1 == v2,
            (Message::Value(v1), Message::List(_)) => {
                Message::List(vec![Message::Value(*v1)]).eq(other)
            }
            (Message::List(_), Message::Value(v2)) => {
                self.eq(&Message::List(vec![Message::Value(*v2)]))
            }
            (Message::List(l1), Message::List(l2)) => l1 == l2,
        }
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Message::Value(v1), Message::Value(v2)) => v1.cmp(v2),
            (Message::Value(v1), Message::List(_)) => {
                Message::List(vec![Message::Value(*v1)]).cmp(other)
            }
            (Message::List(_), Message::Value(v2)) => {
                self.cmp(&Message::List(vec![Message::Value(*v2)]))
            }
            (Message::List(l1), Message::List(l2)) => l1.cmp(l2),
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_int(input: &str) -> IResult<&str, i32> {
    map(digit1, |s: &str| s.parse::<i32>().unwrap())(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Message>> {
    delimited(
        char('['),
        separated_list0(tag(","), parse_message),
        char(']'),
    )(input)
}

fn parse_message(input: &str) -> IResult<&str, Message> {
    alt((
        map(parse_int, Message::Value),
        map(parse_list, Message::List),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Message, Message)>> {
    separated_list1(
        tag("\n"),
        pair(
            terminated(parse_message, tag("\n")),
            terminated(parse_message, tag("\n")),
        ),
    )(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2022/day13/input.txt").unwrap();
    let (_, message_pairs) = parse_input(&input).unwrap();
    let mut total = 0;
    for i in 0..message_pairs.len() {
        let (m1, m2) = &message_pairs[i];
        if m1.partial_cmp(&m2).unwrap() == Ordering::Less {
            total += (i + 1) as i32;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2022/day13/input.txt").unwrap();
    let (_, message_pairs) = parse_input(&input).unwrap();
    let mut messages = Vec::new();
    for (m1, m2) in message_pairs {
        messages.push(m1);
        messages.push(m2);
    }

    let marker1 = Message::List(vec![Message::List(vec![Message::Value(2)])]);
    let marker2 = Message::List(vec![Message::List(vec![Message::Value(6)])]);

    messages.push(marker1.clone());
    messages.push(marker2.clone());
    messages.sort();

    let mut index1 = 0;
    let mut index2 = 0;
    for i in 0..messages.len() {
        if messages[i] == marker1 {
            index1 = i + 1;
        } else if messages[i] == marker2 {
            index2 = i + 1;
        }
    }

    (index1 * index2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 5605);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 24969);
    }
}
