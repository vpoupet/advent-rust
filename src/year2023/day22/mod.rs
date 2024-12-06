use nom::{bytes::complete::tag, sequence::{separated_pair, tuple}, IResult};

use crate::utils::{self, parse_int};

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Brick {
    id: usize,
    p1: Point3D,
    p2: Point3D,
}

impl std::fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {} - {}]", self.id, self.p1, self.p2)
    }
}

fn parse_point(input: &str) -> IResult<&str, Point3D> {
    let (remaining, (x, _, y, _, z)) = tuple((
        parse_int, tag(","), parse_int, tag(","), parse_int
    ))(input)?;
    Ok((remaining, Point3D { x, y, z }))
}

fn make_bricks(filename: &str) -> Vec<Brick> {
    let input = utils::read_input(filename).unwrap();
    let mut bricks = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let (_, (p1, p2)) = separated_pair(parse_point, tag("~"), parse_point)(line).unwrap();
        bricks.push(Brick {
            id: i,
            p1,
            p2,
        });
    }
    return bricks;
}

pub fn solve1() -> i32 {
    let bricks = make_bricks("src/year2023/day22/input.txt");
    for brick in bricks {
        println!("{}", brick);
    }
    0
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/year2023/dayXX/input.txt").unwrap();
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
