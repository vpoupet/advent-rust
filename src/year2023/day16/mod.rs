use std::collections::HashSet;

use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ray {
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
}
impl Ray {
    fn new(i: i32, j: i32, di: i32, dj: i32) -> Self {
        Self { i, j, di, dj }
    }
}

fn make_grid(filename: &str) -> Vec<Vec<char>> {
    let input = utils::read_input(filename).unwrap();
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    grid
}

fn ray_next(grid: &Vec<Vec<char>>, r: &Ray) -> Vec<Ray> {
    let mut result = Vec::new();

    match grid[r.i as usize][r.j as usize] {
        '.' => result.push(Ray::new(r.i + r.di, r.j + r.dj, r.di, r.dj)),
        '|' => {
            if r.dj != 0 {
                result.push(Ray::new(r.i - 1, r.j, -1, 0));
                result.push(Ray::new(r.i + 1, r.j, 1, 0));
            } else {
                result.push(Ray::new(r.i + r.di, r.j + r.dj, r.di, r.dj));
            }
        }
        '-' => {
            if r.di != 0 {
                result.push(Ray::new(r.i, r.j - 1, 0, -1));
                result.push(Ray::new(r.i, r.j + 1, 0, 1));
            } else {
                result.push(Ray::new(r.i + r.di, r.j + r.dj, r.di, r.dj));
            }
        }
        '/' => {
            if r.dj != 0 {
                result.push(Ray::new(r.i - r.dj, r.j, -r.dj, 0));
            } else {
                result.push(Ray::new(r.i, r.j - r.di, 0, -r.di));
            }
        }
        '\\' => {
            if r.dj != 0 {
                result.push(Ray::new(r.i + r.dj, r.j, r.dj, 0));
            } else {
                result.push(Ray::new(r.i, r.j + r.di, 0, r.di));
            }
        }
        _ => panic!("Invalid symbol"),
    }

    return result
        .into_iter()
        .filter(|r| 0 <= r.i && r.i < grid.len() as i32 && 0 <= r.j && r.j < grid.len() as i32)
        .collect::<Vec<Ray>>();
}

fn count_energized_cells(grid: &Vec<Vec<char>>, initial_ray: Ray) -> usize {
    let mut rays_grid = vec![vec![HashSet::new(); grid.len()]; grid.len()];
    let mut to_do = Vec::new();

    rays_grid[initial_ray.i as usize][initial_ray.j as usize].insert(initial_ray);
    to_do.push(initial_ray);

    while !to_do.is_empty() {
        let r = to_do.pop().unwrap();
        let new_rays = ray_next(&grid, &r);
        for new_ray in new_rays {
            if !rays_grid[new_ray.i as usize][new_ray.j as usize].contains(&new_ray) {
                rays_grid[new_ray.i as usize][new_ray.j as usize].insert(new_ray);
                to_do.push(new_ray);
            }
        }
    }

    return rays_grid.iter().flatten().filter(|s| !s.is_empty()).count()
}

pub fn solve1() -> usize {
    let grid = make_grid("src/year2023/day16/input.txt");
    count_energized_cells(&grid, Ray::new(0, 0, 0, 1))
}

pub fn solve2() -> usize {
    let grid = make_grid("src/year2023/day16/input.txt");
    let mut best = 0;
    let n = grid.len() as i32;
    for k in 0..n {
        best = best.max(count_energized_cells(&grid, Ray::new(k, 0, 0, 1)));
        best = best.max(count_energized_cells(&grid, Ray::new(k, n - 1, 0, -1)));
        best = best.max(count_energized_cells(&grid, Ray::new(0, k, 1, 0)));
        best = best.max(count_energized_cells(&grid, Ray::new(n - 1, k, -1, 0)));
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 7979);
    }

    #[test]
    #[ignore = "long test (3s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 8437);
    }
}
