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
    knots: Vec<Point>,
}

impl Rope {
    fn new(nb_knots: usize) -> Rope {
        Rope {
            knots: vec![Point::new(0, 0); nb_knots],
        }
    }

    fn move_head(&mut self, direction: &Point) {
        self.knots[0].x += direction.x;
        self.knots[0].y += direction.y;

        for i in 1..self.knots.len() {
            let delta_x = self.knots[i-1].x - self.knots[i].x;
            let dx = match delta_x {
                x if x > 0 => 1,
                x if x < 0 => -1,
                _ => 0,
            };
            let delta_y = self.knots[i-1].y - self.knots[i].y;
            let dy = match delta_y {
                y if y > 0 => 1,
                y if y < 0 => -1,
                _ => 0,
            };
            if delta_x.abs() >= 2 || delta_y.abs() >= 2 {
                self.knots[i].x += dx;
                self.knots[i].y += dy;
            }
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
    let mut rope = Rope::new(2);
    let mut tail_orbit = HashSet::new();
    for line in input.lines() {
        let (_, (direction, distance)) = parse_line(line).unwrap();
        for _ in 0..distance {
            rope.move_head(&direction);
            tail_orbit.insert(rope.knots[1].clone());
        }
    }
    tail_orbit.len()
}

pub fn solve2() -> usize {
    let input = utils::read_input("src/day09/input.txt").unwrap();
    let mut rope = Rope::new(10);
    let mut tail_orbit = HashSet::new();
    for line in input.lines() {
        let (_, (direction, distance)) = parse_line(line).unwrap();
        for _ in 0..distance {
            rope.move_head(&direction);
            tail_orbit.insert(rope.knots[9].clone());
        }
    }
    tail_orbit.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        println!("Part One: {}", solve1());
        assert_eq!(solve1(), 6190);
    }

    #[test]
    fn test_solve2() {
        println!("Part Two: {}", solve2());
        assert_eq!(solve2(), 2516);
    }
}
