use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::utils;

#[derive(Debug)]
enum Cubes {
    Red(i32),
    Green(i32),
    Blue(i32),
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    alt((
        map(terminated(utils::parse_int, tag(" red")), |v| Cubes::Red(v)),
        map(terminated(utils::parse_int, tag(" green")), |v| {
            Cubes::Green(v)
        }),
        map(terminated(utils::parse_int, tag(" blue")), |v| {
            Cubes::Blue(v)
        }),
    ))(input)
}

fn parse_draw(input: &str) -> IResult<&str, Vec<Cubes>> {
    separated_list1(tag(", "), parse_cubes)(input)
}

fn parse_line(input: &str) -> IResult<&str, (i32, Vec<Vec<Cubes>>)> {
    separated_pair(
        preceded(tag("Game "), utils::parse_int),
        tag(": "),
        separated_list1(tag("; "), parse_draw),
    )(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day02/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, (game_id, draws)) = parse_line(line).unwrap();
        let mut is_valid = true;
        for draw in draws {
            for cubes in draw {
                match cubes {
                    Cubes::Red(v) => is_valid &= v <= 12,
                    Cubes::Green(v) => is_valid &= v <= 13,
                    Cubes::Blue(v) => is_valid &= v <= 14,
                }
            }
        }
        if is_valid {
            total += game_id;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/day02/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, (_, draws)) = parse_line(line).unwrap();
        let mut nb_red = 0;
        let mut nb_green = 0;
        let mut nb_blue = 0;
        for draw in draws {
            for cubes in draw {
                match cubes {
                    Cubes::Red(v) => nb_red = nb_red.max(v),
                    Cubes::Green(v) => nb_green = nb_green.max(v),
                    Cubes::Blue(v) => nb_blue = nb_blue.max(v),
                }
            }
        }
        total += nb_red * nb_green * nb_blue;
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
        assert_eq!(solution, 2204);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 71036);
    }
}
