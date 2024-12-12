use std::collections::{HashMap, HashSet};

use crate::utils;

fn make_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line|
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>()
}

fn get_neighbors(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if i > 0 {
        neighbors.push((i-1, j));
    }
    if i < grid.len() - 1 {
        neighbors.push((i+1, j));
    }
    if j > 0 {
        neighbors.push((i, j-1));
    }
    if j < grid[0].len() - 1 {
        neighbors.push((i, j+1));
    }
    neighbors
}

fn get_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> i32 {
    let mut positions = HashSet::new();
    positions.insert((i, j));
    for v in 1..=9 {
        let mut new_positions = HashSet::new();
        for (i, j) in &positions {
            for (ni, nj) in get_neighbors(grid, *i, *j) {
                if grid[ni][nj] == v {
                    new_positions.insert((ni, nj));
                }
            }
        }
        positions = new_positions;
    }
    positions.len() as i32
}

fn get_rating(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> i32 {
    let mut positions = HashMap::new();
    positions.insert((i, j), 1);
    for v in 1..=9 {
        let mut new_positions = HashMap::new();
        for ((i, j), count) in &positions {
            for (ni, nj) in get_neighbors(grid, *i, *j) {
                if grid[ni][nj] == v {
                    if !new_positions.contains_key(&(ni, nj)) {
                        new_positions.insert((ni, nj), 0);
                    }
                    new_positions.insert((ni, nj), new_positions[&(ni, nj)] + count);
                }
            }
        }
        positions = new_positions;
    }
    positions.values().sum()
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day10/input.txt").unwrap();
    let grid = make_grid(&input);
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                total += get_score(&grid, i, j);
            }
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day10/input.txt").unwrap();
    let grid = make_grid(&input);
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                total += get_rating(&grid, i, j);
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 688);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 1459);
    }
}
