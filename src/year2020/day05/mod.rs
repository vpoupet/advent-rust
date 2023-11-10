use crate::utils;

pub fn get_codes() -> Vec<i32> {
    let input = utils::read_input("src/year2020/day05/input.txt").unwrap();
    let mut codes = Vec::new();

    for line in input.lines() {
        let mut min = 0;
        let mut max = 128;
        for c in line.chars().take(7) {
            let mid = (min + max) / 2;
            match c {
                'F' => max = mid,
                'B' => min = mid,
                _ => panic!("Invalid character"),
            }
        }
        let row = min;
        min = 0;
        max = 8;
        for c in line.chars().skip(7).take(3) {
            let mid = (min + max) / 2;
            match c {
                'L' => max = mid,
                'R' => min = mid,
                _ => panic!("Invalid character"),
            }
        }
        let col = min;
        codes.push(row * 8 + col);
    }
    codes.sort();
    return codes;
}

pub fn solve1() -> i32 {
    let codes = get_codes();
    *codes.last().unwrap()
}

pub fn solve2() -> i32 {
    let codes = get_codes();
    for i in 1..codes.len() {
        if codes[i] - codes[i - 1] == 2 {
            return codes[i] - 1;
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
        assert_eq!(solution, 883);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 532);
    }
}
