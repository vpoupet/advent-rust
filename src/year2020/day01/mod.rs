use crate::utils;

fn find_sum(values: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let mut i = 0;
    let mut j = values.len() - 1;

    while i < j {
        let sum = values[i] + values[j];
        if sum == target {
            return Some((values[i], values[j]));
        } else if sum < target {
            i += 1;
        } else {
            j -= 1;
        }
    }
    None
}

fn parse_input(filename: &str) -> Vec<i32> {
    let mut values = Vec::new();
    let input = utils::read_input(filename).unwrap();
    for line in input.lines() {
        values.push(line.parse::<i32>().unwrap());
    }
    values
}
pub fn solve1() -> i32 {
    let mut values = parse_input("src/year2020/day01/input.txt");
    values.sort();

    if let Some((x, y)) = find_sum(&values, 2020) {
        return x * y;
    }
    0
}

pub fn solve2() -> i32 {
    let mut values = parse_input("src/year2020/day01/input.txt");
    values.sort();

    for v in &values {
        if let Some((a, b)) = find_sum(&values, 2020 - v) {
            return a * b * v;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 444019);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 29212176);
    }
}
