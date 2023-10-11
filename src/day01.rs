use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/day01.txt").unwrap();
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
    let input = utils::read_input("src/day01.txt").unwrap();
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
        println!("Part One: {}", solve1());
        assert_eq!(solve1(), 66719);
    }

    #[test]
    fn test_solve2() {
        println!("Part Two: {}", solve2());
        assert_eq!(solve2(), 198551);
    }
}
