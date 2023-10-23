use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::map,
    multi::many1,
    IResult,
};

use crate::utils;

#[derive(Debug)]
enum Instruction {
    Rotate(i32),
    Move(i32),
}

fn parse_move(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(one_of("RL"), |c| match c {
            'R' => Instruction::Rotate(1),
            'L' => Instruction::Rotate(-1),
            _ => panic!("Invalid rotation"),
        }),
        map(digit1, |s: &str| {
            Instruction::Move(s.parse::<i32>().unwrap())
        }),
    ))(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_move)(input)
}

fn parse_map(filename: &str) -> Vec<Vec<u8>> {
    let map_input = utils::read_input(filename).unwrap();
    let mut map = map_input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_width = map.iter().map(|l| l.len()).max().unwrap();
    for line in map.iter_mut() {
        while line.len() < max_width {
            line.push(' ' as u8);
        }
    }
    map
}

pub fn solve1() -> i32 {
    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut direction_index = 0;

    // parse instructions
    let (_, instructions) =
        parse_moves(&utils::read_input("src/day22/instructions.txt").unwrap()).unwrap();

        // parse map
    let map = parse_map("src/day22/map.txt");
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    // find starting position
    let (mut i, mut j) = (0, 0);
    while map[i as usize][j as usize] != b'.' {
        j += 1;
    }

    for instruction in instructions {
        match instruction {
            Instruction::Rotate(r) => {
                direction_index = (direction_index + r).rem_euclid(4);
            }
            Instruction::Move(m) => {
                let (di, dj) = directions[direction_index as usize];
                for _ in 0..m {
                    if di != 0 {
                        // move vertically
                        let mut i2 = (i + di).rem_euclid(height as i32);
                        while map[i2 as usize][j as usize] == b' ' {
                            i2 = (i2 + di).rem_euclid(height as i32);
                        }
                        if map[i2 as usize][j as usize] == b'#' {
                            // hit a wall
                            break;
                        } else {
                            i = i2;
                        }
                    } else {
                        // move horizontally
                        let mut j2 = (j + dj).rem_euclid(width as i32);
                        while map[i as usize][j2 as usize] == b' ' {
                            j2 = (j2 + dj).rem_euclid(width as i32);
                        }
                        if map[i as usize][j2 as usize] == b'#' {
                            // hit a wall
                            break;
                        } else {
                            j = j2;
                        }
                    }
                }
            }
        }
    }
    1000 * (i + 1) + 4 * (j + 1) + direction_index
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/dayXX/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 126350);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
