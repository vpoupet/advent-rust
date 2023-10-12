use crate::utils;
use nom::character::complete::char;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::sequence::{pair, terminated};
use nom::IResult;

fn parser(input: &str) -> IResult<&str, (i32, i32)> {
    let line_parser = pair(terminated(one_of("ABC"), char(' ')), one_of("XYZ"));
    map(line_parser, |(first, second)| {
        (first as i32 - 'A' as i32, second as i32 - 'X' as i32)
    })(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/day02/input.txt").unwrap();
    let mut score = 0;
    
    for line in input.lines() {
        let (_, (first, second)) = parser(line).unwrap();
        score += second + 1;
        match (3 + second - first) % 3 {
            0 => score += 3,
            1 => score += 6,
            _ => {}
        }
    }

    score
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/day02/input.txt").unwrap();
    let mut score = 0;
    
    for line in input.lines() {
        let (_, (first, second)) = parser(line).unwrap();
        match second {
            0 => { // should lose
                score += (first + 2) % 3 + 1;
            },
            1 => { // should draw
                score += 3 + first + 1;
            },
            2 => {
                // should win
                score += 6 + (first + 1) % 3 + 1;
            },
            _ => {}
        }
    }

    score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        println!("Part One: {}", solve1());
        assert_eq!(solve1(), 12645);
    }

    #[test]
    fn test_solve2() {
        println!("Part Two: {}", solve2());
        assert_eq!(solve2(), 11756);
    }
}
