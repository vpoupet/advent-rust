use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    sequence::{delimited, preceded},
    IResult,
};

use crate::utils;

fn as_int(bits: &Vec<bool>) -> u32 {
    let mut result: u32 = 0;
    for bit in bits {
        result <<= 1;
        if *bit {
            result |= 1;
        }
    }
    result.min(result.reverse_bits() >> 22)
}

fn reverse(mut n: u32) -> u32 {
    n.reverse_bits() >> 22
}

struct Tile {
    id: i64,
    data: Vec<Vec<bool>>,
    top_word: u32,
    bottom_word: u32,
    left_word: u32,
    right_word: u32,
}
impl Tile {
    fn new(id: i64, data: Vec<Vec<bool>>) -> Tile {
        let top = as_int(&data[0]);
        let right = as_int(data.iter().map(|row| row[row.len() - 1]).collect());
        Tile {
            id,
            data,
            top_word: as_int(top),
            bottom_word: as_int(bottom),
            left_word: as_int(&left),
            right_word: as_int(&right),
        }
    }
    fn get_sides(&self) -> Vec<u32> {
        let mut sides = Vec::new();
        let top = &self.data[0];
        sides.push(as_int(top));
        let bottom = &self.data[self.data.len() - 1];
        sides.push(as_int(bottom));
        let left = self.data.iter().map(|row| row[0]).collect();
        sides.push(as_int(&left));
        let right = self.data.iter().map(|row| row[row.len() - 1]).collect();
        sides.push(as_int(&right));
        sides
    }
}

fn parse_tile_header(input: &str) -> IResult<&str, i64> {
    delimited(tag("Tile "), utils::parse_int, tag(":"))(input)
}

fn parse_input() -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut current_index = 0;
    let mut current_data = Vec::new();

    let input = utils::read_input("src/year2020/day20/input.txt").unwrap();
    for line in input.lines() {
        if line.starts_with("Tile") {
            current_index = parse_tile_header(line).unwrap().1;
        } else if line.is_empty() {
            tiles.push(Tile {
                id: current_index,
                data: current_data,
            });
            current_data = Vec::new();
        } else {
            current_data.push(line.chars().map(|c| c == '#').collect());
        }
    }
    tiles.push(Tile {
        id: current_index,
        data: current_data,
    });
    tiles
}

pub fn solve1() -> i64 {
    let tiles = parse_input();
    let mut counter = HashMap::new();
    for tile in &tiles {
        for side in tile.get_sides() {
            let count = counter.entry(side).or_insert(0);
            *count += 1;
        }
    }

    let mut total = 1;
    for tile in &tiles {
        let mut nb_unique_sides = 0;
        for side in tile.get_sides() {
            if counter[&side] < 2 {
                nb_unique_sides += 1;
            }
        }
        if nb_unique_sides >= 2 {
            total *= tile.id;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/dayXX/input.txt").unwrap();
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
