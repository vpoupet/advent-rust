use std::collections::VecDeque;

use crate::utils;

fn is_sum_of_two(num: i64, previous: &VecDeque<i64>) -> bool {
    for i in 0..previous.len() {
        for j in i + 1..previous.len() {
            if previous[i] + previous[j] == num {
                return true;
            }
        }
    }
    false
}

fn make_list() -> Vec<i64> {
    let input = utils::read_input("src/year2020/day09/input.txt").unwrap();
    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse::<i64>().unwrap());
    }
    numbers
}

fn find_target(numbers: &Vec<i64>) -> i64 {
    let mut previous = VecDeque::new();
    for i in 0..25 {
        previous.push_back(numbers[i]);
    }
    let mut i = 25;
    loop {
        if !is_sum_of_two(numbers[i], &previous) {
            return numbers[i];
        }
        previous.pop_front();
        previous.push_back(numbers[i]);
        i += 1;
    }
}

pub fn solve1() -> i64 {
    let numbers = make_list();
    find_target(&numbers)
}

pub fn solve2() -> i64 {
    let numbers = make_list();
    let target = find_target(&numbers);
    let mut i = 0;
    let mut j = 0;
    let mut total = 0;

    loop {
        if total == target {
            let mut min = numbers[i];
            let mut max = numbers[i];
            for k in i..j {
                min = min.min(numbers[k]);
                max = max.max(numbers[k]);
            }
            return min + max;
        } else if total < target {
            total += &numbers[j];
            j += 1;
        } else {
            total -= &numbers[i];
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 1398413738);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
