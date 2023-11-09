use crate::utils;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::fmt;

struct Cave {
    grid: Vec<Vec<u8>>,
    offset: usize,
}

impl Cave {
    fn new(filename: &str) -> Self {
        let input = utils::read_input(filename).unwrap();
        let (_, paths) = parse_input(&input).unwrap();
        let mut max_depth = 0;

        for path in &paths {
            for (_, d) in path {
                max_depth = max_depth.max(*d);
            }
        }

        let offset = 500 - max_depth - 5;
        let max_x = 500 + max_depth + 5;

        let width = max_x - offset;
        let mut grid = vec![vec![b'.'; width]; max_depth + 3];

        for i in 0..width {
            grid[max_depth + 2][i] = b'#';
        }

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

        Self { grid, offset }
    }

    fn drop_sand(&mut self) -> (usize, usize) {
        let mut x = 500 - self.offset;
        let mut d = 0;

        while d + 1 < self.grid.len() {
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
                return (d, x + self.offset);
            }
        }
        panic!("Sand dropped beyond bottom");
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
    let mut cave = Cave::new("src/year2022/day14/input.txt");
    let mut counter = 0;
    loop {
        let (d, _) = cave.drop_sand();
        if d == cave.grid.len() - 2 {
            break;
        }
        counter += 1;
    }
    counter
}

pub fn solve2() -> i32 {
    let mut cave = Cave::new("src/year2022/day14/input.txt");
    let mut counter = 0;
    loop {
        let (d, x) = cave.drop_sand();
        counter += 1;
        if (d, x) == (0, 500) {
            break;
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
        assert_eq!(solution, 715);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 25248);
    }
}
