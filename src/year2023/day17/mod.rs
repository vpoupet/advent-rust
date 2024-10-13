use crate::utils;

// Directions :
// 0: Up (0, -1)
// 1: Left (-1, 0)
// 2: Down (0, 1)
// 3: Right (1, 0)

fn update_score_1(
    score_grid: &mut Vec<Vec<Vec<Vec<i32>>>>,
    heat_loss_grid: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    to_do: &mut Vec<(usize, usize)>
) {
    let height = heat_loss_grid.len();
    let width = heat_loss_grid[0].len();

    let mut dx = 0;
    let mut dy = -1; // try Up neighbor first
    for direction_index in 0..4 {
        let mut did_update_neighbor = false;
        let nx = (x as i32) + dx; // neighbor coordinates
        let ny = (y as i32) + dy;
        if nx >= 0 && nx < (width as i32) && ny >= 0 && ny < (height as i32) {
            let nx = nx as usize;
            let ny = ny as usize;
            let heat_loss = heat_loss_grid[ny][nx];

            // try continuing in the same direction
            for i in 0..2 {
                let score = score_grid[y][x][direction_index][i];
                if score + heat_loss < score_grid[ny][nx][direction_index][i + 1] {
                    score_grid[ny][nx][direction_index][i + 1] = score + heat_loss;
                    did_update_neighbor = true;
                }
            }

            // try turning right
            let di = (direction_index + 3) % 4;
            let score = score_grid[y][x][di].iter().min().unwrap();
            if score + heat_loss < score_grid[ny][nx][direction_index][0] {
                score_grid[ny][nx][direction_index][0] = score + heat_loss;
                did_update_neighbor = true;
            }
            // try turning left
            let di = (direction_index + 1) % 4;
            let score = score_grid[y][x][di].iter().min().unwrap();
            if score + heat_loss < score_grid[ny][nx][direction_index][0] {
                score_grid[ny][nx][direction_index][0] = score + heat_loss;
                did_update_neighbor = true;
            }

            if did_update_neighbor {
                to_do.push((nx, ny));
            }
        }
        (dx, dy) = (dy, -dx); // try next neighbor
    }
}

fn update_score_2(
    score_grid: &mut Vec<Vec<Vec<Vec<i32>>>>,
    heat_loss_grid: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    to_do: &mut Vec<(usize, usize)>
) {
    let height = heat_loss_grid.len();
    let width = heat_loss_grid[0].len();

    let mut dx = 0;
    let mut dy = -1; // try Up neighbor first
    for direction_index in 0..4 {
        let mut did_update_neighbor = false;
        let nx = (x as i32) + dx; // neighbor coordinates
        let ny = (y as i32) + dy;
        if nx >= 0 && nx < (width as i32) && ny >= 0 && ny < (height as i32) {
            let nx = nx as usize;
            let ny = ny as usize;
            let heat_loss = heat_loss_grid[ny][nx];

            // try continuing in the same direction
            for i in 0..9 {
                let score = score_grid[y][x][direction_index][i];
                if score + heat_loss < score_grid[ny][nx][direction_index][i + 1] {
                    score_grid[ny][nx][direction_index][i + 1] = score + heat_loss;
                    did_update_neighbor = true;
                }
            }

            // try turning right
            let di = (direction_index + 3) % 4;
            let score = score_grid[y][x][di][3..].iter().min().unwrap();
            if score + heat_loss < score_grid[ny][nx][direction_index][0] {
                score_grid[ny][nx][direction_index][0] = score + heat_loss;
                did_update_neighbor = true;
            }
            // try turning left
            let di = (direction_index + 1) % 4;
            let score = score_grid[y][x][di][3..].iter().min().unwrap();
            if score + heat_loss < score_grid[ny][nx][direction_index][0] {
                score_grid[ny][nx][direction_index][0] = score + heat_loss;
                did_update_neighbor = true;
            }

            if did_update_neighbor {
                to_do.push((nx, ny));
            }
        }
        (dx, dy) = (dy, -dx); // try next neighbor
    }
}

fn make_heat_loss_grid(filename: &str) -> Vec<Vec<i32>> {
    let input = utils::read_input(filename).unwrap();
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }
    grid
}

pub fn solve1() -> i32 {
    let heat_loss_grid = make_heat_loss_grid("src/year2023/day17/input.txt");
    let height = heat_loss_grid.len();
    let width = heat_loss_grid[0].len();
    let mut score_grid = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(vec![vec![i32::MAX - 10; 3]; 4]);
        }
        score_grid.push(row);
    }
    score_grid[0][0][2][0] = 0;
    score_grid[0][0][3][0] = 0;

    let mut to_do: Vec<(usize, usize)> = Vec::new();
    to_do.push((0, 0));

    while !to_do.is_empty() {
        let (x, y) = to_do.pop().unwrap();
        update_score_1(&mut score_grid, &heat_loss_grid, x, y, &mut to_do);
    }

    let mut min_cost = i32::MAX;
    for i in 0..4 {
        for j in 0..3 {
            if score_grid[height - 1][width - 1][i][j] < min_cost {
                min_cost = score_grid[height - 1][width - 1][i][j];
            }
        }
    }
    min_cost
}

pub fn solve2() -> i32 {
    let heat_loss_grid = make_heat_loss_grid("src/year2023/day17/input.txt");
    let height = heat_loss_grid.len();
    let width = heat_loss_grid[0].len();
    let mut score_grid = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(vec![vec![i32::MAX - 10; 10]; 4]);
        }
        score_grid.push(row);
    }
    score_grid[0][0][2][0] = 0;
    score_grid[0][0][3][0] = 0;

    let mut to_do: Vec<(usize, usize)> = Vec::new();
    to_do.push((0, 0));

    while !to_do.is_empty() {
        let (x, y) = to_do.pop().unwrap();
        update_score_2(&mut score_grid, &heat_loss_grid, x, y, &mut to_do);
    }

    let mut min_cost = i32::MAX;
    for i in 0..4 {
        for j in 3..10 {
            if score_grid[height - 1][width - 1][i][j] < min_cost {
                min_cost = score_grid[height - 1][width - 1][i][j];
            }
        }
    }
    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "long test (5s optimized)"]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 797);
    }

    #[test]
    // #[ignore = "long test (22s optimized)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 914);
    }
}
