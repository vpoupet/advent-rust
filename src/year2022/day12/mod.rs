use crate::utils;
use std::collections::VecDeque;
use std::cmp;

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn new(filename: &str) -> Map {
        let input = utils::read_input(filename).unwrap();
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c as u8);
            }
            grid.push(row);
        }
        
        let height = grid.len();
        let width = grid[0].len();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for i in 0..height {
            for j in 0..width {
                if grid[i][j] == b'S' {
                    start = (i, j);
                    grid[i][j] = b'a';
                }
                if grid[i][j] == b'E' {
                    end = (i, j);
                    grid[i][j] = b'z';
                }
            }
        }

        Map { 
            grid,
            height,
            width,
            start, 
            end,
        }
    }

    fn make_distances_to_end(&self) -> Vec<Vec<i32>> {
        let mut distances = vec![vec![i32::MAX; self.width]; self.height];
        distances[self.end.0][self.end.1] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(self.end);

        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                // check if it's possible to go from (i+di, j+dj) to (i, j)
                if (i as i32 + di < 0) || (i as i32 + di >= self.height as i32) || (j as i32 + dj < 0) || (j as i32 + dj >= self.width as i32) {
                    continue;
                }
                let i2 = (i as i32 + di) as usize;
                let j2 = (j as i32 + dj) as usize;

                if self.grid[i][j] <= self.grid[i2][j2] + 1 && distances[i2][j2] > distances[i][j] + 1 {
                    distances[i2][j2] = distances[i][j] + 1;
                    queue.push_back((i2, j2));
                }
            }
        }
        return distances;
    }
}

pub fn solve1() -> i32 {
    let map = Map::new("src/year2022/day12/input.txt");
    let distances = map.make_distances_to_end();
    return distances[map.start.0][map.start.1];
}

pub fn solve2() -> i32 {
    let map = Map::new("src/year2022/day12/input.txt");
    let distances = map.make_distances_to_end();
    let mut min_distance = i32::MAX;
    
    for i in 0..map.height {
        for j in 0..map.width {
            if map.grid[i][j] == b'a' {
                min_distance = cmp::min(min_distance, distances[i][j]);
            }
        }
    }
    min_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 339);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 332);
    }
}
