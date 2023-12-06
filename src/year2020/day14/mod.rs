use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::utils;

enum Instruction {
    Mask(String),
    Mem(i64, i64),
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Range {
    addr: i64,
    mask: i64,
}

impl Range {
    fn new(addr: i64, mask: i64) -> Self {
        Self {
            addr: addr & !mask,
            mask,
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.addr & !other.mask) == (other.addr & !self.mask)
    }

    fn get_mask_indexes(&self) -> Vec<i64> {
        let mut result = Vec::new();
        for i in 0..64 {
            if self.mask & (1 << i) != 0 {
                result.push(i);
            }
        }
        result
    }

    fn get_addresses(&self) -> Vec<i64> {
        let mut result = Vec::new();
        let indexes = self.get_mask_indexes();
        for i in 0..(1 << indexes.len()) {
            let mut addr = self.addr;
            for (j, index) in indexes.iter().enumerate() {
                if i & (1 << j) != 0 {
                    addr |= 1 << index;
                }
            }
            result.push(addr);
        }
        result
    }

    fn get_nb_addresses(&self) -> i64 {
        let mut nb_addresses = 1;
        for i in 0..64 {
            if self.mask & (1 << i) != 0 {
                nb_addresses *= 2;
            }
        }
        nb_addresses
    }

    fn difference(self, other: &Range) -> Vec<Range> {
        if !self.overlaps(other) {
            return vec![self];
        }

        let mut result = Vec::new();
        let mask = self.mask & other.mask;
        let split_mask = self.mask & !other.mask;
        for addr in Range::new(self.addr, split_mask).get_addresses() {
            if addr & split_mask != other.addr & split_mask {
                result.push(Range::new(addr, mask));
            }
        }
        result
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in (0..64).rev() {
            if self.mask & (1 << i) != 0 {
                s.push('X');
            } else if self.addr & (1 << i) != 0 {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        write!(f, "{}", s)
    }
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("mask = "), is_a("01X")), |s: &str| {
            Instruction::Mask(s.to_string())
        }),
        map(
            separated_pair(
                preceded(tag("mem["), utils::parse_int),
                tag("] = "),
                utils::parse_int,
            ),
            |(a, b)| Instruction::Mem(a, b),
        ),
    ))(input)
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2020/day14/input.txt").unwrap();
    let mut memory = HashMap::new();
    let mut bitmask_0 = 0;
    let mut bitmask_1 = 0;
    for line in input.lines() {
        let (_, instruction) = parse_line(line).unwrap();
        match instruction {
            Instruction::Mask(mask) => {
                bitmask_0 = 0; // positions of 0s in the mask
                bitmask_1 = 0; // positions of 1s in the mask
                for (i, c) in mask.chars().rev().enumerate() {
                    if c == '0' {
                        bitmask_0 |= 1 << i;
                    } else if c == '1' {
                        bitmask_1 |= 1 << i;
                    }
                }
            }
            Instruction::Mem(addr, value) => {
                let value = (value & !bitmask_0) | bitmask_1;
                memory.insert(addr, value);
            }
        }
    }
    memory.values().sum()
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2020/day14/input.txt").unwrap();
    let mut memory: HashMap<Range, i64> = HashMap::new();
    let mut bitmask_1 = 0;
    let mut bitmask_x = 0;
    for line in input.lines() {
        let (_, instruction) = parse_line(line).unwrap();
        match instruction {
            Instruction::Mask(mask_string) => {
                // update mask
                bitmask_1 = 0;
                bitmask_x = 0;
                for (i, c) in mask_string.chars().rev().enumerate() {
                    if c == '1' {
                        bitmask_1 |= 1 << i;
                    }
                    if c == 'X' {
                        bitmask_x |= 1 << i;
                    }
                }
            }
            Instruction::Mem(addr, value) => {
                // add values in memory
                let new_range = Range::new(addr | bitmask_1, bitmask_x);
                // insert value in addresses of specified range
                let mut new_memory = HashMap::new();
                for (r, v) in memory.into_iter() {
                    for r in r.difference(&new_range) {
                        new_memory.insert(r, v);
                    }
                }
                new_memory.insert(new_range, value);
                memory = new_memory;
            }
        }
    }

    let mut total = 0;
    for (range, value) in memory.iter() {
        total += range.get_nb_addresses() * value;
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
        assert_eq!(solution, 7477696999511);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 3687727854171);
    }
}
