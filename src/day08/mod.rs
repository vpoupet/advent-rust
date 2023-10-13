use std::cmp;

use crate::utils;

struct Trees {
    nb_rows: usize,
    nb_cols: usize,
    grid: Vec<Vec<i32>>,
}

impl Trees {
    fn from_file(filename: &str) -> Trees {
        let input = utils::read_input(filename).unwrap();
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as i32);
            }
            grid.push(row);
        }

        Trees {
            nb_rows: grid.len(),
            nb_cols: grid[0].len(),
            grid,
        }
    }

    fn is_visible(&self, i: usize, j: usize) -> bool {
        let val = self.grid[i][j];
        let mut visible = true;
        for k in 0..i {
            if self.grid[k][j] >= val {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
        
        let mut visible = true;
        for k in i+1..self.nb_rows {
            if self.grid[k][j] >= val {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }

        let mut visible = true;
        for k in 0..j {
            if self.grid[i][k] >= val {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }

        let mut visible = true;
        for k in j+1..self.nb_cols {
            if self.grid[i][k] >= val {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
        false
    }

    fn visibility_up(&self, i: usize, j: usize) -> i32 {
        let val = self.grid[i][j];
        let mut counter = 0;
        for k in (0..i).rev() {
            counter += 1;
            if self.grid[k][j] >= val {
                break;
            }
        }
        counter
    }
    fn visibility_down(&self, i: usize, j: usize) -> i32 {
        let val = self.grid[i][j];
        let mut counter = 0;
        for k in i+1..self.nb_rows {
            counter += 1;
            if self.grid[k][j] >= val {
                break;
            }
        }
        counter
    }
    fn visibility_left(&self, i: usize, j: usize) -> i32 {
        let val = self.grid[i][j];
        let mut counter = 0;
        for k in (0..j).rev() {
            counter += 1;
            if self.grid[i][k] >= val {
                break;
            }
        }
        counter
    }
    fn visibility_right(&self, i: usize, j: usize) -> i32 {
        let val = self.grid[i][j];
        let mut counter = 0;
        for k in j+1..self.nb_cols {
            counter += 1;
            if self.grid[i][k] >= val {
                break;
            }
        }
        counter
    }
    fn scenic_score(&self, i: usize, j: usize) -> i32 {
        self.visibility_up(i, j) * self.visibility_down(i, j) * self.visibility_left(i, j) * self.visibility_right(i, j)
    }
}

pub fn solve1() -> i32 {
    let trees = Trees::from_file("src/day08/input.txt");
    let mut counter = 0;
    for i in 0..trees.nb_rows {
        for j in 0..trees.nb_cols {
            if trees.is_visible(i, j) {
                counter += 1;
            }
        }
    }
    counter
}

pub fn solve2() -> i32 {
    let trees = Trees::from_file("src/day08/input.txt");
    let mut best = 0;
    for i in 0..trees.nb_rows {
        for j in 0..trees.nb_cols {
            best = cmp::max(best, trees.scenic_score(i, j));
        }
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
        assert_eq!(solution, 1782);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 474606);
    }
}
