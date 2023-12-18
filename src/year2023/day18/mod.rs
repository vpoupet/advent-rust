use nom::{
    bytes::complete::{is_a, tag},
    character::complete::one_of,
    combinator::map,
    sequence::{delimited, tuple},
};

use crate::utils;

struct Instruction {
    di: i32,
    dj: i32,
    distance: i32,
    _color: String,
}
impl Instruction {
    fn new(dir: char, distance: i32, color: &str) -> Self {
        let color = color.to_string();
        let (di, dj) = match dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => panic!("Invalid direction"),
        };
        Self {
            di,
            dj,
            distance,
            _color: color,
        }
    }
}

fn parse_line(input: &str) -> Instruction {
    map(
        tuple((
            one_of("UDLR"),
            delimited(tag(" "), utils::parse_int, tag(" ")),
            delimited(tag("("), is_a("#0123456789abcdef"), tag(")")),
        )),
        |(dir, dist, color)| Instruction::new(dir, dist, color),
    )(input)
    .unwrap()
    .1
}

pub fn solve1() -> usize {
    let input = utils::read_input("src/year2023/day18/input.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();

    let (mut x, mut min_x, mut max_x) = (0, 0, 0);
    let (mut y, mut min_y, mut max_y) = (0, 0, 0);
    for i in &instructions {
        x += i.di * i.distance;
        y += i.dj * i.distance;
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
            x += i.di;
            y += i.dj;
            grid[y as usize][x as usize] = '#';
        }
    }

    grid[0][0] = 'o';
    let mut to_do = vec![(0, 0)];
    while !to_do.is_empty() {
        let (x, y) = to_do.pop().unwrap();
        for (di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (x + di, y + dj);
            if 0 <= x && x < width && 0 <= y && y < height && grid[y as usize][x as usize] == '.' {
                grid[y as usize][x as usize] = 'o';
                to_do.push((x, y));
            }
        }
    }

    grid.into_iter().flatten().filter(|c| *c != 'o').count()
}

pub fn solve2() -> i32 {
    let _input = utils::read_input("src/year2023/dayXX/input.txt").unwrap();
    0
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
        // assert_eq!(solution, 0);
    }
}
