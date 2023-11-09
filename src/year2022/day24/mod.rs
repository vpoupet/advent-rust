use std::collections::HashSet;

use crate::utils;

static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

struct Board {
    width: i32,
    height: i32,
    left_moving_grid: Vec<Vec<bool>>,
    right_moving_grid: Vec<Vec<bool>>,
    up_moving_grid: Vec<Vec<bool>>,
    down_moving_grid: Vec<Vec<bool>>,
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
            width: width as i32,
            height: height as i32,
            left_moving_grid,
            right_moving_grid,
            up_moving_grid,
            down_moving_grid,
        }
    }

    fn is_safe(&self, i: i32, j: i32, time: i32) -> bool {
        if (i == -1 && j == 0) || (i == self.height && j == self.width - 1) {
            // special case for entrance and exit
            return true;
        }
        if i < 0 || i >= self.height || j < 0 || j >= self.width {
            // out of bounds
            return false;
        }

        if self.left_moving_grid[i as usize][(j + time).rem_euclid(self.width) as usize]
            || self.right_moving_grid[i as usize][(j - time).rem_euclid(self.width) as usize]
            || self.up_moving_grid[(i + time).rem_euclid(self.height) as usize][j as usize]
            || self.down_moving_grid[(i - time).rem_euclid(self.height) as usize][j as usize]
        {
            // there is a moving obstacle on the cell
            return false;
        }
        true
    }

    fn get_next_positions(&self, positions: HashSet<(i32, i32)>, time: i32) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new();
        for (i, j) in positions {
            if self.is_safe(i, j, time) {
                result.insert((i, j));
            }
            for (di, dj) in &DIRECTIONS {
                if self.is_safe(i + di, j + dj, time) {
                    result.insert((i + di, j + dj));
                }
            }
        }
        result
    }
}

pub fn solve1() -> i32 {
    let board = Board::new("src/year2022/day24/input.txt");
    let mut positions = HashSet::new();
    let mut time = 0;
    positions.insert((-1, 0)); // start position

    let exit_position = (board.height as i32, board.width as i32 - 1);
    while !positions.contains(&exit_position) {
        time += 1;
        positions = board.get_next_positions(positions, time);
    }

    time
}

pub fn solve2() -> i32 {
    let board = Board::new("src/year2022/day24/input.txt");
    let start_position = (-1, 0);
    let exit_position = (board.height as i32, board.width as i32 - 1);
    let mut time = 0;
    
    let mut positions = HashSet::new();
    positions.insert(start_position); // start position
    while !positions.contains(&exit_position) {
        time += 1;
        positions = board.get_next_positions(positions, time);
    }
    
    positions = HashSet::new();
    positions.insert(exit_position);
    while !positions.contains(&start_position) {
        time += 1;
        positions = board.get_next_positions(positions, time);
    }
    
    positions = HashSet::new();
    positions.insert(start_position);
    while !positions.contains(&exit_position) {
        time += 1;
        positions = board.get_next_positions(positions, time);
    }
    println!("Time to reach exit: {}", time);

    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 266);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
