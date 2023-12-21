use std::collections::HashSet;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::one_of,
    sequence::{delimited, terminated, tuple},
};

use crate::utils;

#[derive(Debug)]
struct Instruction {
    dx: i64,
    dy: i64,
    distance: i64,
}
impl Instruction {
    fn new(dir: char, distance: i64) -> Self {
        let (dx, dy) = match dir {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid direction"),
        };
        Self { dx, dy, distance }
    }
}

fn parse_line(input: &str) -> (char, i64, &str) {
    tuple((
        terminated(one_of("UDLR"), tag(" ")),
        terminated(utils::parse_int, tag(" ")),
        delimited(tag("(#"), is_a("1234567890abcdef"), tag(")")),
    ))(input)
    .unwrap()
    .1
}

pub fn solve1() -> usize {
    let input = utils::read_input("src/year2023/day18/input.txt").unwrap();
    let mut instructions = Vec::new();
    for line in input.lines() {
        let (dir, dist, _color) = parse_line(line);
        instructions.push(Instruction::new(dir, dist));
    }

    let (mut x, mut min_x, mut max_x) = (0, 0, 0);
    let (mut y, mut min_y, mut max_y) = (0, 0, 0);
    for i in &instructions {
        x += i.dx * i.distance;
        y += i.dy * i.distance;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let width = max_x - min_x + 3;
    let height = max_y - min_y + 3;
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    x = -min_x + 1;
    y = -min_y + 1;

    for i in instructions {
        for _ in 0..i.distance {
            x += i.dx;
            y += i.dy;
            grid[y as usize][x as usize] = '#';
        }
    }

    grid[0][0] = 'o'; // outside
    let mut to_do = vec![(0, 0)];
    while !to_do.is_empty() {
        let (x, y) = to_do.pop().unwrap();
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (x + dx, y + dy);
            if 0 <= x && x < width && 0 <= y && y < height && grid[y as usize][x as usize] == '.' {
                grid[y as usize][x as usize] = 'o';
                to_do.push((x, y));
            }
        }
    }

    grid.into_iter().flatten().filter(|c| *c != 'o').count()
}

fn is_inside(
    x: i64,
    y: i64,
    vertical_segments: &Vec<(i64, i64, i64)>,
    horizontal_segments: &Vec<(i64, i64, i64)>,
) -> bool {
    let mut result = false;
    for (x1, x2, y1) in horizontal_segments {
        if *y1 == y && *x1 <= x && x <= *x2 {
            return true;
        }
    }
    for (x1, y1, y2) in vertical_segments {
        if *x1 == x && *y1 <= y && y <= *y2 {
            return true;
        } else if *x1 > x && *y1 <= y && y < *y2 {
            result = !result;
        }
    }
    result
}

fn column_size(
    x: i64,
    vertical_segments: &Vec<(i64, i64, i64)>,
    horizontal_segments: &Vec<(i64, i64, i64)>,
    values_y: &Vec<i64>,
) -> i64 {
    let mut result = 0;
    for y in values_y {
        if is_inside(x, *y, vertical_segments, horizontal_segments) {
            result += 1;
        }
    }
    for i in 0..&values_y.len() - 1 {
        let y1 = values_y[i];
        if is_inside(x, y1 + 1, vertical_segments, horizontal_segments) {
            result += values_y[i + 1] - y1 - 1;
        }
    }
    result
}

fn polygon_size(
    vertical_segments: &Vec<(i64, i64, i64)>,
    horizontal_segments: &Vec<(i64, i64, i64)>,
    values_x: &Vec<i64>,
    values_y: &Vec<i64>,
) -> i64 {
    let mut result = 0;
    for x in values_x {
        result += column_size(*x, vertical_segments, horizontal_segments, values_y);
    }
    for i in 0..values_x.len() - 1 {
        let x1 = values_x[i];
        result += column_size(x1 + 1, vertical_segments, horizontal_segments, values_y)
            * (values_x[i + 1] - x1 - 1);
    }
    result
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2023/day18/input.txt").unwrap();
    let mut instructions = Vec::new();
    for line in input.lines() {
        let (_, _, color) = parse_line(line);
        let distance = i64::from_str_radix(&color[..color.len() - 1], 16).unwrap();
        match color.chars().last().unwrap() {
            '0' => instructions.push(Instruction::new('R', distance)),
            '1' => instructions.push(Instruction::new('D', distance)),
            '2' => instructions.push(Instruction::new('L', distance)),
            '3' => instructions.push(Instruction::new('U', distance)),
            _ => panic!("Invalid direction"),
        }
    }

    let (mut x, mut y) = (0, 0);
    let mut values_x = HashSet::new();
    let mut values_y = HashSet::new();
    let mut vertical_segments = Vec::new();
    let mut horizontal_segments = Vec::new();
    for i in &instructions {
        if i.dx == 0 {
            let y2 = y + i.dy * i.distance;
            vertical_segments.push((x, y.min(y2), y.max(y2)));
        } else {
            let x2 = x + i.dx * i.distance;
            horizontal_segments.push((x.min(x2), x.max(x2), y));
        }
        x += i.dx * i.distance;
        y += i.dy * i.distance;
        values_x.insert(x);
        values_y.insert(y);
    }
    let mut values_x: Vec<i64> = values_x.into_iter().collect();
    values_x.sort();
    let mut values_y: Vec<i64> = values_y.into_iter().collect();
    values_y.sort();

    polygon_size(
        &vertical_segments,
        &horizontal_segments,
        &values_x,
        &values_y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 50746);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 70086216556038);
    }
}
