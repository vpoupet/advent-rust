use std::collections::HashMap;

use nom::{IResult, sequence::{separated_pair, preceded}, branch::alt, combinator::map, bytes::complete::{tag, is_a}};

use crate::utils;

enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

// struct Range {
//     addr: u64,
//     mask: u64,
// }
// impl Range {
//     fn new(addr: u64, mask: u64) -> Self {
//         Self { addr, mask }
//     }
// }

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("mask = "), is_a("01X")), |s: &str| Instruction::Mask(s.to_string())),
        map(separated_pair(
            preceded(tag("mem["), utils::parse_int),
            tag("] = "),
            utils::parse_int), |(a, b)| Instruction::Mem(a as u64, b as u64)
        ),
    ))(input)
}

pub fn solve1() -> u64 {
    let input = utils::read_input("src/year2020/day14/input.txt").unwrap();
    let mut memory = HashMap::new();
    let mut mask_0 = 0_u64;
    let mut mask_1 = 0_u64;
    for line in input.lines() {
        let (_, instruction) = parse_line(line).unwrap();
        match instruction {
            Instruction::Mask(mask) => {
                mask_0 = 0;
                mask_1 = 0;
                for (i, c) in mask.chars().rev().enumerate() {
                    if c == '0' {
                        mask_0 |= 1 << i;
                    } else if c == '1' {
                        mask_1 |= 1 << i;
                    }
                }
                mask_0 = !mask_0;
            },
            Instruction::Mem(addr, value) => {
                let value = (value & mask_0) | mask_1;
                memory.insert(addr, value);
            },
        }
    }
    memory.values().sum()
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/year2020/day14/input.txt").unwrap();
    // let mut memory = Vec::new();
    // let mut mask_x = 0_u64;
    // let mut mask_1 = 0_u64;
    // for line in input.lines() {
    //     let (_, instruction) = parse_line(line).unwrap();
    //     match instruction {
    //         Instruction::Mask(mask) => {
    //             mask_x = 0;
    //             mask_1 = 0;
    //             for (i, c) in mask.chars().rev().enumerate() {
    //                 if c == 'X' {
    //                     mask_x |= 1 << i;
    //                 } else if c == '1' {
    //                     mask_1 |= 1 << i;
    //                 }
    //             }
    //             mask_x = !mask_x;
    //         },
    //         Instruction::Mem(addr, value) => {
    //             let addr = (addr | mask_1) & mask_x;
    //             memory.push((Range::new(addr, mask_x), value));
    //         },
    //     }
    // }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

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
