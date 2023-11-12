use std::collections::{HashSet, HashMap};

use nom::{
    bytes::complete::{is_not, tag},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

use crate::utils;

struct Field {
    name: String,
    range1: (i32, i32),
    range2: (i32, i32),
}

impl Field {
    fn new(name: &str, range1: (i32, i32), range2: (i32, i32)) -> Self {
        Self {
            name: name.to_string(),
            range1,
            range2,
        }
    }
}

fn parse_field_line(input: &str) -> IResult<&str, Field> {
    map(
        tuple((
            terminated(is_not(":"), tag(": ")),
            terminated(
                separated_pair(utils::parse_int, tag("-"), utils::parse_int),
                tag(" or "),
            ),
            separated_pair(utils::parse_int, tag("-"), utils::parse_int),
        )),
        |(name, range1, range2)| Field::new(name, range1, range2),
    )(input)
}

fn make_fields() -> Vec<Field> {
    let input = utils::read_input("src/year2020/day16/fields.txt").unwrap();
    let mut fields = Vec::new();
    for line in input.lines() {
        let (_, field) = parse_field_line(line).unwrap();
        fields.push(field);
    }
    fields
}

fn is_valid_field(num: i32, fields: &Vec<Field>) -> bool {
    for field in fields {
        if (num >= field.range1.0 && num <= field.range1.1)
            || (num >= field.range2.0 && num <= field.range2.1)
        {
            return true;
        }
    }
    false
}

fn is_valid_ticket(ticket: &Vec<i32>, fields: &Vec<Field>) -> bool {
    for num in ticket {
        if !is_valid_field(*num, fields) {
            return false;
        }
    }
    true
}

fn parse_ticket(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), utils::parse_int)(input)
}

pub fn solve1() -> i32 {
    let fields = make_fields();
    let tickets_input = utils::read_input("src/year2020/day16/tickets.txt").unwrap();
    let mut tickets = Vec::new();
    for line in tickets_input.lines() {
        let (_, ticket) = parse_ticket(line).unwrap();
        tickets.push(ticket);
    }

    let mut total = 0;
    for ticket in tickets {
        for num in ticket {
            if !is_valid_field(num, &fields) {
                total += num;
            }
        }
    }
    total
}

pub fn solve2() -> i64 {
    let fields = make_fields();
    let tickets_input = utils::read_input("src/year2020/day16/tickets.txt").unwrap();
    let mut tickets = Vec::new();
    for line in tickets_input.lines() {
        let (_, ticket) = parse_ticket(line).unwrap();
        if is_valid_ticket(&ticket, &fields) {
            tickets.push(ticket);
        }
    }

    let mut possibilities = HashMap::new();
    let mut solution = HashMap::new();

    for i in 0..tickets[0].len() {
        let mut possible_fields = HashSet::new();
        for field in &fields {
            possible_fields.insert(field.name.clone());
        }
        for ticket in &tickets {
            for field in &fields {
                if (ticket[i] < field.range1.0 || ticket[i] > field.range1.1)
                    && (ticket[i] < field.range2.0 || ticket[i] > field.range2.1)
                {
                    possible_fields.remove(&field.name);
                }
            }
        }
        possibilities.insert(i, possible_fields);
    }

    while possibilities.len() > 0 {
        let mut solved_indexes = Vec::new();
        let mut solved_fields = Vec::new();
        for (index, possible_fields) in &possibilities {
            if possible_fields.len() == 1 {
                let field_name = possible_fields.iter().next().unwrap().clone();
                solution.insert(*index, field_name.clone());
                solved_indexes.push(*index);
                solved_fields.push(field_name.clone());
            }
        }
        for index in solved_indexes {
            possibilities.remove(&index);
        }
        for field in solved_fields {
            for (_, possible_fields) in &mut possibilities {
                possible_fields.remove(&field);
            }
        }
    }

    let ticket_input = utils::read_input("src/year2020/day16/ticket.txt").unwrap();
    let (_, ticket) = parse_ticket(&ticket_input).unwrap();
    let mut total = 1;
    for (index, field) in &solution {
        if field.starts_with("departure ") {
            total *= ticket[*index] as i64;
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
        assert_eq!(solution, 23009);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 10458887314153);
    }
}
