use crate::utils;

struct Configuration {
    grid: Vec<Vec<char>>,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Configuration {
    fn step(&mut self) -> bool {
        let next_x = self.x + self.dx;
        let next_y = self.y + self.dy;
        if
            next_x < 0 ||
            next_x >= (self.grid[0].len() as i32) ||
            next_y < 0 ||
            next_y >= (self.grid.len() as i32)
        {
            // exit the grid
            self.x = next_x;
            self.y = next_y;
            return false;
        }

        if self.grid[next_y as usize][next_x as usize] == '#' {
            // turn right
            (self.dx, self.dy) = (-self.dy, self.dx);
        } else {
            // move forward
            self.x = next_x;
            self.y = next_y;
        }
        return true;
    }
}

fn make_starting_configuration(grid: Vec<Vec<char>>) -> Configuration {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return Configuration {
                    grid: grid,
                    x: j as i32,
                    y: i as i32,
                    dx: 0,
                    dy: -1,
                };
            }
        }
    }
    panic!("No starting position found");
}

fn test_obstacle(grid: &Vec<Vec<char>>, x: i32, y: i32, start_x: i32, start_y: i32) -> bool {
    let mut new_grid = grid.clone();
    new_grid[y as usize][x as usize] = '#';
    let mut c = Configuration {
        grid: new_grid,
        x: start_x,
        y: start_y,
        dx: 0,
        dy: -1,
    };

    for _ in 0..grid.len() * grid[0].len() {
        if !c.step() {
            return false;
        }
    }
    return true;
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day06/input.txt").unwrap();
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut c = make_starting_configuration(grid);
    c.grid[c.y as usize][c.x as usize] = 'X';

    let mut total = 1;
    while c.step() {
        if c.grid[c.y as usize][c.x as usize] == '.' {
            c.grid[c.y as usize][c.x as usize] = 'X';
            total += 1;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day06/input.txt").unwrap();
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut c = make_starting_configuration(grid);
    let start_x = c.x;
    let start_y = c.y;

    let mut possible_positions = Vec::new();
    while c.step() {
        if c.grid[c.y as usize][c.x as usize] == '.' {
            c.grid[c.y as usize][c.x as usize] = 'X';
            possible_positions.push((c.x, c.y));
        }
    }

    let mut total = 0;
    for (x, y) in possible_positions {
        if test_obstacle(&c.grid, x as i32, y as i32, start_x, start_y) {
            total += 1;
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
        assert_eq!(solution, 4967);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 1789);
    }
}
