use std::collections::HashSet;

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
                        if x < 0
                            || x > 21
                            || y < 0
                            || y > 21
                            || z < 0
                            || z > 21
                            || !grid[x as usize][y as usize][z as usize]
                        {
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
    let input = utils::read_input("src/day18/input.txt").unwrap();
    let mut points = Vec::new();
    let mut max_coordinate = 0;
    for line in input.lines() {
        let (x, y, z) = parse_line(line).unwrap().1;
        let (x, y, z) = (x as usize, y as usize, z as usize);
        max_coordinate = max_coordinate.max(x).max(y).max(z);
        points.push((x as usize, y as usize, z as usize));
    }
    let grid_size = max_coordinate + 3;
    let mut grid = vec![vec![vec![false; grid_size]; grid_size]; grid_size];
    for (x, y, z) in points {
        grid[x + 1][y + 1][z + 1] = true;
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

    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push((0, 0, 0));
    visited.insert((0, 0, 0));

    while !stack.is_empty() {
        let (x, y, z) = stack.pop().unwrap();
        for (dx, dy, dz) in &neighbors {
            let nx = x + dx;
            let ny = y + dy;
            let nz = z + dz;
            if nx < 0
                || nx >= grid_size as i32
                || ny < 0
                || ny >= grid_size as i32
                || nz < 0
                || nz >= grid_size as i32
            {
                continue;
            } else if grid[nx as usize][ny as usize][nz as usize] {
                counter += 1;
            } else {
                if !visited.contains(&(nx, ny, nz)) {
                    stack.push((nx, ny, nz));
                    visited.insert((nx, ny, nz));
                }
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 4512);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 2554);
    }
}
