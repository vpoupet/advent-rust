use std::collections::HashSet;

use crate::utils;
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
        }
    }
    fn move_head(&mut self, direction: &Point) {
        self.head.x += direction.x;
        self.head.y += direction.y;
        self.move_tail();
    }

    fn move_tail(&mut self) {
        let delta_x = self.head.x - self.tail.x;
        let dx = match delta_x {
            x if x > 0 => 1,
            x if x < 0 => -1,
            _ => 0,
        };
        let delta_y = self.head.y - self.tail.y;
        let dy = match delta_y {
            y if y > 0 => 1,
            y if y < 0 => -1,
            _ => 0,
        };
        if delta_x.abs() >= 2 || delta_y.abs() >= 2 {
            self.tail.x += dx;
            self.tail.y += dy;
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (Point, usize)> {
    separated_pair(
        map(one_of("UDLR"), |x| match x {
            'U' => Point { x: 0, y: 1 },
            'D' => Point { x: 0, y: -1 },
            'L' => Point { x: -1, y: 0 },
            'R' => Point { x: 1, y: 0 },
            _ => panic!("Invalid direction"),
        }),
        char(' '),
        map(digit1, |x: &str| x.parse().unwrap()),
    )(input)
}

pub fn solve1() -> usize {
    let input = utils::read_input("src/day09/input.txt").unwrap();
    let mut rope = Rope::new();
    let mut tail_orbit = HashSet::new();
    for line in input.lines() {
        let (_, (direction, distance)) = parse_line(line).unwrap();
        for _ in 0..distance {
            rope.move_head(&direction);
            tail_orbit.insert(rope.tail.clone());
        }
    }
    tail_orbit.len()
}

// pub fn solve2() -> i32 {
//     let input = utils::read_input("src/day09/input.txt").unwrap();
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        println!("Part One: {}", solve1());
        assert_eq!(solve1(), 6190);
    }

    // #[test]
    // fn test_solve2() {
    //     println!("Part Two: {}", solve2());
    //     assert_eq!(solve2(), 2516);
    // }
}
