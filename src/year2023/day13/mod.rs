use crate::utils;

fn parse_input() -> Vec<Vec<Vec<bool>>> {
    let input = utils::read_input("src/year2023/day13/input.txt").unwrap();
    let mut result = Vec::new();
    let mut current_pattern = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            result.push(current_pattern);
            current_pattern = Vec::new();
            continue;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        current_pattern.push(row);
    }
    result.push(current_pattern);
    result
}

fn has_horizontal_mirror(pattern: &Vec<Vec<bool>>, row: usize) -> bool {
    let mut result = true;
    let mut i = 0;
    while i + 1 <= row && row + i < pattern.len() && result {
        result &= pattern[row - 1 - i] == pattern[row + i];
        i += 1;
    }
    result
}

fn has_vertical_mirror(pattern: &Vec<Vec<bool>>, col: usize) -> bool {
    let mut result = true;
    let mut j = 0;
    while j + 1 <= col && col + j < pattern[0].len() && result {
        for i in 0..pattern.len() {
            result &= pattern[i][col - j - 1] == pattern[i][col + j];
        }
        j += 1;
    }
    result
}

pub fn solve1() -> usize {
    let patterns = parse_input();
    let mut total = 0;
    for pattern in patterns {
        for col in 1..pattern[0].len() {
            if has_vertical_mirror(&pattern, col) {
                total += col;
            }
        }
        for row in 1..pattern.len() {
            if has_horizontal_mirror(&pattern, row) {
                total += 100 * row;
            }
        }
    }
    total
}

fn has_horizontal_mirror_with_smudge(pattern: &Vec<Vec<bool>>, row: usize) -> bool {
    let mut did_find_smudge = false;
    let mut i = 0;
    while i + 1 <= row && row + i < pattern.len() {
        for j in 0..pattern[0].len() {
            if pattern[row - 1 - i][j] != pattern[row + i][j] {
                if !did_find_smudge {
                    did_find_smudge = true;
                } else {
                    return false;
                }
            }
        }
        i += 1;
    }
    did_find_smudge
}

fn has_vertical_mirror_with_smudge(pattern: &Vec<Vec<bool>>, col: usize) -> bool {
    let mut did_find_smudge = false;
    let mut j = 0;
    while j + 1 <= col && col + j < pattern[0].len() {
        for i in 0..pattern.len() {
            if pattern[i][col - j - 1] != pattern[i][col + j] {
                if !did_find_smudge {
                    did_find_smudge = true;
                } else {
                    return false;
                }
            }
        }
        j += 1;
    }
    did_find_smudge
}

pub fn solve2() -> usize {
    let patterns = parse_input();
    let mut total = 0;
    'pattern:
    for pattern in patterns {
        for col in 1..pattern[0].len() {
            if has_vertical_mirror_with_smudge(&pattern, col) {
                total += col;
                continue 'pattern;
            }
        }
        for row in 1..pattern.len() {
            if has_horizontal_mirror_with_smudge(&pattern, row) {
                total += 100 * row;
                continue 'pattern;
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
        assert_eq!(solution, 35691);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 39037);
    }
}
