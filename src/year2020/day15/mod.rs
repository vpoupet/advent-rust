use std::collections::HashMap;

fn get_nth_number(starting_numbers: &Vec<usize>, n: usize) -> usize {
    let mut known_numbers = HashMap::new();
    for i in 0..starting_numbers.len() - 1 {
        known_numbers.insert(starting_numbers[i], i);
    }
    let mut last_number = *starting_numbers.last().unwrap();
    for i in starting_numbers.len() - 1..n-1 {
        let next_number = match known_numbers.get(&last_number) {
            Some(&last_index) => i - last_index,
            None => 0,
        };
        known_numbers.insert(last_number, i);
        last_number = next_number;
    }
    last_number
}

pub fn solve1() -> usize {
    get_nth_number(&vec![0,13,1,8,6,15], 2020)
}

pub fn solve2() -> usize {
    get_nth_number(&vec![0,13,1,8,6,15], 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(get_nth_number(&vec![1,3,2], 2020), 1);
        assert_eq!(get_nth_number(&vec![2,1,3], 2020), 10);
        assert_eq!(get_nth_number(&vec![1,2,3], 2020), 27);
        assert_eq!(get_nth_number(&vec![2,3,1], 2020), 78);
        assert_eq!(get_nth_number(&vec![3,2,1], 2020), 438);
        assert_eq!(get_nth_number(&vec![3,1,2], 2020), 1836);
    }

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 1618);
    }

    #[test]
    #[ignore = "long test (14s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 548531);
    }
}
