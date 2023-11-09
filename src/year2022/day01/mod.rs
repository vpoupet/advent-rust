use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2022/day01/input.txt").unwrap();
    let mut best_total = 0;
    let mut current_total = 0;
    for line in input.lines() {
        if line == "" {
            if current_total > best_total {
                best_total = current_total;
            }
            current_total = 0;
        } else {
            current_total += line.parse::<i32>().unwrap();
        }
    }
    best_total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2022/day01/input.txt").unwrap();
    let mut totals = Vec::new();
    let mut current_total = 0;
    for line in input.lines() {
        if line == "" {
            totals.push(current_total);
            current_total = 0;
        } else {
            current_total += line.parse::<i32>().unwrap();
        }
    }
    totals.sort();
    totals.reverse();

    totals[0] + totals[1] + totals[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 66719);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 198551);
    }
}
