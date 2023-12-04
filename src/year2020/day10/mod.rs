use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/day10/input.txt").unwrap();
    let mut values = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    values.sort();
    values.insert(0, 0);
    values.push(values[values.len() - 1] + 3);

    let mut counter = vec![0, 0, 0, 0];
    for i in 0..values.len() - 1 {
        counter[(values[i + 1] - values[i]) as usize] += 1;
    }
    counter[1] * counter[3]
}

fn count_paths(values: &Vec<i32>, index: usize, memo: &mut Vec<Option<i64>>) -> i64 {
    if index == values.len() - 1 {
        return 1;
    }

    if let Some(n) = memo[index] {
        return n;
    }

    let mut count = 0;
    for i in index + 1..=(index + 3).min(values.len() - 1) {
        if values[i] - values[index] <= 3 {
            count += count_paths(values, i, memo);
        }
    }

    memo[index] = Some(count);
    count
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2020/day10/input.txt").unwrap();
    let mut values = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    values.sort();
    values.insert(0, 0);
    values.push(values[values.len() - 1] + 3);

    let mut memo = vec![None; values.len()];
    count_paths(&values, 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 1856);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 2314037239808);
    }
}
