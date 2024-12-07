use nom::{ bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult };
use crate::utils;

fn parse_input(input: &str) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    separated_pair(
        separated_list1(tag("\n"), separated_pair(utils::parse_int, tag("|"), utils::parse_int)),
        tag("\n\n"),
        separated_list1(tag("\n"), separated_list1(tag(","), utils::parse_int))
    )(input)
}

fn is_valid(production: &Vec<i32>, pairs: &Vec<(i32, i32)>) -> bool {
    for pair in pairs {
        let index0 = production.iter().position(|&x| x == pair.0);
        let index1 = production.iter().position(|&x| x == pair.1);
        if let (Some(index0), Some(index1)) = (index0, index1) {
            if index0 > index1 {
                return false;
            }
        }
    }
    true
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day05/input.txt").unwrap();
    let (_, (pairs, productions)) = parse_input(&input).unwrap();

    let mut total = 0;
    for production in productions {
        if is_valid(&production, &pairs) {
            total += production.get(production.len() / 2).unwrap();
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day05/input.txt").unwrap();
    let (_, (pairs, productions)) = parse_input(&input).unwrap();

    let mut total = 0;
    for mut production in productions {
        if !is_valid(&production, &pairs) {
            production.sort_by(|a, b| {
                for (p1, p2) in &pairs {
                    if a == p1 && b == p2 {
                        return std::cmp::Ordering::Less;
                    } else if a == p2 && b == p1 {
                        return std::cmp::Ordering::Greater;
                    }
                }
                return std::cmp::Ordering::Equal;
            });
            total += production.get(production.len() / 2).unwrap();
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
        assert_eq!(solution, 5129);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 4077);
    }
}
