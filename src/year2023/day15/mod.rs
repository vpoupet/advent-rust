use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    sequence::pair,
    IResult,
};

use crate::utils;

#[derive(Debug, PartialEq)]
enum StepAction {
    Remove,
    Insert(usize),
}

struct Lens {
    label: String,
    focal_length: usize,
}
impl Lens {
    fn new(label: String, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

fn parse_input(filename: &str) -> Vec<String> {
    let input = utils::read_input(filename).unwrap();
    let steps = input.trim_end().split(',').map(|s| s.to_string()).collect();
    steps
}

fn parse_step(input: &str) -> IResult<&str, (String, StepAction)> {
    pair(
        map(alpha1, |s: &str| s.to_string()),
        alt((
            map(tag("-"), |_| StepAction::Remove),
            map(pair(tag("="), utils::parse_unsigned_int), |(_, i)| {
                StepAction::Insert(i)
            }),
        )),
    )(input)
}

fn hash(input: &str) -> usize {
    let mut acc: u8 = 0;
    for c in input.chars() {
        acc = acc.wrapping_add(c as u8);
        acc = acc.wrapping_mul(17);
    }
    acc as usize
}

pub fn solve1() -> usize {
    let steps = parse_input("src/year2023/day15/input.txt");
    let mut total = 0;
    for step in steps {
        total += hash(&step);
    }
    total
}

pub fn solve2() -> usize {
    let steps = parse_input("src/year2023/day15/input.txt");
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for step in steps {
        let (_, (label, action)) = parse_step(&step).unwrap();
        let current_box = &mut boxes[hash(&label)];
        match action {
            StepAction::Remove => {
                if let Some(i) = current_box.iter().position(|lens| lens.label == label) {
                    current_box.remove(i);
                }
            }
            StepAction::Insert(f) => {
                if let Some(i) = current_box.iter().position(|lens| lens.label == label) {
                    // replace existing lens (change focal length)
                    current_box[i].focal_length = f;
                } else {
                    current_box.push(Lens::new(label, f));
                }
            }
        }
    }

    // get total focusing power
    let mut total = 0;
    for (box_index, box_) in boxes.iter().enumerate() {
        for (lens_index, f) in box_.iter().enumerate() {
            total += (box_index + 1) * (lens_index + 1) * f.focal_length;
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
        assert_eq!(solution, 494980);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 247933);
    }
}
