use nom::{
    character::complete::char,
    sequence::{terminated, tuple},
    IResult,
};

use crate::utils::{self, parse_int};

fn parse_line(input: &str) -> IResult<&str, (i32, i32, i32)> {
    tuple((
        terminated(parse_int, char(',')),
        terminated(parse_int, char(',')),
        parse_int,
    ))(input)
}

pub fn solve1() -> i32 {
    let mut grid = vec![vec![vec![false; 22]; 22]; 22];
    let input = utils::read_input("src/day18/input.txt").unwrap();
    for line in input.lines() {
        let (x, y, z) = parse_line(line).unwrap().1;
        grid[x as usize][y as usize][z as usize] = true;
    }
    let neighbors: Vec<(i32, i32, i32)> = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    let mut counter = 0;
    for x in 0..=21 {
        for y in 0..=21 {
            for z in 0..=21 {
                if grid[x][y][z] {
                    for (dx, dy, dz) in neighbors.iter() {
                        let x = x as i32 + dx;
                        let y = y as i32 + dy;
                        let z = z as i32 + dz;
                        if x < 0 || x > 21 || y < 0 || y > 21 || z < 0 || z > 21 || !grid[x as usize][y as usize][z as usize] {
                            counter += 1;
                        }
                    }
                }
            }
        }
    }
    counter
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/dayXX/input.txt").unwrap();
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
