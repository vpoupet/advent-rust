use std::collections::HashSet;

use crate::utils;

fn make_grid(filename: &str) -> (Vec<Vec<bool>>, usize, usize) {
    let input = utils::read_input(filename).unwrap();
    let mut i0 = 0;
    let mut j0 = 0;
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut grid_line = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                i0 = i;
                j0 = j;
            }
            grid_line.push(c == '#');
        }
        grid.push(grid_line);
    }
    (grid, i0, j0)
}

pub fn solve1() -> i32 {
    let (grid, i0, j0) = make_grid("src/year2023/day21/input.txt");
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert((i0, j0));
    for _ in 0..64 {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for (i, j) in positions.iter().cloned() {
            if !grid[i+1][j] {
                new_positions.insert((i + 1, j));
            }
            if !grid[i-1][j] {
                new_positions.insert((i - 1, j));
            }
            if !grid[i][j+1] {
                new_positions.insert((i, j+1));
            }
            if !grid[i][j-1] {
                new_positions.insert((i, j-1));
            }
        }
        positions = new_positions;
    }
    positions.len() as i32
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/year2023/day21/input.txt").unwrap();
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
