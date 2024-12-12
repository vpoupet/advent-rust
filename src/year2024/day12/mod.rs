use std::collections::HashSet;

use crate::utils;

fn get(grid: &Vec<Vec<char>>, i: i32, j: i32) -> Option<char> {
    if i < 0 || i >= (grid.len() as i32) || j < 0 || j >= (grid[0].len() as i32) {
        None
    } else {
        Some(grid[i as usize][j as usize])
    }
}

fn get_price_1(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut to_do = Vec::new();
    let region = grid[i][j];
    let mut visited = HashSet::new();
    to_do.push((i, j));
    visited.insert((i, j));

    let mut perimeter: i32 = 0;
    while to_do.len() > 0 {
        let (i, j) = to_do.pop().unwrap();
        // check (i-1, j)
        if get(&grid, (i as i32) - 1, j as i32) != Some(region) {
            perimeter += 1;
        } else {
            if !visited.contains(&(i - 1, j)) {
                to_do.push((i - 1, j));
                visited.insert((i - 1, j));
            }
        }
        // check (i+1, j)
        if get(&grid, (i as i32) + 1, j as i32) != Some(region) {
            perimeter += 1;
        } else {
            if !visited.contains(&(i + 1, j)) {
                to_do.push((i + 1, j));
                visited.insert((i + 1, j));
            }
        }
        // check (i, j-1)
        if get(&grid, i as i32, (j as i32) - 1) != Some(region) {
            perimeter += 1;
        } else {
            if !visited.contains(&(i, j - 1)) {
                to_do.push((i, j - 1));
                visited.insert((i, j - 1));
            }
        }
        // check (i, j+1)
        if get(&grid, i as i32, (j as i32) + 1) != Some(region) {
            perimeter += 1;
        } else {
            if !visited.contains(&(i, j + 1)) {
                to_do.push((i, j + 1));
                visited.insert((i, j + 1));
            }
        }
    }

    for (i, j) in &visited {
        grid[*i][*j] = '.';
    }

    perimeter * (visited.len() as i32)
}

fn get_price_2(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut to_do = Vec::new();
    let region = grid[i][j];
    let mut visited = HashSet::new();
    to_do.push((i, j));
    visited.insert((i, j));
    let mut nb_sides = 0;

    while to_do.len() > 0 {
        let (i, j) = to_do.pop().unwrap();
        // check (i-1, j)
        if get(&grid, (i as i32) - 1, j as i32) == Some(region) {
            if !visited.contains(&(i - 1, j)) {
                to_do.push((i - 1, j));
                visited.insert((i - 1, j));
            }
        } else {
            if
                get(&grid, i as i32, (j as i32) + 1) != Some(region) ||
                (get(&grid, i as i32, (j as i32) + 1) == Some(region) &&
                    get(&grid, (i as i32) - 1, (j as i32) + 1) == Some(region))
            {
                nb_sides += 1;
            }
        }
        // check (i+1, j)
        if get(&grid, (i as i32) + 1, j as i32) == Some(region) {
            if !visited.contains(&(i + 1, j)) {
                to_do.push((i + 1, j));
                visited.insert((i + 1, j));
            }
        } else {
            if
                get(&grid, i as i32, (j as i32) + 1) != Some(region) ||
                (get(&grid, i as i32, (j as i32) + 1) == Some(region) &&
                    get(&grid, (i as i32) + 1, (j as i32) + 1) == Some(region))
            {
                nb_sides += 1;
            }
        }
        // check (i, j-1)
        if get(&grid, i as i32, (j as i32) - 1) == Some(region) {
            if !visited.contains(&(i, j - 1)) {
                to_do.push((i, j - 1));
                visited.insert((i, j - 1));
            }
        } else {
            if
                get(&grid, (i as i32) + 1, j as i32) != Some(region) ||
                (get(&grid, (i as i32) + 1, j as i32) == Some(region) &&
                    get(&grid, (i as i32) + 1, (j as i32) - 1) == Some(region))
            {
                nb_sides += 1;
            }
        }
        // check (i, j+1)
        if get(&grid, i as i32, (j as i32) + 1) == Some(region) {
            if !visited.contains(&(i, j + 1)) {
                to_do.push((i, j + 1));
                visited.insert((i, j + 1));
            }
        } else {
            if
                get(&grid, (i as i32) + 1, j as i32) != Some(region) ||
                (get(&grid, (i as i32) + 1, j as i32) == Some(region) &&
                    get(&grid, (i as i32) + 1, (j as i32) + 1) == Some(region))
            {
                nb_sides += 1;
            }
        }
    }

    for (i, j) in &visited {
        grid[*i][*j] = '.';
    }

    nb_sides * (visited.len() as i32)
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day12/input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                total += get_price_1(&mut grid, i, j);
            }
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day12/input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                total += get_price_2(&mut grid, i, j);
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
        assert_eq!(solution, 1451030);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 859494);
    }
}
