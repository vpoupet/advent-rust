use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day04/input.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut total = 0;
    for i in 0..height {
        for j in 0..width {
            if j + 3 < width {
                if
                    grid[i][j] == 'X' &&
                    grid[i][j + 1] == 'M' &&
                    grid[i][j + 2] == 'A' &&
                    grid[i][j + 3] == 'S'
                {
                    total += 1;
                }
                if
                    grid[i][j] == 'S' &&
                    grid[i][j + 1] == 'A' &&
                    grid[i][j + 2] == 'M' &&
                    grid[i][j + 3] == 'X'
                {
                    total += 1;
                }
            }
            if i + 3 < height {
                if
                    grid[i][j] == 'X' &&
                    grid[i + 1][j] == 'M' &&
                    grid[i + 2][j] == 'A' &&
                    grid[i + 3][j] == 'S'
                {
                    total += 1;
                }
                if
                    grid[i][j] == 'S' &&
                    grid[i + 1][j] == 'A' &&
                    grid[i + 2][j] == 'M' &&
                    grid[i + 3][j] == 'X'
                {
                    total += 1;
                }
            }
            if i + 3 < height && j + 3 < width {
                if
                    grid[i][j] == 'X' &&
                    grid[i + 1][j + 1] == 'M' &&
                    grid[i + 2][j + 2] == 'A' &&
                    grid[i + 3][j + 3] == 'S'
                {
                    total += 1;
                }
                if
                    grid[i][j] == 'S' &&
                    grid[i + 1][j + 1] == 'A' &&
                    grid[i + 2][j + 2] == 'M' &&
                    grid[i + 3][j + 3] == 'X'
                {
                    total += 1;
                }
            }
            if i + 3 < height && j >= 3 {
                if
                    grid[i][j] == 'X' &&
                    grid[i + 1][j - 1] == 'M' &&
                    grid[i + 2][j - 2] == 'A' &&
                    grid[i + 3][j - 3] == 'S'
                {
                    total += 1;
                }
                if
                    grid[i][j] == 'S' &&
                    grid[i + 1][j - 1] == 'A' &&
                    grid[i + 2][j - 2] == 'M' &&
                    grid[i + 3][j - 3] == 'X'
                {
                    total += 1;
                }
            }
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day04/input.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut total = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if grid[i][j] == 'A' {
                if
                    grid[i - 1][j - 1] == 'M' &&
                    grid[i - 1][j + 1] == 'M' &&
                    grid[i + 1][j + 1] == 'S' &&
                    grid[i + 1][j - 1] == 'S'
                {
                    total += 1;
                }
                if
                    grid[i - 1][j - 1] == 'S' &&
                    grid[i - 1][j + 1] == 'M' &&
                    grid[i + 1][j + 1] == 'M' &&
                    grid[i + 1][j - 1] == 'S'
                {
                    total += 1;
                }
                if
                    grid[i - 1][j - 1] == 'S' &&
                    grid[i - 1][j + 1] == 'S' &&
                    grid[i + 1][j + 1] == 'M' &&
                    grid[i + 1][j - 1] == 'M'
                {
                    total += 1;
                }
                if
                    grid[i - 1][j - 1] == 'M' &&
                    grid[i - 1][j + 1] == 'S' &&
                    grid[i + 1][j + 1] == 'S' &&
                    grid[i + 1][j - 1] == 'M'
                {
                    total += 1;
                }
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
        assert_eq!(solution, 2613);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 1905);
    }
}
