use std::{fmt, collections::HashSet};

use crate::utils;

static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

struct Board {
    width: usize,
    height: usize,
    left_moving_grid: Vec<Vec<bool>>,
    right_moving_grid: Vec<Vec<bool>>,
    up_moving_grid: Vec<Vec<bool>>,
    down_moving_grid: Vec<Vec<bool>>,
    time: i32,
}

impl Board {
    fn new(filename: &str) -> Self {
        let input = utils::read_input(filename).unwrap();
        let height = input.lines().count() - 2;
        let width = input.lines().next().unwrap().chars().count() - 2;
        let mut left_moving_grid = vec![vec![false; width]; height];
        let mut right_moving_grid = vec![vec![false; width]; height];
        let mut up_moving_grid = vec![vec![false; width]; height];
        let mut down_moving_grid = vec![vec![false; width]; height];

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '<' => left_moving_grid[i - 1][j - 1] = true,
                    '>' => right_moving_grid[i - 1][j - 1] = true,
                    '^' => up_moving_grid[i - 1][j - 1] = true,
                    'v' => down_moving_grid[i - 1][j - 1] = true,
                    _ => (),
                }
            }
        }

        Self {
            width,
            height,
            left_moving_grid,
            right_moving_grid,
            up_moving_grid,
            down_moving_grid,
            time: 0,
        }
    }

    fn has_obstacle(&self, i: usize, j: usize) -> bool {
        {
            let j = (j as i32 + self.time).rem_euclid(self.width as i32);
            if self.left_moving_grid[i][j as usize] {
                return true;
            }
        }
        {
            let j = (j as i32 - self.time).rem_euclid(self.width as i32);
            if self.right_moving_grid[i][j as usize] {
                return true;
            }
        }
        {
            let i = (i as i32 + self.time).rem_euclid(self.height as i32);
            if self.up_moving_grid[i as usize][j] {
                return true;
            }
        }
        {
            let i = (i as i32 - self.time).rem_euclid(self.height as i32);
            if self.down_moving_grid[i as usize][j] {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.has_obstacle(i, j) {
                    write!(f, "*")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve1() -> i32 {
    let mut board = Board::new("src/day24/input.txt");
    while board.has_obstacle(0, 0) {
        board.time += 1;
    }
    let mut positions = HashSet::new();
    positions.insert((0, 0));

    loop {
        board.time += 1;
        let mut new_positions = HashSet::new();
        for (i, j) in &positions {
            if !board.has_obstacle(*i as usize, *j as usize) {
                new_positions.insert((*i, *j));
            }
            for (di, dj) in &DIRECTIONS {
                if (0..board.height as i32).contains(&(i + di)) && (0..board.width as i32).contains(&(j + dj)) && !board.has_obstacle((i + di) as usize, (j + dj) as usize) {
                    new_positions.insert((i+di, j+dj));
                }
            }
        }
        positions = new_positions;
        if positions.contains(&(board.height as i32 - 1, board.width as i32 - 1)) {
            return board.time + 1;
        }
    }
}

pub fn solve2() -> i32 {
    let _input = utils::read_input("src/day24/input.txt").unwrap();
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
