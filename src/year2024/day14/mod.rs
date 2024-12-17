use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::utils::{ self, parse_int };

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
impl Robot {
    fn steps(&mut self, time: i32, width: i32, height: i32) {
        self.x = (self.x + self.vx * time).rem_euclid(width);
        self.y = (self.y + self.vy * time).rem_euclid(height);
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(
        tag("\n"),
        map(
            tuple((
                tag("p="),
                parse_int,
                tag(","),
                parse_int,
                tag(" v="),
                parse_int,
                tag(","),
                parse_int,
            )),
            |(_, x, _, y, _, vx, _, vy)| Robot { x, y, vx, vy }
        )
    )(input)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day14/input.txt").unwrap();
    let (_, mut robots) = parse_input(&input).unwrap();

    let width = 101;
    let height = 103;

    for robot in &mut robots {
        robot.steps(100, width, height);
    }
    let mut t0 = 0;
    let mut t1 = 0;
    let mut t2 = 0;
    let mut t3 = 0;

    for robot in &robots {
        if robot.x < width / 2 {
            if robot.y < height / 2 {
                t0 += 1;
            } else if robot.y > height / 2 {
                t1 += 1;
            }
        } else if robot.x > width / 2 {
            if robot.y < height / 2 {
                t2 += 1;
            } else if robot.y > height / 2 {
                t3 += 1;
            }
        }
    }

    t0 * t1 * t2 * t3
}

fn print_grid(robots: &Vec<Robot>, width: i32, height: i32) {
    for i in 0..height {
        for j in 0..width {
            let mut found = false;
            for robot in robots {
                if robot.x == j && robot.y == i {
                    found = true;
                    break;
                }
            }
            if found {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn has_line(robots: &mut Vec<Robot>) -> bool {
    robots.sort_by(|r1, r2| {
        if r1.y == r2.y {
            r1.x.cmp(&r2.x)
        } else {
            r1.y.cmp(&r2.y)
        }
    });

    let mut length = 0;
    for i in 0..robots.len() - 1 {
        if robots[i].y == robots[i + 1].y && robots[i + 1].x == robots[i].x + 1 {
            length += 1;
            if length >= 10 {
                return true;
            }
        } else {
            length = 0;
        }
    }
    false
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day14/input.txt").unwrap();
    let (_, mut robots) = parse_input(&input).unwrap();
    let width = 101;
    let height = 103;

    let mut time = 0;

    loop {
        for robot in &mut robots {
            robot.steps(1, width, height);
        }
        time += 1;

        if has_line(&mut robots) {
            println!("Time: {}", time);
            print_grid(&robots, width, height);
            break;
        }
    }
    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 226179492);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 7502);
    }
}
