use crate::utils;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};
use std::fmt;

struct Cave {
    grid: Vec<Vec<u8>>,
    depth: usize,
    offset: usize,
    width: usize,
}

impl Cave {
    fn new(filename: &str) -> Self {
        let input = utils::read_input(filename).unwrap();
        let (_, paths) = parse_input(&input).unwrap();
        let mut min_x = usize::MAX;
        let mut max_x = 0;
        let mut max_depth = 0;

        for path in &paths {
            for (x, d) in path {
                min_x = min_x.min(*x);
                max_x = max_x.max(*x);
                max_depth = max_depth.max(*d);
            }
        }

        let offset = min_x - 1;
        let width = max_x - min_x + 3;
        let depth = max_depth + 1;
        let mut grid = vec![vec![b'.'; width]; depth];

        for path in &paths {
            for i in 0..path.len() - 1 {
                let (x1, d1) = path[i];
                let (x2, d2) = path[i + 1];
                if d1 == d2 {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid[d1][x - offset] = b'#';
                    }
                } else if x1 == x2 {
                    for d in d1.min(d2)..=d1.max(d2) {
                        grid[d][x1 - offset] = b'#';
                    }
                }
            }
        }

        Self {
            grid,
            depth,
            offset,
            width,
        }
    }

    fn drop_sand(&mut self) -> Option<(usize, usize)> {
        let mut x = 500 - self.offset;
        let mut d = 0;

        while d + 1 < self.depth {
            if self.grid[d + 1][x] == b'.' {
                d += 1;
            } else if self.grid[d + 1][x - 1] == b'.' {
                d += 1;
                x -= 1;
            } else if self.grid[d + 1][x + 1] == b'.' {
                d += 1;
                x += 1;
            } else {
                self.grid[d][x] = b'o';
                return Some((x, d));
            }
        }
        None
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                write!(f, "{}", self.grid[i][j] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_int(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse().unwrap())(input)
}

fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_int, char(','), parse_int)(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(" -> "), parse_point)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(usize, usize)>>> {
    separated_list1(char('\n'), parse_path)(input)
}

pub fn solve1() -> i32 {
    let mut cave = Cave::new("src/day14/input.txt");
    let mut counter = 0;
    while let Some(_) = cave.drop_sand() {
        counter += 1;
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
        assert_eq!(solution, 715);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
